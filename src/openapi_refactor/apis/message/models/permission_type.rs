use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum PermissionType {
    SpecifyUser = 0,
    AdminOnly = 1,
    Everyone = 2,
    SpecifyRole = 3,
}
