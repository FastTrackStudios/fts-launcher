//! Reaper Actions provider.
//!
//! Sources actions from the Reaper action list.
//! In a full integration this would read from the Reaper API via reaper-rs.
//! For now it provides a curated set of common actions.

use launcher_core::{
    ActionModifier, ActivationResult, Item, ItemAction, Provider, ProviderConfig,
};

pub struct ReaperActionsProvider {
    config: ProviderConfig,
    actions: Vec<ReaperAction>,
}

struct ReaperAction {
    id: String,
    name: String,
    section: String,
    command_id: String,
}

impl ReaperActionsProvider {
    pub fn new() -> Self {
        Self {
            config: ProviderConfig {
                name: "reaper-actions".into(),
                icon: "A".into(),
                prefix: None, // 'a' alias handles this via tags
                default_tags: vec!["reaper/actions".into()],
                ..Default::default()
            },
            actions: Vec::new(),
        }
    }

    fn load_actions(&mut self) {
        // TODO: In a real Reaper extension, this would call the Reaper API:
        // reaper.enum_actions() to get the full action list.
        // For now, seed with common built-in actions.
        self.actions = common_reaper_actions();
        tracing::info!(count = self.actions.len(), "Loaded Reaper actions");
    }
}

impl Provider for ReaperActionsProvider {
    fn name(&self) -> &str {
        "reaper-actions"
    }
    fn config(&self) -> &ProviderConfig {
        &self.config
    }
    fn config_mut(&mut self) -> &mut ProviderConfig {
        &mut self.config
    }

    fn setup(&mut self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        self.load_actions();
        Ok(())
    }

    fn query(
        &self,
        _query: &str,
        _exact: bool,
    ) -> Result<Vec<Item>, Box<dyn std::error::Error + Send + Sync>> {
        let items = self
            .actions
            .iter()
            .map(|action| {
                let tag = format!("reaper/actions/{}", action.section.to_lowercase());
                Item::new(&action.id, &action.name, "reaper-actions")
                    .with_sub(&format!("Action: {} > {}", action.section, action.name))
                    .with_icon("A")
                    .with_tags(&["reaper/actions", &tag])
                    .with_search_fields(vec![
                        action.name.clone(),
                        action.section.clone(),
                        action.command_id.clone(),
                    ])
                    .with_actions(vec![
                        ItemAction::new("Run", format!("reaper:{}", action.command_id)),
                        ItemAction::new("Run (keep open)", format!("reaper:{}", action.command_id))
                            .with_modifier(ActionModifier::Shift)
                            .with_keep_open(),
                    ])
            })
            .collect();
        Ok(items)
    }

    fn activate(
        &self,
        item: &Item,
        action: &str,
    ) -> Result<ActivationResult, Box<dyn std::error::Error + Send + Sync>> {
        tracing::info!(action = action, item = %item.label, "Reaper action");
        // TODO: Call reaper.main_on_command() via reaper-rs
        Ok(ActivationResult::Close)
    }
}

fn common_reaper_actions() -> Vec<ReaperAction> {
    vec![
        // Transport
        ReaperAction { id: "ra-play".into(), name: "Play/Stop".into(), section: "Transport".into(), command_id: "1007".into() },
        ReaperAction { id: "ra-record".into(), name: "Record".into(), section: "Transport".into(), command_id: "1013".into() },
        ReaperAction { id: "ra-pause".into(), name: "Pause".into(), section: "Transport".into(), command_id: "1008".into() },
        ReaperAction { id: "ra-stop".into(), name: "Stop".into(), section: "Transport".into(), command_id: "1016".into() },
        ReaperAction { id: "ra-rewind".into(), name: "Go to start".into(), section: "Transport".into(), command_id: "40042".into() },
        ReaperAction { id: "ra-repeat".into(), name: "Toggle repeat".into(), section: "Transport".into(), command_id: "1068".into() },
        // File
        ReaperAction { id: "ra-save".into(), name: "Save project".into(), section: "File".into(), command_id: "40026".into() },
        ReaperAction { id: "ra-saveas".into(), name: "Save project as...".into(), section: "File".into(), command_id: "40022".into() },
        ReaperAction { id: "ra-render".into(), name: "Render project to disk".into(), section: "File".into(), command_id: "40015".into() },
        ReaperAction { id: "ra-open".into(), name: "Open project...".into(), section: "File".into(), command_id: "40025".into() },
        ReaperAction { id: "ra-new".into(), name: "New project".into(), section: "File".into(), command_id: "40023".into() },
        // Edit
        ReaperAction { id: "ra-undo".into(), name: "Undo".into(), section: "Edit".into(), command_id: "40029".into() },
        ReaperAction { id: "ra-redo".into(), name: "Redo".into(), section: "Edit".into(), command_id: "40030".into() },
        ReaperAction { id: "ra-copy".into(), name: "Copy items".into(), section: "Edit".into(), command_id: "40057".into() },
        ReaperAction { id: "ra-cut".into(), name: "Cut items".into(), section: "Edit".into(), command_id: "40059".into() },
        ReaperAction { id: "ra-paste".into(), name: "Paste items".into(), section: "Edit".into(), command_id: "40058".into() },
        ReaperAction { id: "ra-selectall".into(), name: "Select all items".into(), section: "Edit".into(), command_id: "40182".into() },
        // Track
        ReaperAction { id: "ra-addtrack".into(), name: "Insert new track".into(), section: "Track".into(), command_id: "40001".into() },
        ReaperAction { id: "ra-deltrack".into(), name: "Remove selected tracks".into(), section: "Track".into(), command_id: "40005".into() },
        ReaperAction { id: "ra-mutetrack".into(), name: "Toggle mute for selected tracks".into(), section: "Track".into(), command_id: "40281".into() },
        ReaperAction { id: "ra-solotrack".into(), name: "Toggle solo for selected tracks".into(), section: "Track".into(), command_id: "40282".into() },
        ReaperAction { id: "ra-armtrack".into(), name: "Toggle arm for selected tracks".into(), section: "Track".into(), command_id: "40294".into() },
        // View
        ReaperAction { id: "ra-mixer".into(), name: "Toggle mixer".into(), section: "View".into(), command_id: "40078".into() },
        ReaperAction { id: "ra-fxbrowser".into(), name: "Show FX browser".into(), section: "View".into(), command_id: "40271".into() },
        ReaperAction { id: "ra-actions".into(), name: "Show action list".into(), section: "View".into(), command_id: "40605".into() },
        ReaperAction { id: "ra-routing".into(), name: "Show routing matrix".into(), section: "View".into(), command_id: "40251".into() },
    ]
}
