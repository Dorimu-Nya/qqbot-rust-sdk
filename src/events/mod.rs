mod macros;
mod typed;

pub(crate) use macros::event_kind;
pub use typed::EventSpec;

pub mod c2c;
pub mod common;
pub mod group;
pub mod guild;
pub mod interaction;
pub mod markers;
pub mod message_reaction;
pub mod payload;
pub mod validation;
