use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Debug, Clone, Copy, Serialize_repr, Deserialize_repr)]
#[repr(u32)]
pub enum ThreadFormat {
    FormatText = 1,
    FormatHtml = 2,
    FormatMarkdown = 3,
    FormatJson = 4,
}
