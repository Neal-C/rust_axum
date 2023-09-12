use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AppCtx {
    user_id: u64,
}

// Constructor
impl AppCtx {
    pub fn new(user_id: u64) -> Self {
        Self { user_id }
    }
}

impl AppCtx {
    pub fn user_id(&self) -> u64 {
        self.user_id
    }
}
