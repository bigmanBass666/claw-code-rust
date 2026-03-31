use std::sync::Arc;

use futures::StreamExt;
use tracing::{debug, info, warn};

use agent_provider::{ModelProvider, ModelRequest, ResponseContent, StopReason, StreamEvent};
use agent_tools::{ToolCall, ToolContext, ToolOrchestrator, ToolRegistry};

use crate::{AgentError, ContentBlock, Message, Role, SessionState};

/// Events emitted during a query for the caller (CLI/UI) to observe.
#[derive(Debug, Clone)]
pub enum QueryEvent {
    /// Incremental text from the assistant.
    TextDelta(String),
    /// The assistant started a tool call.
    ToolUseStart { id: String, name: String },
    /// A tool call completed.
    ToolResult {
        tool_use_id: String,
        content: String,
        is_error: bool,
    },
    /// A turn is complete (model stopped generating).
    TurnComplete { stop_reason: StopReason },
    /// Token usage update.
    Usage {
        input_tokens: usize,
        output_tokens: usize,
    },
}

/// Callback for streaming query events to the UI layer.
pub type EventCallback = Arc<dyn Fn(QueryEvent) + Send + Sync>;

/// The recursive agent loop — the beating heart of the runtime.
///
/// This is the Rust equivalent of Claude Code's `query.ts`. It drives
/// multi-turn conversations by:
///
/// 1. Building the model request from session state
/// 2. Streaming the model response
/// 3. Collecting assistant text and tool_use blocks
/// 4. Executing tool calls via the orchestrator
/// 5. Appending tool_result messages
/// 6. Recursing if the model wants to continue
///
/// The loop terminates when:
/// - The model emits `end_turn` with no tool calls
/// - Max turns are exceeded
/// - An unrecoverable error occurs
pub async fn query(
    session: &mut SessionState,
    provider: &dyn ModelProvider,
    registry: Arc<ToolRegistry>,
    orchestrator: &ToolOrchestrator,
    on_event: Option<EventCallback>,
) -> Result<(), AgentError> {
    let emit = |event: QueryEvent| {
        if let Some(ref cb) = on_event {
            cb(event);
        }
    };

    loop {
        if session.turn_count >= session.config.max_turns {
            return Err(AgentError::MaxTurnsExceeded(session.config.max_turns));
        }

        session.turn_count += 1;
        info!(turn = session.turn_count, "starting turn");

        // Build model request
        let request = ModelRequest {
            model: session.config.model.clone(),
            system: if session.config.system_prompt.is_empty() {
                None
            } else {
                Some(session.config.system_prompt.clone())
            },
            messages: session.to_request_messages(),
            max_tokens: session.config.token_budget.max_output_tokens,
            tools: Some(registry.tool_definitions()),
            temperature: None,
        };

        // Stream model response
        let mut stream = provider
            .stream(request)
            .await
            .map_err(AgentError::Provider)?;

        let mut assistant_text = String::new();
        let mut tool_uses: Vec<(String, String, String)> = Vec::new(); // (id, name, json_accum)
        let mut stop_reason = None;

        while let Some(event) = stream.next().await {
            match event {
                Ok(StreamEvent::TextDelta { text, .. }) => {
                    assistant_text.push_str(&text);
                    emit(QueryEvent::TextDelta(text));
                }
                Ok(StreamEvent::ContentBlockStart {
                    content: ResponseContent::ToolUse { id, name, .. },
                    ..
                }) => {
                    emit(QueryEvent::ToolUseStart {
                        id: id.clone(),
                        name: name.clone(),
                    });
                    tool_uses.push((id, name, String::new()));
                }
                Ok(StreamEvent::InputJsonDelta { partial_json, .. }) => {
                    if let Some(last) = tool_uses.last_mut() {
                        last.2.push_str(&partial_json);
                    }
                }
                Ok(StreamEvent::MessageDone { response }) => {
                    stop_reason = response.stop_reason.clone();
                    session.total_input_tokens += response.usage.input_tokens;
                    session.total_output_tokens += response.usage.output_tokens;
                    emit(QueryEvent::Usage {
                        input_tokens: response.usage.input_tokens,
                        output_tokens: response.usage.output_tokens,
                    });
                }
                Ok(_) => {}
                Err(e) => {
                    warn!(error = %e, "stream error");
                    return Err(AgentError::Provider(e));
                }
            }
        }

        // Build assistant message
        let mut assistant_content: Vec<ContentBlock> = Vec::new();

        if !assistant_text.is_empty() {
            assistant_content.push(ContentBlock::Text {
                text: assistant_text,
            });
        }

        let tool_calls: Vec<ToolCall> = tool_uses
            .into_iter()
            .map(|(id, name, json_str)| {
                let input = serde_json::from_str(&json_str).unwrap_or(serde_json::Value::Object(
                    serde_json::Map::new(),
                ));
                assistant_content.push(ContentBlock::ToolUse {
                    id: id.clone(),
                    name: name.clone(),
                    input: input.clone(),
                });
                ToolCall { id, name, input }
            })
            .collect();

        session.push_message(Message {
            role: Role::Assistant,
            content: assistant_content,
        });

        // If no tool calls, we're done
        if tool_calls.is_empty() {
            if let Some(sr) = stop_reason {
                emit(QueryEvent::TurnComplete { stop_reason: sr });
            }
            debug!("no tool calls, ending query loop");
            return Ok(());
        }

        // Execute tool calls
        let tool_ctx = ToolContext {
            cwd: session.cwd.clone(),
            permissions: Arc::new(
                agent_permissions::RuleBasedPolicy::new(agent_permissions::PermissionMode::AutoApprove),
            ),
            session_id: session.id.clone(),
        };

        let results = orchestrator.execute_batch(&tool_calls, &tool_ctx).await;

        // Build tool result message (user role, per Anthropic API convention)
        let result_content: Vec<ContentBlock> = results
            .into_iter()
            .map(|r| {
                emit(QueryEvent::ToolResult {
                    tool_use_id: r.tool_use_id.clone(),
                    content: r.output.content.clone(),
                    is_error: r.output.is_error,
                });
                ContentBlock::ToolResult {
                    tool_use_id: r.tool_use_id,
                    content: r.output.content,
                    is_error: r.output.is_error,
                }
            })
            .collect();

        session.push_message(Message {
            role: Role::User,
            content: result_content,
        });

        // The model used tools → continue the loop for the next turn
        if stop_reason == Some(StopReason::EndTurn) {
            debug!("stop_reason=end_turn after tool use, ending");
            return Ok(());
        }
    }
}
