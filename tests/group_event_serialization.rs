#![cfg(feature = "events")]

use qqbot_rust_sdk::events::group::event::GroupEvent;
use qqbot_rust_sdk::events::payload::event::Event;
use qqbot_rust_sdk::events::payload::payload::DispatchPayload;

#[test]
fn deserializes_group_add_robot_dispatch_payload() {
    let json = r#"{
        "op": 0,
        "id": "GROUP_ADD_ROBOT:0a018cfd-e5f1-4179-83f6-e5cb88837b4c",
        "d": {
            "timestamp": 1786417565,
            "group_openid": "94C6ADF2932C56B0DAE1706706AB9538",
            "op_member_openid": "D2901E9E6CB413A66568F732895F6903"
        },
        "t": "GROUP_ADD_ROBOT"
    }"#;

    let payload: DispatchPayload = serde_json::from_str(json).unwrap();

    match payload.event {
        Event::GroupEvent(GroupEvent::GroupAddRobot(event)) => {
            assert_eq!(event.timestamp, 1_786_417_565);
            assert_eq!(event.group_openid, "94C6ADF2932C56B0DAE1706706AB9538");
            assert_eq!(event.op_member_openid, "D2901E9E6CB413A66568F732895F6903");
        }
        event => panic!("unexpected event: {event:?}"),
    }
}

#[test]
fn serializes_group_add_robot_with_platform_event_name() {
    let event =
        GroupEvent::GroupAddRobot(qqbot_rust_sdk::events::group::models::GroupAddRobotEvent {
            timestamp: 1_786_417_565,
            group_openid: "94C6ADF2932C56B0DAE1706706AB9538".to_string(),
            op_member_openid: "D2901E9E6CB413A66568F732895F6903".to_string(),
        });

    let json = serde_json::to_value(event).unwrap();

    assert_eq!(json["t"], "GROUP_ADD_ROBOT");
}
