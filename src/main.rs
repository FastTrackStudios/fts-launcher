//! FTS Launcher — FastTrackStudios application launcher for Reaper.
//!
//! Built on the dioxus-launcher engine with Reaper-specific providers,
//! workflow packs, and tag definitions.

#[cfg(feature = "desktop")]
use dioxus::prelude::*;
#[cfg(feature = "native")]
use dioxus_native::prelude::*;

use launcher_core::QueryEngine;
use launcher_ui::components::Launcher;
use launcher_ui::state::LauncherState;
use launcher_ui::theme::Theme;
use providers::{ApplicationsProvider, CalcProvider, WorkflowProvider};

mod reaper;

fn build_engine() -> QueryEngine {
    // Load workflow packs
    let mut loaded_packs = Vec::new();

    // Bundled packs (shipped with fts-launcher)
    let bundled_dir = std::path::PathBuf::from(
        std::env::var("FTS_LAUNCHER_PACKS").unwrap_or_else(|_| "packs".into()),
    );
    loaded_packs.extend(launcher_core::pack::scan_packs(&bundled_dir));

    // User packs
    let user_dir = launcher_core::pack::default_pack_dir();
    loaded_packs.extend(launcher_core::pack::scan_packs(&user_dir));

    tracing::info!(packs = loaded_packs.len(), "Loaded workflow packs");

    let engine = QueryEngine::builder()
        .max_results(50)
        .register_tags(|tags| {
            reaper::register_reaper_tags(tags);
        })
        .register_packs(&loaded_packs)
        // Magic words
        .magic_word("C", "Compressors")
        .magic_word("R", "Reverbs")
        .magic_word("E", "EQ")
        .magic_word("I", "Instruments")
        .magic_word("A", "Actions")
        .magic_word("V", "Visibility")
        .magic_word("T", "Tracks")
        // Providers
        .provider(Box::new(reaper::ReaperActionsProvider::new()))
        .provider(Box::new(reaper::ReaperFxProvider::new()))
        .provider(Box::new(reaper::ReaperTracksProvider::new()))
        .provider(Box::new(ApplicationsProvider::new()))
        .provider(Box::new(CalcProvider::new()))
        .provider(Box::new(WorkflowProvider::from_items(
            loaded_packs.iter().flat_map(|p| p.to_items()).collect(),
        )))
        .build();

    // Register pack presets
    for pack in &loaded_packs {
        for preset in &pack.def.presets {
            if engine.load_preset(&preset.name).is_none() {
                engine.save_preset(
                    &preset.name,
                    launcher_core::FilterState {
                        include: preset.include.clone(),
                        exclude: preset.exclude.clone(),
                        ..Default::default()
                    },
                );
            }
        }
    }

    // Seed default presets
    use launcher_core::FilterState;
    let default_presets = [
        ("Compressors", vec!["audio/effects/dynamics"]),
        ("Reverbs", vec!["audio/effects/reverb"]),
        ("EQ", vec!["audio/effects/eq"]),
        ("Instruments", vec!["audio/instruments"]),
        ("Actions", vec!["reaper/actions"]),
        ("Tracks", vec!["reaper/tracks"]),
    ];
    for (name, include) in default_presets {
        if engine.load_preset(name).is_none() {
            engine.save_preset(
                name,
                FilterState {
                    include: include.into_iter().map(|s| s.to_string()).collect(),
                    ..Default::default()
                },
            );
        }
    }

    engine
}

fn app() -> Element {
    let state = use_signal(|| LauncherState::new(build_engine()));
    let on_close = |_: ()| close_window();

    rsx! {
        Launcher { state, theme: fts_theme(), on_close: on_close }
    }
}

/// FTS dark theme — based on Catppuccin Mocha with FTS accent colors.
fn fts_theme() -> Theme {
    let mut theme = Theme::dark();
    theme.name = "fts-dark".into();
    // FTS accent: warm gold
    theme.accent = "#f5c2e7".into();
    theme.accent_hover = "#f5c2e7".into();
    theme
}

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter("info,wgpu_hal=error,wgpu_core=error")
        .init();

    let engine = build_engine();
    tracing::info!(providers = engine.provider_names().len(), "FTS Launcher initialized");
    drop(engine);

    #[cfg(feature = "desktop")]
    {
        dioxus::LaunchBuilder::new()
            .with_cfg(
                dioxus::desktop::Config::new().with_window(
                    dioxus::desktop::WindowBuilder::new()
                        .with_title("FTS Launcher")
                        .with_inner_size(dioxus::desktop::LogicalSize::new(800.0, 520.0))
                        .with_decorations(false)
                        .with_always_on_top(true)
                        .with_resizable(true),
                ),
            )
            .launch(app);
    }

    #[cfg(feature = "native")]
    {
        use std::any::Any;
        let window_attrs = winit::window::WindowAttributes::default()
            .with_title("FTS Launcher")
            .with_surface_size(winit::dpi::LogicalSize::new(800.0, 520.0))
            .with_decorations(false)
            .with_resizable(true);
        let configs: Vec<Box<dyn Any>> = vec![Box::new(window_attrs)];
        dioxus_native::launch_cfg(app, Vec::new(), configs);
    }
}

fn close_window() {
    std::process::exit(0);
}
