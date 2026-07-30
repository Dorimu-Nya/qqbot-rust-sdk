#![cfg(feature = "events")]

use qqbot_rust_sdk::events::group::event_type::{GroupEventType, GroupEventTypeKind};
use qqbot_rust_sdk::events::guild::event_type::{GuildEventType, GuildEventTypeKind};
use qqbot_rust_sdk::events::interaction::event_type::{
    InteractionEventType, InteractionEventTypeKind,
};
use qqbot_rust_sdk::events::interaction::models::Interaction;
use strum::IntoEnumIterator;

#[test]
fn converts_unit_and_empty_tuple_variants_to_kinds() {
    let group = GroupEventType::SubscribeMessageStatus;
    assert_eq!(group.to_kind(), GroupEventTypeKind::SubscribeMessageStatus);
    assert_eq!(
        GroupEventTypeKind::from(group),
        GroupEventTypeKind::SubscribeMessageStatus
    );

    let guild = GuildEventType::PublicMessageDelete();
    assert_eq!(guild.to_kind(), GuildEventTypeKind::PublicMessageDelete);
    assert_eq!(
        GuildEventTypeKind::from(guild),
        GuildEventTypeKind::PublicMessageDelete
    );
}

#[test]
fn converts_payload_variants_to_kinds() {
    let event = InteractionEventType::InteractionCreate(Interaction {
        id: "interaction-id".to_string(),
        kind: None,
        scene: None,
        chat_type: None,
        timestamp: None,
        guild_id: None,
        channel_id: None,
        user_openid: None,
        group_openid: None,
        group_member_openid: None,
        data: None,
        version: None,
    });

    assert_eq!(
        InteractionEventTypeKind::from(&event),
        InteractionEventTypeKind::InteractionCreate
    );
    assert_eq!(
        InteractionEventTypeKind::from(event),
        InteractionEventTypeKind::InteractionCreate
    );
}

#[test]
fn generated_kind_enums_remain_iterable() {
    assert_eq!(GroupEventTypeKind::iter().count(), 6);
}
