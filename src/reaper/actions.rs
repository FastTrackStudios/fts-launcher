//! DAW Actions provider.
//!
//! The action registry in the `daw` crate is for registering/executing actions,
//! not enumerating them. For now, we provide a curated set of common actions.
//! When running inside Reaper, the full action list can be read via
//! the Reaper-specific API extension.

use launcher_core::{
    ActionModifier, ActivationResult, Item, ItemAction, Provider, ProviderConfig,
};

pub struct DawActionsProvider {
    config: ProviderConfig,
    actions: Vec<Item>,
}

impl DawActionsProvider {
    pub fn new() -> Self {
        Self {
            config: ProviderConfig {
                name: "actions".into(),
                icon: "A".into(),
                prefix: None,
                default_tags: vec!["reaper/actions".into()],
                ..Default::default()
            },
            actions: Vec::new(),
        }
    }
}

impl Provider for DawActionsProvider {
    fn name(&self) -> &str { "actions" }
    fn config(&self) -> &ProviderConfig { &self.config }
    fn config_mut(&mut self) -> &mut ProviderConfig { &mut self.config }

    fn setup(&mut self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        self.actions = common_actions();
        tracing::info!(count = self.actions.len(), "Loaded DAW actions");
        Ok(())
    }

    fn query(&self, _q: &str, _exact: bool) -> Result<Vec<Item>, Box<dyn std::error::Error + Send + Sync>> {
        Ok(self.actions.clone())
    }

    fn activate(&self, item: &Item, action: &str) -> Result<ActivationResult, Box<dyn std::error::Error + Send + Sync>> {
        let Some(daw) = daw::get() else { return Ok(ActivationResult::Close); };

        let exec = item.actions.iter().find(|a| a.name == action).map(|a| a.exec.as_str()).unwrap_or("");
        let parts: Vec<&str> = exec.splitn(3, ':').collect();
        if parts.len() < 3 { return Ok(ActivationResult::Close); }

        let command_id: u32 = parts[2].parse().unwrap_or(0);
        if command_id == 0 { return Ok(ActivationResult::Close); }

        let keep_open = item.actions.iter().find(|a| a.name == action).map(|a| a.keep_open).unwrap_or(false);

        daw::block_on(async {
            daw.action_registry().execute_command(command_id).await.ok()
        });

        if keep_open { Ok(ActivationResult::KeepOpen) } else { Ok(ActivationResult::Close) }
    }
}

fn common_actions() -> Vec<Item> {
    let actions = [
        ("play",     "Play/Stop",                "Transport", 1007),
        ("record",   "Record",                   "Transport", 1013),
        ("pause",    "Pause",                    "Transport", 1008),
        ("stop",     "Stop",                     "Transport", 1016),
        ("rewind",   "Go to start",              "Transport", 40042),
        ("repeat",   "Toggle repeat",            "Transport", 1068),
        ("save",     "Save project",             "File",      40026),
        ("saveas",   "Save project as...",       "File",      40022),
        ("render",   "Render project to disk",   "File",      40015),
        ("open",     "Open project...",          "File",      40025),
        ("new",      "New project",              "File",      40023),
        ("undo",     "Undo",                     "Edit",      40029),
        ("redo",     "Redo",                     "Edit",      40030),
        ("copy",     "Copy items",               "Edit",      40057),
        ("cut",      "Cut items",                "Edit",      40059),
        ("paste",    "Paste items",              "Edit",      40058),
        ("selall",   "Select all items",         "Edit",      40182),
        ("addtrk",   "Insert new track",         "Track",     40001),
        ("deltrk",   "Remove selected tracks",   "Track",     40005),
        ("mute",     "Toggle mute",              "Track",     40281),
        ("solo",     "Toggle solo",              "Track",     40282),
        ("arm",      "Toggle arm",               "Track",     40294),
        ("mixer",    "Toggle mixer",             "View",      40078),
        ("fxbrow",   "Show FX browser",          "View",      40271),
        ("actlist",  "Show action list",         "View",      40605),
    ];

    actions
        .iter()
        .map(|(id, name, section, cmd_id)| {
            let tag = format!("reaper/actions/{}", section.to_lowercase());
            Item::new(&format!("action-{id}"), *name, "actions")
                .with_sub(&format!("Action: {section} > {name}"))
                .with_icon("A")
                .with_tags(&["reaper/actions", &tag])
                .with_search_fields(vec![name.to_string(), section.to_string()])
                .with_actions(vec![
                    ItemAction::new("Run", format!("daw:action:{cmd_id}")),
                    ItemAction::new("Run (keep open)", format!("daw:action:{cmd_id}"))
                        .with_modifier(ActionModifier::Shift)
                        .with_keep_open(),
                ])
        })
        .collect()
}
