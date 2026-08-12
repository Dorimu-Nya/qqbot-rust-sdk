#![cfg(feature = "events")]

use qqbot_rust_sdk::events::group::models::{GroupMention, GroupMessage};
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

#[test]
fn serializes_and_deserializes_group_message_with_all_mention() {
    let input = json!({
        "op": 0,
        "id": "GROUP_MESSAGE_CREATE:2eys9hem80qmfgws0txqocr3vy6qyjyyelxar88su1xqkgxwpynhysl6q6ws849",
        "d": {
            "id": "ROBOT1.0_2eys9HEM80qMFgWs0Tx.QNuTBavm.9nqbRhirAI4jntPgYH8j7b1GhDEd6gGr61-OB1uvyOI1fGyQCJXOh5nfFNq7PfpPFCZNAhZuJqZ1CDPlNcCu0N7N5SmoOzGYJpn",
            "content": "<@all> ",
            "timestamp": "2026-08-12T10:06:32+08:00",
            "author": {
                "id": "8612C390E5716C0608A7A4E526C1EA45",
                "username": "✪sheip9",
                "bot": false,
                "member_openid": "8612C390E5716C0608A7A4E526C1EA45",
                "member_role": "owner",
                "union_openid": ""
            },
            "mentions": [{
                "username": "全体成员",
                "scope": "all",
                "is_you": true
            }],
            "group_id": "5AEE0B71A81D3889E610D3273453F4D3",
            "group_openid": "5AEE0B71A81D3889E610D3273453F4D3",
            "message_scene": {
                "source": "default",
                "ext": [
                    "msg_idx=REFIDX_0Vhe0GMkmh0un8qH50kD+MtG81ovPjw88HwjHppK6Gc=",
                    "auth_token=30i6ggAf91e7aHXthPX-FLale4DzRgf12Wh5DKLdhbND1MaI9RMtV1xvS6fYFhENU3JwwET9p7HUSNFzDQzIIDZngvd5irC5f-q3AgOfn8RPPCS3jYlcX8CrFregJoQ"
                ]
            },
            "message_type": 0
        },
        "t": "GROUP_MESSAGE_CREATE"
    });

    let message: GroupMessage = serde_json::from_value(input["d"].clone()).unwrap();
    let mentions = message.mentions.as_ref().unwrap();

    assert_eq!(message.content.as_deref(), Some("<@all> "));
    assert_eq!(message.author.user.username, "✪sheip9");
    assert_eq!(message.author.user.union_openid.as_deref(), Some(""));
    assert_eq!(message.author.member_role, "owner");
    assert_eq!(message.group_openid, "5AEE0B71A81D3889E610D3273453F4D3");
    assert_eq!(mentions.len(), 1);
    assert!(matches!(&mentions[0], GroupMention::All(_)));

    let serialized = serde_json::to_value(&message).unwrap();

    assert_eq!(serialized["mentions"][0]["username"], json!("全体成员"));
    assert_eq!(serialized["mentions"][0]["scope"], json!("all"));
    assert_eq!(serialized["mentions"][0]["is_you"], json!(true));
    assert_eq!(
        serialized["message_scene"]["ext"],
        input["d"]["message_scene"]["ext"]
    );
}

#[test]
fn serializes_and_deserializes_group_message_with_single_mentions() {
    let input = json!({
        "op": 0,
        "id": "GROUP_MESSAGE_CREATE:2eys9hem80qmfgws0txqoxiicynozkxo8whalrhg8rxqkgxwpynhysl6q6ws849",
        "d": {
            "id": "ROBOT1.0_2eys9HEM80qMFgWs0Tx.QPp1hD2DOYK6QCkzpojmH4DBptJEMom4NiSFT2-bbTQdL264SMAftX2KdJLQfWRSBMUKS6CQO8P05TVQoQ3a5PxUiF3PQ7ZIcqjqQuz3oPNR",
            "content": "<@D67BAF0591F086CEBF21403C3404BCAA> <@8612C390E5716C0608A7A4E526C1EA45> ",
            "timestamp": "2026-08-12T10:07:30+08:00",
            "author": {
                "id": "8612C390E5716C0608A7A4E526C1EA45",
                "username": "✪sheip9",
                "bot": false,
                "member_openid": "8612C390E5716C0608A7A4E526C1EA45",
                "member_role": "owner",
                "union_openid": ""
            },
            "mentions": [{
                "id": "D67BAF0591F086CEBF21403C3404BCAA",
                "username": "爱丽丝",
                "bot": true,
                "member_openid": "D67BAF0591F086CEBF21403C3404BCAA",
                "scope": "single",
                "is_you": true,
                "member_role": "member"
            }, {
                "id": "8612C390E5716C0608A7A4E526C1EA45",
                "username": "✪sheip9",
                "bot": false,
                "member_openid": "8612C390E5716C0608A7A4E526C1EA45",
                "scope": "single",
                "is_you": false,
                "member_role": "owner"
            }],
            "group_id": "5AEE0B71A81D3889E610D3273453F4D3",
            "group_openid": "5AEE0B71A81D3889E610D3273453F4D3",
            "message_scene": {
                "source": "default",
                "ext": [
                    "msg_idx=REFIDX_pIUDcCxPHNmnX1pvgJLxRstG81ovPjw88HwjHppK6Gc=",
                    "auth_token=RUqHU4mI4rmFk8FEn0RQKveD8OHmG3bIxnNuMToC87sEoVvSZwjbBcR43OrkCLMsd7RlftI8oFDtrF6eB_gV5mD_nzRkEee8fIRacfpnf7Rt2pTN8jICKZyYdSs-4Wg"
                ]
            },
            "message_type": 0
        },
        "t": "GROUP_MESSAGE_CREATE"
    });

    let message: GroupMessage = serde_json::from_value(input["d"].clone()).unwrap();
    let mentions = message.mentions.as_ref().unwrap();

    assert_eq!(mentions.len(), 2);
    assert!(mentions
        .iter()
        .all(|mention| matches!(mention, GroupMention::Single(_))));

    let serialized = serde_json::to_value(&message).unwrap();

    assert_eq!(
        serialized["mentions"][0]["id"],
        json!("D67BAF0591F086CEBF21403C3404BCAA")
    );
    assert_eq!(serialized["mentions"][0]["username"], json!("爱丽丝"));
    assert_eq!(serialized["mentions"][0]["bot"], json!(true));
    assert_eq!(serialized["mentions"][0]["scope"], json!("single"));
    assert_eq!(serialized["mentions"][0]["is_you"], json!(true));
    assert_eq!(serialized["mentions"][0]["member_role"], json!("member"));
    assert_eq!(serialized["mentions"][1]["username"], json!("✪sheip9"));
    assert_eq!(serialized["mentions"][1]["scope"], json!("single"));
    assert_eq!(serialized["mentions"][1]["is_you"], json!(false));
    assert_eq!(serialized["mentions"][1]["member_role"], json!("owner"));
    assert_eq!(
        serialized["message_scene"]["ext"],
        input["d"]["message_scene"]["ext"]
    );
}

#[test]
fn serializes_and_deserializes_group_message_without_mentions() {
    let input = json!({
        "op": 0,
        "id": "GROUP_MESSAGE_CREATE:2eys9hem80qmfgws0txqdx32ldikzcio259rdfmu2vxqkgxwpynhysl6q6ws849",
        "d": {
            "id": "ROBOT1.0_2eys9HEM80qMFgWs0Tx.QE1NoLCpefXetGGzsNccJ2Od6d8VU.Yqkr.6s2WdDOCbzUFmmVoNnv508NkV-gyyryIHEKTEZVh6Pp0CbJjRxhw!",
            "content": " ",
            "timestamp": "2026-08-12T10:08:52+08:00",
            "author": {
                "id": "8612C390E5716C0608A7A4E526C1EA45",
                "username": "✪sheip9",
                "bot": false,
                "member_openid": "8612C390E5716C0608A7A4E526C1EA45",
                "member_role": "owner",
                "union_openid": ""
            },
            "group_id": "5AEE0B71A81D3889E610D3273453F4D3",
            "group_openid": "5AEE0B71A81D3889E610D3273453F4D3",
            "message_scene": {
                "source": "default",
                "ext": [
                    "msg_idx=REFIDX_61b92/z0j2u9swxM/4p3PstG81ovPjw88HwjHppK6Gc=",
                    "auth_token=xr9dDc-s5nUDPWUjxJGTk_3dS_uaKez3YlAG3cVOjfcmlMq3qPuhQ6oX9fklQSyrNCjruieVNQUry7cUqDXuDJSohX2Ip2Ywdh9PIA30Q5SwXqaL2O8K2r9aMTwdaFc"
                ]
            },
            "message_type": 0
        },
        "t": "GROUP_MESSAGE_CREATE"
    });

    let message: GroupMessage = serde_json::from_value(input["d"].clone()).unwrap();

    assert_eq!(
        message.id,
        "ROBOT1.0_2eys9HEM80qMFgWs0Tx.QE1NoLCpefXetGGzsNccJ2Od6d8VU.Yqkr.6s2WdDOCbzUFmmVoNnv508NkV-gyyryIHEKTEZVh6Pp0CbJjRxhw!"
    );
    assert_eq!(message.content.as_deref(), Some(" "));
    assert_eq!(
        message.timestamp.as_deref(),
        Some("2026-08-12T10:08:52+08:00")
    );
    assert_eq!(message.group_openid, "5AEE0B71A81D3889E610D3273453F4D3");
    assert_eq!(message.message_type, 0);
    assert!(message.mentions.is_none());

    let serialized = serde_json::to_value(&message).unwrap();

    assert_eq!(serialized["content"], json!(" "));
    assert_eq!(serialized["mentions"], serde_json::Value::Null);
    assert_eq!(
        serialized["message_scene"]["ext"],
        input["d"]["message_scene"]["ext"]
    );
}
