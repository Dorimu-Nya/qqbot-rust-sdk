#![cfg(feature = "events")]

use qqbot_rust_sdk::events::group::event::{GroupEvent, GroupEventKind};
use qqbot_rust_sdk::events::guild::event::{GuildEvent, GuildEventKind};
use qqbot_rust_sdk::events::interaction::event::{InteractionEvent, InteractionEventKind};
use qqbot_rust_sdk::events::interaction::models::Interaction;
use strum::IntoEnumIterator;

#[test]
fn converts_unit_and_empty_tuple_variants_to_kinds() {
    let group = GroupEvent::SubscribeMessageStatus;
    assert_eq!(group.to_kind(), GroupEventKind::SubscribeMessageStatus);
    assert_eq!(
        GroupEventKind::from(group),
        GroupEventKind::SubscribeMessageStatus
    );

    let guild = GuildEvent::PublicMessageDelete();
    assert_eq!(guild.to_kind(), GuildEventKind::PublicMessageDelete);
    assert_eq!(
        GuildEventKind::from(guild),
        GuildEventKind::PublicMessageDelete
    );
}

#[test]
fn converts_payload_variants_to_kinds() {
    let event = InteractionEvent::InteractionCreate(Interaction {
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
        InteractionEventKind::from(&event),
        InteractionEventKind::InteractionCreate
    );
    assert_eq!(
        InteractionEventKind::from(event),
        InteractionEventKind::InteractionCreate
    );
}

#[test]
fn generated_kind_enums_remain_iterable() {
    assert_eq!(GroupEventKind::iter().count(), 6);
}
