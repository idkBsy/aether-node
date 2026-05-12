// SUMMARY: Extensible OSINT provider trait for the recursive investigation loop.
use async_trait::async_trait;
use crate::error::AetherError;
use crate::cortex::attribution::Entity;

/// Extensible trait for recursive OSINT investigations.
#[async_trait]
pub trait OsintProvider: Send + Sync {
    /// Name of the provider
    fn name(&self) -> &str;

    /// Triggers an investigation based on a confirmed hardware signature
    async fn investigate(&self, entity: &Entity) -> Result<String, AetherError>;
}

/// A dummy local provider that simulates server log correlation.
pub struct LocalLogProvider;

#[async_trait]
impl OsintProvider for LocalLogProvider {
    fn name(&self) -> &str {
        "LocalLogProvider"
    }

    async fn investigate(&self, entity: &Entity) -> Result<String, AetherError> {
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
        
        Ok(format!(
            "[{}] Confirmed correlation: Entity '{}' matched with signature \u{0394} = {:.6} (Confidence: {:.2}%)",
            self.name(),
            entity.name,
            entity.signature_delta,
            entity.confidence * 100.0
        ))
    }
}
