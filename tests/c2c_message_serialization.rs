#![cfg(feature = "events")]

use qqbot_rust_sdk::events::c2c::models::C2cMessage;
use serde_json::json;

#[test]
fn serializes_and_deserializes_text_c2c_message() {
    let input = json!({
        "id": "ROBOT1.0_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",
        "author": {
            "id": "A1B2C3D4E5F6A1B2C3D4E5F6A1B2C3D4",
            "user_openid": "A1B2C3D4E5F6A1B2C3D4E5F6A1B2C3D4",
            "union_openid": "",
            "username": "",
            "bot": false
        },
        "content": "你好，今天有什么推荐的活动吗？",
        "message_type": 0,
        "message_scene": {
            "source": "default",
            "ext": [
                "msg_idx=REFIDX_xxxxxxxxxxxxxxx=="
            ]
        },
        "timestamp": "2026-07-21T10:00:00+08:00"
    });

    let message: C2cMessage = serde_json::from_value(input).unwrap();

    assert_eq!(
        message.id,
        "ROBOT1.0_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx"
    );
    assert_eq!(
        message.author.user.id.as_deref(),
        Some("A1B2C3D4E5F6A1B2C3D4E5F6A1B2C3D4")
    );
    assert_eq!(
        message.author.user_openid,
        "A1B2C3D4E5F6A1B2C3D4E5F6A1B2C3D4"
    );
    assert_eq!(message.author.user.union_openid.as_deref(), Some(""));
    assert_eq!(
        message.content.as_deref(),
        Some("你好，今天有什么推荐的活动吗？")
    );
    assert_eq!(message.message_type, Some(0));
    assert_eq!(
        message.timestamp.as_deref(),
        Some("2026-07-21T10:00:00+08:00")
    );
    assert!(message.attachments.is_none());
    assert!(message.ark_data.is_none());
    assert!(message.msg_elements.is_none());

    let serialized = serde_json::to_value(&message).unwrap();

    assert_eq!(
        serialized["author"]["user_openid"],
        json!(message.author.user_openid)
    );
    assert_eq!(
        serialized["content"],
        json!("你好，今天有什么推荐的活动吗？")
    );
    assert_eq!(
        serialized["message_scene"],
        json!({
            "source": "default",
            "ext": ["msg_idx=REFIDX_xxxxxxxxxxxxxxx=="]
        })
    );
}

#[test]
fn serializes_and_deserializes_ark_c2c_message() {
    let input = json!({
        "id": "ROBOT1.0_yyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyy",
        "author": {
            "id": "B2C3D4E5F6A1B2C3D4E5F6A1B2C3D4E5",
            "user_openid": "B2C3D4E5F6A1B2C3D4E5F6A1B2C3D4E5",
            "union_openid": "B2C3D4E5F6A1B2C3D4E5F6A1B2C3D4E5",
            "username": "",
            "bot": false
        },
        "content": "[卡片消息] 小程序\n摘要: [每日打卡]快来完成今日学习打卡",
        "message_type": 3,
        "ark_data": {
            "ark_type": "miniapp",
            "ark_name": "小程序",
            "prompt": "[每日打卡]快来完成今日学习打卡",
            "fields": {
                "title": "快来完成今日学习打卡",
                "source": "学习助手",
                "tag": "微信小程序",
                "preview": "https://pubminishare-30161.picsz.qpic.cn/preview_a1b2c3d4",
                "source_logo": "https://miniapp.gtimg.cn/generated-icon/app_a1b2c3d4.png",
                "tag_icon": "https://miniapp.gtimg.cn/public/miniwx.png"
            }
        },
        "message_scene": {
            "source": "default",
            "ext": [
                "msg_idx=REFIDX_yyyyyyyyyyyyyyy=="
            ]
        },
        "timestamp": "2026-07-21T10:01:00+08:00"
    });

    let message: C2cMessage = serde_json::from_value(input).unwrap();
    let ark_data = message.ark_data.as_ref().unwrap();

    assert_eq!(message.message_type, Some(3));
    assert_eq!(ark_data.ark_type, "miniapp");
    assert_eq!(ark_data.ark_name, "小程序");
    assert_eq!(ark_data.prompt, "[每日打卡]快来完成今日学习打卡");
    assert_eq!(
        ark_data.fields.get("source").map(String::as_str),
        Some("学习助手")
    );
    assert_eq!(
        ark_data.fields.get("tag_icon").map(String::as_str),
        Some("https://miniapp.gtimg.cn/public/miniwx.png")
    );

    let serialized = serde_json::to_value(&message).unwrap();

    assert_eq!(serialized["ark_data"]["ark_type"], json!("miniapp"));
    assert_eq!(
        serialized["ark_data"]["fields"]["title"],
        json!("快来完成今日学习打卡")
    );
    assert_eq!(serialized["ark_data"]["fields"]["tag"], json!("微信小程序"));
}

#[test]
fn serializes_and_deserializes_referenced_c2c_message() {
    let referenced_content = "每天坚持阅读半小时，一个月后你会发现自己的变化";
    let input = json!({
        "id": "ROBOT1.0_zzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzz",
        "author": {
            "id": "C3D4E5F6A1B2C3D4E5F6A1B2C3D4E5F6",
            "user_openid": "C3D4E5F6A1B2C3D4E5F6A1B2C3D4E5F6",
            "union_openid": "",
            "username": "",
            "bot": false
        },
        "content": "这个建议很有帮助，谢谢你！",
        "message_type": 103,
        "msg_elements": [
            {
                "msg_idx": "REFIDX_aaaaaaaaaaaaaaa==",
                "message_type": 103,
                "content": referenced_content
            }
        ],
        "message_scene": {
            "source": "default",
            "ext": [
                "ref_msg_idx=REFIDX_aaaaaaaaaaaaaaa==",
                "msg_idx=REFIDX_zzzzzzzzzzzzzzz=="
            ]
        },
        "timestamp": "2026-07-21T10:02:00+08:00"
    });

    let message: C2cMessage = serde_json::from_value(input).unwrap();
    let elements = message.msg_elements.as_ref().unwrap();

    assert_eq!(message.message_type, Some(103));
    assert_eq!(elements.len(), 1);
    assert_eq!(
        elements[0].msg_idx.as_deref(),
        Some("REFIDX_aaaaaaaaaaaaaaa==")
    );
    assert_eq!(elements[0].message_type, Some(103));
    assert_eq!(elements[0].content.as_deref(), Some(referenced_content));
    assert!(elements[0].attachments.is_none());
    assert!(elements[0].msg_elements.is_none());

    let serialized = serde_json::to_value(&message).unwrap();

    assert_eq!(
        serialized["msg_elements"][0]["content"],
        json!(referenced_content)
    );
    assert_eq!(
        serialized["message_scene"]["ext"],
        json!([
            "ref_msg_idx=REFIDX_aaaaaaaaaaaaaaa==",
            "msg_idx=REFIDX_zzzzzzzzzzzzzzz=="
        ])
    );
}
