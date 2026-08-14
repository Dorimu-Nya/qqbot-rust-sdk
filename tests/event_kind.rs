#![cfg(feature = "events")]

use qqbot_rust_sdk::events::c2c::models::C2cMessage;
use qqbot_rust_sdk::events::group::event::{GroupEvent, GroupEventKind};
use qqbot_rust_sdk::events::group::models::GroupMessage;
use qqbot_rust_sdk::events::guild::event::{GuildEvent, GuildEventKind};
use qqbot_rust_sdk::events::interaction::event::{InteractionEvent, InteractionEventKind};
use qqbot_rust_sdk::events::interaction::models::Interaction;
use qqbot_rust_sdk::events::markers::{
    c2c as c2c_markers, group as group_markers, guild as guild_markers,
    interaction as interaction_markers,
};
use qqbot_rust_sdk::events::EventSpec;
use std::any::TypeId;

fn assert_payload<E, P>()
where
    E: EventSpec<Payload = P>,
{
}

#[test]
fn markers_bind_the_expected_payload_types() {
    assert_payload::<c2c_markers::C2cMessageCreate, C2cMessage>();
    assert_payload::<interaction_markers::InteractionCreate, Interaction>();
    assert_payload::<group_markers::SubscribeMessageStatus, ()>();
    assert_payload::<guild_markers::PublicMessageDelete, ()>();
}

#[test]
fn markers_with_the_same_payload_still_have_distinct_type_ids() {
    assert_payload::<group_markers::GroupAtMessageCreate, GroupMessage>();
    assert_payload::<group_markers::GroupMessageCreate, GroupMessage>();

    assert_ne!(
        TypeId::of::<group_markers::GroupAtMessageCreate>(),
        TypeId::of::<group_markers::GroupMessageCreate>()
    );
}

#[test]
fn converts_unit_and_empty_tuple_variants_to_kinds() {
    let group = GroupEvent::SubscribeMessageStatus;
    assert_eq!(group.to_kind(), GroupEventKind::SubscribeMessageStatus);
    assert!(group.data().downcast_ref::<()>().is_some());
    assert_eq!(
        GroupEventKind::from(group),
        GroupEventKind::SubscribeMessageStatus
    );

    let guild = GuildEvent::PublicMessageDelete();
    assert_eq!(guild.to_kind(), GuildEventKind::PublicMessageDelete);
    assert!(guild.data().downcast_ref::<()>().is_some());
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

    let data = event.data().downcast_ref::<Interaction>().unwrap();
    assert_eq!(data.id, "interaction-id");

    assert_eq!(
        InteractionEventKind::from(&event),
        InteractionEventKind::InteractionCreate
    );
    assert_eq!(
        InteractionEventKind::from(event),
        InteractionEventKind::InteractionCreate
    );
}
