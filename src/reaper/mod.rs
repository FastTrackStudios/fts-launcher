//! Reaper-specific providers and tag definitions.
//!
//! This module defines everything that makes the launcher Reaper-aware:
//! - Tag hierarchy for Reaper concepts (actions, FX, tracks, etc.)
//! - Providers that source data from Reaper (actions list, FX database, tracks)
//! - Integration points for Reaper extensions

mod actions;
mod fx;
mod tags;
mod tracks;

pub use actions::ReaperActionsProvider;
pub use fx::ReaperFxProvider;
pub use tags::register_reaper_tags;
pub use tracks::ReaperTracksProvider;
