//! FTS Launcher — library for embedding the launcher in Reaper extensions.
//!
//! # Usage from a Reaper extension
//!
//! ```rust,ignore
//! use fts_launcher::{LauncherEngine, ActionDefs};
//!
//! // In plugin_main:
//! let engine = LauncherEngine::new();
//!
//! // Get action definitions to register with REAPER
//! let actions = engine.action_defs();
//! // Register actions via daw.action_registry()...
//!
//! // When "FTS_LAUNCHER_TOGGLE" action is triggered:
//! engine.toggle();
//! ```

pub mod reaper;

use std::sync::Arc;

use launcher_core::{FilterState, QueryEngine};
use providers::{CalcProvider, ExtensionProvider, WorkflowProvider};

pub use launcher_core;

#[cfg(feature = "ui")]
pub use launcher_ui;

/// Action handler type: (command_id, display_name, handler).
pub type ActionDef = (String, String, Arc<dyn Fn() + Send + Sync>);

/// The launcher engine — holds the query engine, providers, and configuration.
/// Created once at extension startup, used to build UI and handle actions.
pub struct LauncherEngine {
    engine: QueryEngine,
    ext_registry: launcher_core::ExtensionRegistry,
}

impl LauncherEngine {
    /// Create a new launcher engine, loading packs and extensions from
    /// standard directories.
    pub fn new() -> Self {
        // Load workflow packs
        let mut loaded_packs = Vec::new();
        let bundled_dir = std::path::PathBuf::from(
            std::env::var("FTS_LAUNCHER_PACKS").unwrap_or_else(|_| "packs".into()),
        );
        loaded_packs.extend(launcher_core::pack::scan_packs(&bundled_dir));
        let user_dir = launcher_core::pack::default_pack_dir();
        loaded_packs.extend(launcher_core::pack::scan_packs(&user_dir));
        tracing::info!(packs = loaded_packs.len(), "Loaded workflow packs");

        // Load extensions
        let mut ext_registry = launcher_core::ExtensionRegistry::new();
        let bundled_ext = std::path::PathBuf::from(
            std::env::var("FTS_LAUNCHER_EXTENSIONS").unwrap_or_else(|_| "extensions".into()),
        );
        ext_registry.scan_dir(&bundled_ext);
        let user_ext = launcher_core::extension::default_extensions_dir();
        ext_registry.scan_dir(&user_ext);
        tracing::info!(extensions = ext_registry.extensions().len(), "Loaded extensions");

        let engine = QueryEngine::builder()
            .max_results(50)
            .register_tags(|tags| {
                reaper::register_reaper_tags(tags);
            })
            .register_packs(&loaded_packs)
            .register_tags(|tags| {
                ext_registry.register_tags(tags);
            })
            .magic_word("C", "Compressors")
            .magic_word("R", "Reverbs")
            .magic_word("E", "EQ")
            .magic_word("I", "Instruments")
            .magic_word("A", "Actions")
            .magic_word("V", "Visibility")
            .magic_word("T", "Tracks")
            // DAW-powered providers
            .provider(Box::new(reaper::DawTracksProvider::new()))
            .provider(Box::new(reaper::DawFxProvider::new()))
            .provider(Box::new(reaper::DawActionsProvider::new()))
            .provider(Box::new(reaper::DawMarkersProvider::new()))
            .provider(Box::new(reaper::DawTransportProvider::new()))
            // Generic providers
            .provider(Box::new(CalcProvider::new()))
            .provider(Box::new(WorkflowProvider::from_items(
                loaded_packs.iter().flat_map(|p| p.to_items()).collect(),
            )))
            .provider(Box::new(ExtensionProvider::from_registry(&ext_registry)))
            .build();

        // Register pack presets
        for pack in &loaded_packs {
            for preset in &pack.def.presets {
                if engine.load_preset(&preset.name).is_none() {
                    engine.save_preset(
                        &preset.name,
                        FilterState {
                            include: preset.include.clone(),
                            exclude: preset.exclude.clone(),
                            ..Default::default()
                        },
                    );
                }
            }
        }

        // Seed default presets
        for (name, include) in [
            ("Compressors", vec!["audio/effects/dynamics"]),
            ("Reverbs", vec!["audio/effects/reverb"]),
            ("EQ", vec!["audio/effects/eq"]),
            ("Instruments", vec!["audio/instruments"]),
            ("Actions", vec!["reaper/actions"]),
            ("Tracks", vec!["reaper/tracks"]),
        ] {
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

        tracing::info!(
            providers = engine.provider_names().len(),
            "FTS Launcher engine initialized"
        );

        Self { engine, ext_registry }
    }

    /// Get the query engine.
    pub fn into_engine(self) -> QueryEngine {
        self.engine
    }

    /// Get a reference to the query engine.
    pub fn engine(&self) -> &QueryEngine {
        &self.engine
    }

    /// Create a LauncherState for use with the Dioxus UI.
    #[cfg(feature = "ui")]
    pub fn into_state(self) -> launcher_ui::state::LauncherState {
        launcher_ui::state::LauncherState::new(self.engine)
    }

    /// Get the FTS dark theme.
    #[cfg(feature = "ui")]
    pub fn theme() -> launcher_ui::theme::Theme {
        let mut theme = launcher_ui::theme::Theme::dark();
        theme.name = "fts-dark".into();
        theme.accent = "#f5c2e7".into();
        theme.accent_hover = "#f5c2e7".into();
        theme
    }

    /// Build action definitions for registering with REAPER.
    /// Returns tuples of (command_id, display_name, handler).
    pub fn action_defs() -> Vec<ActionDef> {
        vec![
            (
                "FTS_LAUNCHER_TOGGLE".to_string(),
                "FTS: Toggle Launcher".to_string(),
                Arc::new(|| {
                    tracing::info!("FTS Launcher toggle requested");
                    // The actual toggle is handled by the extension host
                    // which owns the window/panel lifecycle.
                }),
            ),
            (
                "FTS_LAUNCHER_FOCUS".to_string(),
                "FTS: Focus Launcher Search".to_string(),
                Arc::new(|| {
                    tracing::info!("FTS Launcher focus requested");
                }),
            ),
        ]
    }
}

impl Default for LauncherEngine {
    fn default() -> Self {
        Self::new()
    }
}
