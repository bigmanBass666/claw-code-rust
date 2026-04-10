use async_trait::async_trait;
use serde_json::json;
use tokio::fs;

use crate::{Tool, ToolContext, ToolOutput};

const DESCRIPTION: &str = "Apply multiple exact string replacements in a file.";

pub struct MultiEditTool;

#[async_trait]
impl Tool for MultiEditTool {
    fn name(&self) -> &str {
        "multiedit"
    }

    fn description(&self) -> &str {
        DESCRIPTION
    }

    fn input_schema(&self) -> serde_json::Value {
        json!({
            "type": "object",
            "properties": {
                "filePath": {"type": "string"},
                "edits": {"type": "array"}
            },
            "required": ["filePath", "edits"]
        })
    }

    async fn execute(
        &self,
        ctx: &ToolContext,
        input: serde_json::Value,
    ) -> anyhow::Result<ToolOutput> {
        let file = input["filePath"].as_str().unwrap_or("");
        let path = if std::path::Path::new(file).is_absolute() {
            file.into()
        } else {
            ctx.cwd.join(file).to_string_lossy().to_string()
        };
        let content = fs::read_to_string(&path).await?;
        let edits = input["edits"].as_array().cloned().unwrap_or_default();
        let mut new_content = content;
        for edit in edits {
            let old = edit["old_string"].as_str().unwrap_or("");
            let new = edit["new_string"].as_str().unwrap_or("");
            new_content = new_content.replacen(old, new, 1);
        }
        fs::write(&path, new_content).await?;
        Ok(ToolOutput::success(format!("updated {path}")))
    }
}
