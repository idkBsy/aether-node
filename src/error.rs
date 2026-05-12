use thiserror::Error;

#[derive(Error, Debug)]
pub enum AetherError {
    #[error("Hardware capture initialization failed: {0}")]
    CaptureInitFault(String),
    #[error("Hardware execution fault: {0}")]
    HardwareFault(String),
    #[error("GPU/ROCm initialization or execution failed: {0}")]
    GpuFault(String),
    #[error("OSINT Provider Error: {0}")]
    ProviderFault(String),
}
