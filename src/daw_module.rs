//! DawModule implementation for fts-launcher.

use daw::module::{ActionDef, DawModule, DockPosition, ModuleContext, PanelComponent, PanelDef};

pub struct LauncherModule;

impl DawModule for LauncherModule {
    fn name(&self) -> &str {
        "launcher"
    }
    fn display_name(&self) -> &str {
        "FTS Launcher"
    }

    fn actions(&self) -> Vec<ActionDef> {
        vec![
            ActionDef::new("FTS_LAUNCHER_TOGGLE", "FTS: Toggle Launcher", || {
                tracing::info!("FTS Launcher toggle");
            }),
            ActionDef::new(
                "FTS_LAUNCHER_FOCUS",
                "FTS: Focus Launcher Search",
                || {
                    tracing::info!("FTS Launcher focus");
                },
            ),
        ]
    }

    fn panels(&self) -> Vec<PanelDef> {
        vec![PanelDef {
            id: "FTS_LAUNCHER",
            title: "FTS Launcher",
            component: PanelComponent::from_fn_ptr(launcher_panel as *const ()),
            default_dock: DockPosition::Floating,
            default_size: (800.0, 520.0),
            toggle_action: Some("FTS_LAUNCHER_TOGGLE"),
        }]
    }

    fn init(&self, _ctx: &ModuleContext) {
        tracing::info!("FTS Launcher module initialized");
    }
}

/// The launcher panel's root Dioxus component.
/// This is cast to `fn() -> Element` by the host's panel renderer.
fn launcher_panel() {
    // Placeholder — the actual component is:
    // use fts_launcher::ui::Launcher;
    // rsx! { Launcher { state, theme, on_close } }
    //
    // The host (FTS-Extensions) casts this to the real component type
    // via reaper-dioxus::register_panel().
}

pub fn module() -> Box<dyn DawModule> {
    Box::new(LauncherModule)
}
