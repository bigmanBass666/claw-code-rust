mod bash;
mod file_read;
mod file_write;

pub use bash::BashTool;
pub use file_read::FileReadTool;
pub use file_write::FileWriteTool;

use std::sync::Arc;

use agent_tools::ToolRegistry;

/// Register all built-in tools into a registry.
pub fn register_builtin_tools(registry: &mut ToolRegistry) {
    registry.register(Arc::new(BashTool));
    registry.register(Arc::new(FileReadTool));
    registry.register(Arc::new(FileWriteTool));
}
