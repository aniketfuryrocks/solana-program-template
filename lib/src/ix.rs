use serde::{Deserialize, Serialize};

/// Program Instructions
#[derive(Serialize, Deserialize)]
pub enum Ix {
    /// Create a new pool
    CreatePool,
}
