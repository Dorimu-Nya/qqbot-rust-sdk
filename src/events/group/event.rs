use super::models::{GroupAddRobotEvent, GroupDelRobotEvent, GroupMessage, GroupMsgRejectEvent};
use crate::events::event_kind;
use crate::events::payload::event::Event;
use crate::events::payload::payload::{DispatchPayload, FromDispatchPayload};
use serde::{Deserialize, Serialize};

event_kind!(
    /// 群事件
    #[derive(Debug, Clone, Serialize, Deserialize)]
    #[serde(tag = "t", content = "d")]
    pub enum GroupEvent {
        /// 群@机器人消息
        ///
        /// 用户在群里@机器人发送消息时触发。这是机器人最常接收的事件。 content 字段已自动去除@机器人的前缀。
        #[serde(rename = "GROUP_AT_MESSAGE_CREATE")]
        GroupAtMessageCreate(GroupMessage),
        /// 群消息（全量模式）
        ///
        /// 当机器人开启了"接收所有消息"功能后，群里的每一条消息（不限于@机器人）都会推送此事件。各字段含义与 GROUP_AT_MESSAGE_CREATE 完全一致。
        #[serde(rename = "GROUP_MESSAGE_CREATE")]
        GroupMessageCreate(GroupMessage),
        /// 群添加机器人
        #[serde(rename = "GROUP_AT_ROBOT")]
        GroupAddRobot(GroupAddRobotEvent),
        /// 群移除机器人
        #[serde(rename = "GROUP_DEL_ROBOT")]
        GroupDelRobot(GroupDelRobotEvent),
        /// 群开启消息推送
        #[serde(rename = "GROUP_MSG_RECEIVE")]
        GroupMsgReceive(GroupAddRobotEvent),
        /// 群关闭消息推送
        #[serde(rename = "GROUP_MSG_REJECT")]
        GroupMsgReject(GroupMsgRejectEvent),
        /// 消息授权状态变更
        #[serde(rename = "SUBSCRIBE_MESSAGE_STATUS")]
        SubscribeMessageStatus,
    }
);

impl FromDispatchPayload for GroupMessage {
    fn from(payload: &DispatchPayload) -> Option<Self> {
        match &payload.event {
            Event::GroupEvent(GroupEvent::GroupAtMessageCreate(value)) => Some(value.clone()),
            _ => None,
        }
    }
}

impl FromDispatchPayload for GroupAddRobotEvent {
    fn from(payload: &DispatchPayload) -> Option<Self> {
        match &payload.event {
            Event::GroupEvent(GroupEvent::GroupAddRobot(value))
            | Event::GroupEvent(GroupEvent::GroupMsgReceive(value)) => Some(value.clone()),
            _ => None,
        }
    }
}

impl FromDispatchPayload for GroupDelRobotEvent {
    fn from(payload: &DispatchPayload) -> Option<Self> {
        match &payload.event {
            Event::GroupEvent(GroupEvent::GroupDelRobot(value)) => Some(value.clone()),
            _ => None,
        }
    }
}

impl FromDispatchPayload for GroupMsgRejectEvent {
    fn from(payload: &DispatchPayload) -> Option<Self> {
        match &payload.event {
            Event::GroupEvent(GroupEvent::GroupMsgReject(value)) => Some(value.clone()),
            _ => None,
        }
    }
}
