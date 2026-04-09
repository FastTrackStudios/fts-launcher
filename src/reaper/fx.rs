//! Reaper FX provider.
//!
//! Sources FX plugins from the Reaper FX database.
//! In a full integration this reads from Reaper's plugin scan cache.
//! For now provides demo entries.

use launcher_core::{
    ActionModifier, ActivationResult, Item, ItemAction, Provider, ProviderConfig,
};

pub struct ReaperFxProvider {
    config: ProviderConfig,
    plugins: Vec<FxEntry>,
}

struct FxEntry {
    id: String,
    name: String,
    developer: String,
    format: String,
    is_instrument: bool,
    category: String,
}

impl ReaperFxProvider {
    pub fn new() -> Self {
        Self {
            config: ProviderConfig {
                name: "reaper-fx".into(),
                icon: "F".into(),
                prefix: Some('f'),
                ..Default::default()
            },
            plugins: Vec::new(),
        }
    }
}

impl Provider for ReaperFxProvider {
    fn name(&self) -> &str { "reaper-fx" }
    fn config(&self) -> &ProviderConfig { &self.config }
    fn config_mut(&mut self) -> &mut ProviderConfig { &mut self.config }

    fn setup(&mut self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // TODO: Read from Reaper's reaper-vstplugins64.ini or API
        self.plugins = demo_fx_entries();
        tracing::info!(count = self.plugins.len(), "Loaded Reaper FX");
        Ok(())
    }

    fn query(&self, _query: &str, _exact: bool) -> Result<Vec<Item>, Box<dyn std::error::Error + Send + Sync>> {
        let items = self.plugins.iter().map(|fx| {
            let base_tag = if fx.is_instrument { "audio/instruments" } else { "audio/effects" };
            let format_tag = format!("audio/effects/plugin/{}", fx.format.to_lowercase());
            let cat_tag = if !fx.category.is_empty() {
                format!("{}/{}", base_tag, fx.category.to_lowercase())
            } else {
                base_tag.to_string()
            };

            Item::new(&fx.id, &fx.name, "reaper-fx")
                .with_sub(&format!("{} — {} ({})", fx.developer, fx.category, fx.format))
                .with_icon("F")
                .with_tags(&[base_tag, &format_tag, &cat_tag])
                .with_search_fields(vec![
                    fx.name.clone(), fx.developer.clone(), fx.format.clone(), fx.category.clone(),
                ])
                .with_actions(vec![
                    ItemAction::new("Add to track", format!("reaper:fx-add:{}", fx.id)),
                    ItemAction::new("Add to new track", format!("reaper:fx-new-track:{}", fx.id))
                        .with_modifier(ActionModifier::Shift),
                    ItemAction::new("Replace chain", format!("reaper:fx-replace:{}", fx.id))
                        .with_modifier(ActionModifier::CtrlShift),
                ])
        }).collect();
        Ok(items)
    }

    fn activate(&self, item: &Item, action: &str) -> Result<ActivationResult, Box<dyn std::error::Error + Send + Sync>> {
        tracing::info!(action = action, item = %item.label, "Reaper FX action");
        Ok(ActivationResult::Close)
    }
}

fn demo_fx_entries() -> Vec<FxEntry> {
    vec![
        FxEntry { id: "fx-reaverbate".into(), name: "ReaVerbate".into(), developer: "Cockos".into(), format: "VST".into(), is_instrument: false, category: "reverb".into() },
        FxEntry { id: "fx-reaeq".into(), name: "ReaEQ".into(), developer: "Cockos".into(), format: "VST".into(), is_instrument: false, category: "eq".into() },
        FxEntry { id: "fx-reacomp".into(), name: "ReaComp".into(), developer: "Cockos".into(), format: "VST".into(), is_instrument: false, category: "dynamics".into() },
        FxEntry { id: "fx-readelay".into(), name: "ReaDelay".into(), developer: "Cockos".into(), format: "VST".into(), is_instrument: false, category: "delay".into() },
        FxEntry { id: "fx-reagate".into(), name: "ReaGate".into(), developer: "Cockos".into(), format: "VST".into(), is_instrument: false, category: "dynamics".into() },
        FxEntry { id: "fx-reafir".into(), name: "ReaFir".into(), developer: "Cockos".into(), format: "VST".into(), is_instrument: false, category: "eq".into() },
        FxEntry { id: "fx-reastream".into(), name: "ReaStream".into(), developer: "Cockos".into(), format: "VST".into(), is_instrument: false, category: "utility".into() },
        FxEntry { id: "fx-vital".into(), name: "Vital".into(), developer: "Matt Tytel".into(), format: "VST3".into(), is_instrument: true, category: "synth".into() },
        FxEntry { id: "fx-samplomatic".into(), name: "ReaSamplomatic5000".into(), developer: "Cockos".into(), format: "VST".into(), is_instrument: true, category: "sampler".into() },
        FxEntry { id: "fx-pianoone".into(), name: "Piano One".into(), developer: "SoundMagic".into(), format: "VST".into(), is_instrument: true, category: "piano".into() },
        FxEntry { id: "fx-surgext".into(), name: "Surge XT".into(), developer: "Surge Synth Team".into(), format: "CLAP".into(), is_instrument: true, category: "synth".into() },
        FxEntry { id: "fx-dexed".into(), name: "Dexed".into(), developer: "Digital Suburban".into(), format: "VST3".into(), is_instrument: true, category: "synth".into() },
    ]
}
