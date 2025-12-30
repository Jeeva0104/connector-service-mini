use crate::types::HeaderMaskingConfig;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

#[derive(Clone)]
pub struct AppConfig {
    pub unmasked_headers: HeaderMaskingConfig,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            unmasked_headers: HeaderMaskingConfig::default(),
        }
    }
}

#[derive(Clone)]
pub struct AppState {
    pub flow_name: String,
    // pub vehicles: Arc<Mutex<HashMap<Uuid, VehicleInfo>>>,
    pub config: AppConfig,
    // TODO: use a separate schema for accounts_store
}

impl AppState {
    pub fn new() -> Self {
        Self {
            flow_name: "vehicle_management".to_string(),
            // vehicles: Arc::new(Mutex::new(HashMap::new())),
            config: AppConfig::default(),
        }
    }

    pub fn conf(&self) -> &AppConfig {
        &self.config
    }
}
