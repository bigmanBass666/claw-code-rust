use std::path::{Component, Path, PathBuf};

use async_trait::async_trait;
use serde_json::json;
use tokio::fs;

use crate::{Tool, ToolContext, ToolOutput};

const DESCRIPTION: &str = include_str!("apply_patch.txt");

pub struct ApplyPatchTool;

#[async_trait]
impl Tool for ApplyPatchTool {
    fn name(&self) -> &str {
        "apply_patch"
    }

    fn description(&self) -> &str {
        DESCRIPTION
    }

    fn input_schema(&self) -> serde_json::Value {
        json!({
            "type": "object",
            "properties": {
                "patchText": {
                    "type": "string",
                    "description": "The full patch text that describes all changes to be made"
                }
            },
            "required": ["patchText"]
        })
    }

    async fn execute(
        &self,
        ctx: &ToolContext,
        input: serde_json::Value,
    ) -> anyhow::Result<ToolOutput> {
        let patch_text = input["patchText"].as_str().unwrap_or("");
        if patch_text.trim().is_empty() {
            return Ok(ToolOutput::error("patchText is required"));
        }

        let patch = parse_patch(patch_text)?;
        if patch.is_empty() {
            let normalized = patch_text
                .replace("\r\n", "\n")
                .replace('\r', "\n")
                .trim()
                .to_string();
            if normalized == "*** Begin Patch\n*** End Patch" {
                return Ok(ToolOutput::error("patch rejected: empty patch"));
            }
            return Ok(ToolOutput::error(
                "apply_patch verification failed: no hunks found",
            ));
        }

        let mut files = Vec::with_capacity(patch.len());
        let mut summary = Vec::with_capacity(patch.len());
        let mut total_diff = String::new();

        for change in &patch {
            let source_path = resolve_relative(&ctx.cwd, &change.path)?;
            let target_path = change
                .move_path
                .as_deref()
                .map(|path| resolve_relative(&ctx.cwd, path))
                .transpose()?;

            let old_content = match change.kind {
                PatchKind::Add => String::new(),
                _ => read_file(&source_path).await?,
            };
            let new_content = match change.kind {
                PatchKind::Delete => String::new(),
                _ => change.content.clone(),
            };

            let additions = new_content.lines().count();
            let deletions = old_content.lines().count();
            let relative_path =
                relative_worktree_path(target_path.as_ref().unwrap_or(&source_path), &ctx.cwd);
            let kind_name = change.kind.as_str();
            let diff = format!("--- {}\n+++ {}\n", relative_path, relative_path);

            files.push(json!({
                "filePath": source_path,
                "relativePath": relative_path,
                "type": kind_name,
                "patch": diff,
                "additions": additions,
                "deletions": deletions,
                "movePath": target_path,
            }));
            total_diff.push_str(&diff);
            total_diff.push('\n');

            summary.push(match change.kind {
                PatchKind::Add => format!("A {}", relative_worktree_path(&source_path, &ctx.cwd)),
                PatchKind::Delete => {
                    format!("D {}", relative_worktree_path(&source_path, &ctx.cwd))
                }
                PatchKind::Update | PatchKind::Move => {
                    format!(
                        "M {}",
                        relative_worktree_path(
                            target_path.as_ref().unwrap_or(&source_path),
                            &ctx.cwd
                        )
                    )
                }
            });
        }

        for change in &patch {
            apply_change(&ctx.cwd, change).await?;
        }

        Ok(ToolOutput {
            content: format!(
                "Success. Updated the following files:\n{}",
                summary.join("\n")
            ),
            is_error: false,
            metadata: Some(json!({
                "diff": total_diff,
                "files": files,
                "diagnostics": {},
            })),
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PatchKind {
    Add,
    Update,
    Delete,
    Move,
}

impl PatchKind {
    fn as_str(self) -> &'static str {
        match self {
            Self::Add => "add",
            Self::Update => "update",
            Self::Delete => "delete",
            Self::Move => "move",
        }
    }
}

#[derive(Debug, Clone)]
struct PatchChange {
    path: String,
    move_path: Option<String>,
    content: String,
    kind: PatchKind,
}

fn parse_patch(patch_text: &str) -> anyhow::Result<Vec<PatchChange>> {
    let normalized = patch_text.replace("\r\n", "\n").replace('\r', "\n");
    let mut lines = normalized.lines().peekable();

    while let Some(line) = lines.next() {
        if line == "*** Begin Patch" {
            break;
        }
    }

    let mut changes = Vec::new();
    while let Some(line) = lines.next() {
        if line == "*** End Patch" {
            break;
        }

        if let Some(path) = line.strip_prefix("*** Add File: ") {
            let contents = collect_plus_block(&mut lines);
            changes.push(PatchChange {
                path: path.to_string(),
                move_path: None,
                content: contents,
                kind: PatchKind::Add,
            });
            continue;
        }

        if let Some(path) = line.strip_prefix("*** Delete File: ") {
            changes.push(PatchChange {
                path: path.to_string(),
                move_path: None,
                content: String::new(),
                kind: PatchKind::Delete,
            });
            continue;
        }

        if let Some(path) = line.strip_prefix("*** Update File: ") {
            let mut move_path = None;
            if matches!(lines.peek(), Some(next) if next.starts_with("*** Move to: ")) {
                let next = lines.next().unwrap_or_default();
                move_path = Some(next.trim_start_matches("*** Move to: ").to_string());
            }
            let content = collect_hunk_block(&mut lines)?;
            let kind = if move_path.is_some() {
                PatchKind::Move
            } else {
                PatchKind::Update
            };
            changes.push(PatchChange {
                path: path.to_string(),
                move_path,
                content,
                kind,
            });
        }
    }

    Ok(changes)
}

fn collect_plus_block(lines: &mut std::iter::Peekable<std::str::Lines<'_>>) -> String {
    let mut content = String::new();
    while let Some(next) = lines.peek() {
        if next.starts_with("*** ") {
            break;
        }
        let line = lines.next().unwrap_or_default();
        if let Some(rest) = line.strip_prefix('+') {
            content.push_str(rest);
        } else {
            content.push_str(line);
        }
        content.push('\n');
    }
    content
}

fn collect_hunk_block(
    lines: &mut std::iter::Peekable<std::str::Lines<'_>>,
) -> anyhow::Result<String> {
    let mut content = String::new();
    let mut saw_hunk = false;

    while let Some(next) = lines.peek() {
        if next.starts_with("*** ") && !next.starts_with("*** End of File") {
            break;
        }
        let line = lines.next().unwrap_or_default();
        if line == "*** End of File" {
            break;
        }
        if line == "@@" || line.starts_with("@@ ") {
            saw_hunk = true;
            continue;
        }
        match line.chars().next() {
            Some('+') | Some(' ') => {
                content.push_str(&line[1..]);
                content.push('\n');
            }
            Some('-') => {
                saw_hunk = true;
            }
            _ => {
                content.push_str(line);
                content.push('\n');
            }
        }
    }

    if !saw_hunk && content.is_empty() {
        return Err(anyhow::anyhow!("no hunks found"));
    }

    Ok(content)
}

fn resolve_relative(base: &Path, rel: &str) -> anyhow::Result<PathBuf> {
    let candidate = Path::new(rel);
    if candidate.is_absolute() {
        return Err(anyhow::anyhow!(
            "file references can only be relative, NEVER ABSOLUTE."
        ));
    }

    let mut out = base.to_path_buf();
    for component in candidate.components() {
        match component {
            Component::CurDir => {}
            Component::Normal(part) => out.push(part),
            Component::ParentDir => out.push(".."),
            Component::Prefix(_) | Component::RootDir => {
                return Err(anyhow::anyhow!(
                    "file references can only be relative, NEVER ABSOLUTE."
                ));
            }
        }
    }
    Ok(out)
}

fn relative_worktree_path(path: &Path, base: &Path) -> String {
    path.strip_prefix(base)
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/")
}

async fn read_file(path: &Path) -> anyhow::Result<String> {
    Ok(fs::read_to_string(path).await?)
}

async fn apply_change(base: &Path, change: &PatchChange) -> anyhow::Result<()> {
    let source = resolve_relative(base, &change.path)?;
    match change.kind {
        PatchKind::Add => {
            if let Some(parent) = source.parent() {
                fs::create_dir_all(parent).await?;
            }
            fs::write(&source, &change.content).await?;
        }
        PatchKind::Update => {
            fs::write(&source, &change.content).await?;
        }
        PatchKind::Delete => {
            let _ = fs::remove_file(&source).await;
        }
        PatchKind::Move => {
            if let Some(dest) = &change.move_path {
                let dest = resolve_relative(base, dest)?;
                if let Some(parent) = dest.parent() {
                    fs::create_dir_all(parent).await?;
                }
                fs::write(&dest, &change.content).await?;
                let _ = fs::remove_file(&source).await;
            }
        }
    }
    Ok(())
}
