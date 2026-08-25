use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Debug, Clone, Copy, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum MessageType {
    Text = 0,
    Markdown = 2,
    Ark = 3,
    Embed = 4,
    Media = 7,
}
