#![cfg(feature = "events")]

use qqbot_rust_sdk::events::group::models::GroupMessage;
use serde_json::json;

#[test]
fn serializes_and_deserializes_group_message() {
    let input = json!({
        "id": "ROBOT1.0_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",
        "author": {
            "id": "A1B2C3D4E5F6A1B2C3D4E5F6A1B2C3D4",
            "member_openid": "A1B2C3D4E5F6A1B2C3D4E5F6A1B2C3D4",
            "member_role": "member",
            "username": "小明",
            "bot": false
        },
        "content": "大家早上好呀",
        "group_openid": "B2C3D4E5F6A1B2C3D4E5F6A1B2C3D4E5",
        "message_type": 0,
        "timestamp": "2026-07-21T08:00:00+08:00",
        "message_scene": {
            "source": "default",
            "ext": [
                "msg_idx=REFIDX_xxxxxxxxxxxxxxx==",
                "auth_token=xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx"
            ]
        }
    });

    let message: GroupMessage = serde_json::from_value(input).unwrap();

    assert_eq!(
        message.id,
        "ROBOT1.0_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx"
    );
    assert_eq!(
        message.author.user.id.as_deref(),
        Some("A1B2C3D4E5F6A1B2C3D4E5F6A1B2C3D4")
    );
    assert_eq!(
        message.author.member_openid,
        "A1B2C3D4E5F6A1B2C3D4E5F6A1B2C3D4"
    );
    assert_eq!(message.author.member_role, "member");
    assert_eq!(message.author.user.username, "小明");
    assert!(!message.author.user.bot);
    assert_eq!(message.content.as_deref(), Some("大家早上好呀"));
    assert_eq!(message.group_openid, "B2C3D4E5F6A1B2C3D4E5F6A1B2C3D4E5");
    assert_eq!(message.message_type, 0);
    assert_eq!(
        message.timestamp.as_deref(),
        Some("2026-07-21T08:00:00+08:00")
    );
    assert_eq!(message.message_scene.source.as_deref(), Some("default"));
    assert_eq!(
        message.message_scene.ext.as_deref(),
        Some(
            [
                "msg_idx=REFIDX_xxxxxxxxxxxxxxx==".to_string(),
                "auth_token=xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx".to_string(),
            ]
            .as_slice()
        )
    );
    assert!(message.attachments.is_none());
    assert!(message.mentions.is_none());
    assert!(message.ark_data.is_none());
    assert!(message.msg_elements.is_none());

    let serialized = serde_json::to_value(&message).unwrap();

    assert_eq!(serialized["id"], json!(message.id));
    assert_eq!(serialized["author"]["username"], json!("小明"));
    assert_eq!(serialized["content"], json!("大家早上好呀"));
    assert_eq!(
        serialized["message_scene"],
        json!({
            "source": "default",
            "ext": [
                "msg_idx=REFIDX_xxxxxxxxxxxxxxx==",
                "auth_token=xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx"
            ]
        })
    );
}

#[test]
fn serializes_and_deserializes_group_message_with_attachment() {
    let input = json!({
        "id": "ROBOT1.0_yyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyy",
        "author": {
            "id": "C3D4E5F6A1B2C3D4E5F6A1B2C3D4E5F6",
            "member_openid": "C3D4E5F6A1B2C3D4E5F6A1B2C3D4E5F6",
            "member_role": "owner",
            "username": "小红",
            "bot": false
        },
        "content": "分享一张今天的风景照",
        "group_openid": "B2C3D4E5F6A1B2C3D4E5F6A1B2C3D4E5",
        "message_type": 0,
        "timestamp": "2026-07-21T09:30:00+08:00",
        "attachments": [
            {
                "content_type": "image/jpeg",
                "filename": "photo.jpg",
                "url": "https://multimedia.nt.qq.com.cn/download?appid=xxx&fileid=xxx&rkey=xxx&spec=0",
                "width": 1920,
                "height": 1080,
                "size": 256000
            }
        ],
        "message_scene": {
            "source": "default",
            "ext": [
                "msg_idx=REFIDX_yyyyyyyyyyyyyyy==",
                "auth_token=yyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyy"
            ]
        }
    });

    let message: GroupMessage = serde_json::from_value(input).unwrap();
    let attachment = &message.attachments.as_ref().unwrap()[0];

    assert_eq!(message.author.member_role, "owner");
    assert_eq!(message.author.user.username, "小红");
    assert_eq!(attachment.content_type.as_deref(), Some("image/jpeg"));
    assert_eq!(attachment.filename.as_deref(), Some("photo.jpg"));
    assert_eq!(attachment.width, Some(1920));
    assert_eq!(attachment.height, Some(1080));
    assert_eq!(attachment.size, Some(256000));
    assert_eq!(
        attachment.url.as_deref(),
        Some("https://multimedia.nt.qq.com.cn/download?appid=xxx&fileid=xxx&rkey=xxx&spec=0")
    );

    let serialized = serde_json::to_value(&message).unwrap();

    assert_eq!(serialized["attachments"][0]["filename"], json!("photo.jpg"));
    assert_eq!(serialized["attachments"][0]["width"], json!(1920));
    assert_eq!(serialized["attachments"][0]["height"], json!(1080));
}

#[test]
fn serializes_and_deserializes_group_message_with_message_elements() {
    let content = "=== 消息 1 ===\n[消息内容] 今天的学习计划已完成\n\n=== 消息 2 ===\n[消息内容] 很棒！继续保持，明天继续加油\n\n=== 消息 3 ===\n[消息内容] 好的，一起进步！";
    let input = json!({
        "id": "ROBOT1.0_zzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzz",
        "author": {
            "id": "D4E5F6A1B2C3D4E5F6A1B2C3D4E5F6A1",
            "member_openid": "D4E5F6A1B2C3D4E5F6A1B2C3D4E5F6A1",
            "member_role": "admin",
            "username": "小华",
            "bot": false
        },
        "content": " ",
        "group_openid": "B2C3D4E5F6A1B2C3D4E5F6A1B2C3D4E5",
        "message_type": 103,
        "timestamp": "2026-07-21T10:10:00+08:00",
        "msg_elements": [
            {
                "content": content
            }
        ],
        "message_scene": {
            "source": "default",
            "ext": [
                "msg_idx=REFIDX_zzzzzzzzzzzzzzz==",
                "auth_token=zzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzz",
                "ref_msg_idx=TMP_xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx"
            ]
        }
    });

    let message: GroupMessage = serde_json::from_value(input).unwrap();
    let elements = message.msg_elements.as_ref().unwrap();

    assert_eq!(message.author.member_role, "admin");
    assert_eq!(message.message_type, 103);
    assert_eq!(elements.len(), 1);
    assert_eq!(elements[0].content.as_deref(), Some(content));
    assert!(elements[0].attachments.is_none());
    assert!(elements[0].msg_elements.is_none());

    let serialized = serde_json::to_value(&message).unwrap();

    assert_eq!(serialized["msg_elements"][0]["content"], json!(content));
    assert_eq!(
        serialized["message_scene"]["ext"][2],
        json!("ref_msg_idx=TMP_xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx")
    );
}
