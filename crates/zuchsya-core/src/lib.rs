//! Zuchsya Core - Common types and utilities
//!
//! This crate contains the core types shared across the Zuchsya project:
//! - Game state
//! - Beatmap format and types
//! - Hit objects (Note, HoldNote)
//! - Timing points
//! - Scoring/Judgement types

pub mod beatmap;
pub mod hit_object;
pub mod scoring;
pub mod scroll_velocity;
pub mod state;
pub mod timing;

pub use beatmap::*;
pub use hit_object::*;
pub use scoring::*;
pub use scroll_velocity::*;
pub use state::*;
pub use timing::*;
