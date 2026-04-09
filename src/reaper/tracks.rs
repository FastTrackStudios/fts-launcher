//! Reaper Tracks provider.
//!
//! Sources tracks from the current Reaper project.
//! In a full integration this reads live track data via reaper-rs.

use launcher_core::{
    ActionModifier, ActivationResult, Item, ItemAction, Provider, ProviderConfig,
};

pub struct ReaperTracksProvider {
    config: ProviderConfig,
    tracks: Vec<TrackEntry>,
}

struct TrackEntry {
    id: String,
    name: String,
    index: usize,
    is_folder: bool,
    muted: bool,
    soloed: bool,
}

impl ReaperTracksProvider {
    pub fn new() -> Self {
        Self {
            config: ProviderConfig {
                name: "reaper-tracks".into(),
                icon: "T".into(),
                prefix: Some('t'),
                default_tags: vec!["reaper/tracks".into()],
                ..Default::default()
            },
            tracks: Vec::new(),
        }
    }
}

impl Provider for ReaperTracksProvider {
    fn name(&self) -> &str { "reaper-tracks" }
    fn config(&self) -> &ProviderConfig { &self.config }
    fn config_mut(&mut self) -> &mut ProviderConfig { &mut self.config }

    fn setup(&mut self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // TODO: Read from Reaper project via reaper-rs
        self.tracks = demo_tracks();
        tracing::info!(count = self.tracks.len(), "Loaded Reaper tracks");
        Ok(())
    }

    fn query(&self, _query: &str, _exact: bool) -> Result<Vec<Item>, Box<dyn std::error::Error + Send + Sync>> {
        let items = self.tracks.iter().map(|track| {
            let status = match (track.muted, track.soloed) {
                (true, _) => " [muted]",
                (_, true) => " [solo]",
                _ => "",
            };
            let sub = format!("Track {}{}{}", track.index + 1,
                if track.is_folder { " (folder)" } else { "" }, status);

            Item::new(&track.id, &track.name, "reaper-tracks")
                .with_sub(&sub)
                .with_icon(if track.is_folder { "F" } else { "T" })
                .with_search_fields(vec![track.name.clone()])
                .with_actions(vec![
                    ItemAction::new("Select", format!("reaper:track-select:{}", track.index)),
                    ItemAction::new("Solo", format!("reaper:track-solo:{}", track.index))
                        .with_modifier(ActionModifier::Shift),
                    ItemAction::new("Mute", format!("reaper:track-mute:{}", track.index))
                        .with_modifier(ActionModifier::Ctrl),
                    ItemAction::new("Show FX", format!("reaper:track-fx:{}", track.index))
                        .with_modifier(ActionModifier::Alt),
                ])
        }).collect();
        Ok(items)
    }

    fn activate(&self, item: &Item, action: &str) -> Result<ActivationResult, Box<dyn std::error::Error + Send + Sync>> {
        tracing::info!(action = action, item = %item.label, "Reaper track action");
        Ok(ActivationResult::Close)
    }
}

fn demo_tracks() -> Vec<TrackEntry> {
    vec![
        TrackEntry { id: "trk-master".into(), name: "Master".into(), index: 0, is_folder: false, muted: false, soloed: false },
        TrackEntry { id: "trk-drums".into(), name: "Drums".into(), index: 1, is_folder: true, muted: false, soloed: false },
        TrackEntry { id: "trk-kick".into(), name: "Kick".into(), index: 2, is_folder: false, muted: false, soloed: false },
        TrackEntry { id: "trk-snare".into(), name: "Snare".into(), index: 3, is_folder: false, muted: false, soloed: false },
        TrackEntry { id: "trk-hihat".into(), name: "Hi-Hat".into(), index: 4, is_folder: false, muted: false, soloed: false },
        TrackEntry { id: "trk-bass".into(), name: "Bass".into(), index: 5, is_folder: false, muted: false, soloed: false },
        TrackEntry { id: "trk-guitars".into(), name: "Guitars".into(), index: 6, is_folder: true, muted: false, soloed: false },
        TrackEntry { id: "trk-gtr-l".into(), name: "Guitar L".into(), index: 7, is_folder: false, muted: false, soloed: false },
        TrackEntry { id: "trk-gtr-r".into(), name: "Guitar R".into(), index: 8, is_folder: false, muted: false, soloed: false },
        TrackEntry { id: "trk-vocals".into(), name: "Lead Vocals".into(), index: 9, is_folder: false, muted: false, soloed: false },
        TrackEntry { id: "trk-bgv".into(), name: "Background Vocals".into(), index: 10, is_folder: false, muted: true, soloed: false },
        TrackEntry { id: "trk-keys".into(), name: "Keys".into(), index: 11, is_folder: false, muted: false, soloed: false },
    ]
}
