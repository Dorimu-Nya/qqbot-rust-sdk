use std::any::Any;

/// 描述一个具体事件变体及其对应的载荷类型。
pub trait EventSpec: Copy + Send + Sync + 'static {
    /// 当前事件变体携带的载荷类型。
    type Payload: Any + Clone + Send + Sync + 'static;
}
