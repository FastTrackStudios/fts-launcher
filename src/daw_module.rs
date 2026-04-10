//! DawModule implementation for fts-launcher.

use daw::module::{ActionDef, DawModule, ModuleContext};

pub struct LauncherModule;

impl DawModule for LauncherModule {
    fn name(&self) -> &str { "launcher" }
    fn display_name(&self) -> &str { "FTS Launcher" }

    fn actions(&self) -> Vec<ActionDef> {
        vec![
            ActionDef::new("FTS_LAUNCHER_TOGGLE", "FTS: Toggle Launcher", || {
                tracing::info!("FTS Launcher toggle");
            }),
            ActionDef::new("FTS_LAUNCHER_FOCUS", "FTS: Focus Launcher Search", || {
                tracing::info!("FTS Launcher focus");
            }),
        ]
    }

    fn init(&self, _ctx: &ModuleContext) {
        tracing::info!("FTS Launcher module initialized");
        // The LauncherEngine is created lazily when the UI panel is shown
    }
}

pub fn module() -> Box<dyn DawModule> {
    Box::new(LauncherModule)
}
