use std::path::PathBuf;

use chrono::{DateTime, Utc};
use clawcr_core::{ItemId, SessionId, TurnId, TurnStatus};
use serde::{Deserialize, Serialize};
use smol_str::SmolStr;

/// Describes the transport kind used by one connected client.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ClientTransportKind {
    /// The client is connected over stdio.
    Stdio,
    /// The client is connected over a WebSocket transport.
    WebSocket,
    /// The client is connected through an embedded in-process bridge.
    Embedded,
}

/// Carries the data required by the initial `initialize` request.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InitializeParams {
    /// The human-readable client name.
    pub client_name: String,
    /// The client version string.
    pub client_version: String,
    /// The transport used by the client.
    pub transport: ClientTransportKind,
    /// Whether the client can consume streamed deltas.
    pub supports_streaming: bool,
    /// Whether the client supports binary image payloads directly.
    pub supports_binary_images: bool,
    /// Exact notification method names the client wants suppressed.
    pub opt_out_notification_methods: Vec<String>,
}

/// Carries the result returned by a successful `initialize` request.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InitializeResult {
    /// The server product name.
    pub server_name: String,
    /// The server version string.
    pub server_version: String,
    /// The operating-system family of the running server.
    pub platform_family: String,
    /// The operating-system identifier of the running server.
    pub platform_os: String,
    /// The server home directory.
    pub server_home: PathBuf,
    /// The capability flags supported by this server instance.
    pub capabilities: ServerCapabilities,
}

/// Advertises which runtime capabilities this server instance supports.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ServerCapabilities {
    /// Whether `session/resume` is supported.
    pub session_resume: bool,
    /// Whether `session/fork` is supported.
    pub session_fork: bool,
    /// Whether `turn/interrupt` is supported.
    pub turn_interrupt: bool,
    /// Whether approval requests may be routed to clients.
    pub approval_requests: bool,
    /// Whether streaming events are emitted.
    pub event_streaming: bool,
}

/// Describes the payload for `session/start`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SessionStartParams {
    /// The working directory for the new session.
    pub cwd: PathBuf,
    /// Whether the session should be treated as ephemeral.
    pub ephemeral: bool,
    /// The explicit title to assign at creation time, if any.
    pub title: Option<String>,
    /// An optional requested model slug.
    pub model: Option<String>,
}

/// Describes the response returned by `session/start`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SessionStartResult {
    /// The created session identifier.
    pub session_id: SessionId,
    /// The session creation timestamp.
    pub created_at: DateTime<Utc>,
    /// The working directory assigned to the session.
    pub cwd: PathBuf,
    /// Whether the session is ephemeral.
    pub ephemeral: bool,
    /// The model resolved for the initial session state.
    pub resolved_model: Option<String>,
}

/// Describes the payload for `turn/start`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TurnStartParams {
    /// The session receiving the new turn.
    pub session_id: SessionId,
    /// The primary user input for the turn.
    pub input: Vec<InputItem>,
    /// An optional requested model slug override.
    pub model: Option<String>,
    /// An optional sandbox override description.
    pub sandbox: Option<String>,
    /// An optional approval-policy override description.
    pub approval_policy: Option<String>,
    /// An optional working-directory override.
    pub cwd: Option<PathBuf>,
}

/// Describes the accepted result returned by `turn/start`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TurnStartResult {
    /// The created turn identifier.
    pub turn_id: TurnId,
    /// The initial accepted turn status.
    pub status: TurnStatus,
    /// The time when the turn was accepted.
    pub accepted_at: DateTime<Utc>,
}

/// Describes an input item accepted by the runtime API.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum InputItem {
    /// A plain text input item.
    Text {
        /// The text payload.
        text: String,
    },
    /// A skill reference input item.
    Skill {
        /// The referenced skill identifier.
        id: String,
    },
    /// A local image reference input item.
    LocalImage {
        /// The absolute filesystem path to the image.
        path: PathBuf,
    },
}

/// Describes the payload for `turn/interrupt`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TurnInterruptParams {
    /// The session that owns the turn.
    pub session_id: SessionId,
    /// The turn being interrupted.
    pub turn_id: TurnId,
    /// An optional human-readable interruption reason.
    pub reason: Option<String>,
}

/// Describes the payload returned by `turn/interrupt`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TurnInterruptResult {
    /// The interrupted turn identifier.
    pub turn_id: TurnId,
    /// The terminal interruption status.
    pub status: TurnStatus,
}

/// Describes the payload for `turn/steer`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TurnSteerParams {
    /// The session containing the active turn.
    pub session_id: SessionId,
    /// The turn identifier the client expects to still be active.
    pub expected_turn_id: TurnId,
    /// Additional same-turn user input.
    pub input: Vec<InputItem>,
}

/// Describes the response returned by `turn/steer`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TurnSteerResult {
    /// The turn that accepted the steering input.
    pub turn_id: TurnId,
}

/// Describes a client response to a pending approval request.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ApprovalRespondParams {
    /// The session that owns the approval request.
    pub session_id: SessionId,
    /// The turn that owns the approval request.
    pub turn_id: TurnId,
    /// The approval request identifier being answered.
    pub approval_id: SmolStr,
    /// The decision selected by the client.
    pub decision: ApprovalDecisionValue,
    /// The scope associated with the decision.
    pub scope: ApprovalScopeValue,
}

/// Enumerates client decisions for approval requests.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ApprovalDecisionValue {
    /// Approve the request.
    Approve,
    /// Deny the request.
    Deny,
    /// Cancel the request without granting it.
    Cancel,
}

/// Enumerates the scopes supported by approval responses.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ApprovalScopeValue {
    /// Apply the decision once.
    Once,
    /// Apply the decision for the remainder of the current turn.
    Turn,
    /// Apply the decision for the remainder of the session.
    Session,
    /// Apply the decision to a path-prefix resource scope.
    PathPrefix,
    /// Apply the decision to a host resource scope.
    Host,
    /// Apply the decision to a tool name scope.
    Tool,
}

/// Carries the common envelope used for server notifications.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotificationEnvelope<T> {
    /// The notification method name.
    pub method: String,
    /// The typed notification payload.
    pub params: T,
}

/// Carries the common correlation metadata attached to streamed events.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EventContext {
    /// The session associated with the event.
    pub session_id: SessionId,
    /// The turn associated with the event, when one exists.
    pub turn_id: Option<TurnId>,
    /// The item associated with the event, when one exists.
    pub item_id: Option<ItemId>,
    /// The per-connection monotonic event sequence number.
    pub seq: u64,
}

/// Carries the payload for a streamed item event.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ItemEventPayload {
    /// The event correlation context.
    pub context: EventContext,
    /// The persisted item kind name.
    pub item_kind: String,
    /// The event payload content.
    pub payload: serde_json::Value,
}

/// Enumerates the protocol errors exposed by the runtime server.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, thiserror::Error)]
pub enum ProtocolErrorCode {
    /// The client issued a request before initialization completed.
    #[error("not_initialized")]
    NotInitialized,
    /// The request payload was malformed or incomplete.
    #[error("invalid_params")]
    InvalidParams,
    /// The requested session does not exist.
    #[error("session_not_found")]
    SessionNotFound,
    /// The requested turn does not exist.
    #[error("turn_not_found")]
    TurnNotFound,
    /// Another turn is already active for the session.
    #[error("turn_already_running")]
    TurnAlreadyRunning,
    /// The referenced approval request does not exist.
    #[error("approval_not_found")]
    ApprovalNotFound,
    /// The action was denied by policy.
    #[error("policy_denied")]
    PolicyDenied,
    /// The request exceeded the active context budget.
    #[error("context_limit_exceeded")]
    ContextLimitExceeded,
    /// An internal invariant or transport failure occurred.
    #[error("internal_error")]
    InternalError,
}

/// Carries a typed protocol error response payload.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProtocolError {
    /// The stable protocol error code.
    pub code: ProtocolErrorCode,
    /// The human-readable error message.
    pub message: String,
    /// Optional structured error details.
    pub data: serde_json::Value,
}

#[cfg(test)]
mod tests {
    use super::{
        ApprovalDecisionValue, ApprovalRespondParams, ApprovalScopeValue, EventContext,
        InitializeParams, InputItem,
    };
    use clawcr_core::{SessionId, TurnId};

    #[test]
    fn initialize_params_roundtrip() {
        let params = InitializeParams {
            client_name: "desktop".into(),
            client_version: "1.0.0".into(),
            transport: super::ClientTransportKind::Stdio,
            supports_streaming: true,
            supports_binary_images: false,
            opt_out_notification_methods: vec!["turn/plan/updated".into()],
        };

        let json = serde_json::to_string(&params).expect("serialize");
        let restored: InitializeParams = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(params, restored);
    }

    #[test]
    fn approval_response_roundtrip() {
        let payload = ApprovalRespondParams {
            session_id: SessionId::new(),
            turn_id: TurnId::new(),
            approval_id: "approval-1".into(),
            decision: ApprovalDecisionValue::Approve,
            scope: ApprovalScopeValue::Session,
        };

        let json = serde_json::to_string(&payload).expect("serialize");
        let restored: ApprovalRespondParams = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(payload, restored);
    }

    #[test]
    fn event_context_keeps_correlation_ids() {
        let context = EventContext {
            session_id: SessionId::new(),
            turn_id: Some(TurnId::new()),
            item_id: None,
            seq: 7,
        };

        assert_eq!(context.seq, 7);
        assert!(context.turn_id.is_some());
    }

    #[test]
    fn input_item_serializes_tagged_shape() {
        let input = InputItem::Skill {
            id: "rust-docs".into(),
        };

        let json = serde_json::to_string(&input).expect("serialize");
        assert!(json.contains("\"type\":\"skill\""));
    }
}
