use std::sync::Arc;

use yams_core::App;

pub struct AppApi {
    app: Arc<App>,
}

impl AppApi {
    pub fn new(app: App) -> Self {
        Self { app: Arc::new(app) }
    }
}
