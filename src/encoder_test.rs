use crate::{
    decoder, encoder, Connack, Connect, ConnectReturnCode, Packet, PacketIdentifier, Protocol,
    Publish, QoS, Suback, Subscribe, SubscribeReturnCodes, SubscribeTopic, Unsubscribe,
};
use bytes::BytesMut;
#[test]
fn test_connect() {
    let packet = Connect {
        protocol: Protocol::new("MQTT", 4).unwrap(),
        keep_alive: 120,
        client_id: "imvj".to_string(),
        clean_session: true,
        last_will: None,
        username: None,
        password: None,
    };
    let mut buffer = BytesMut::with_capacity(1024);
    let _ = encoder::encode(&Packet::Connect(packet), &mut buffer);
    let decoded = decoder::decode(&mut buffer).unwrap();
    match decoded {
        Some(Packet::Connect(_c)) => {
            assert!(true);
        }
        _ => assert!(false),
    }
}

#[test]
fn test_connack() {
    let packet = Connack {
        session_present: true,
        code: ConnectReturnCode::Accepted,
    };
    let mut buffer = BytesMut::with_capacity(1024);
    let _ = encoder::encode(&Packet::Connack(packet), &mut buffer);
    let decoded = decoder::decode(&mut buffer).unwrap();
    match decoded {
        Some(Packet::Connack(_c)) => {
            assert!(true);
        }
        _ => assert!(false),
    }
}

#[test]
fn test_publish() {
    let packet = Publish {
        dup: false,
        qos: QoS::ExactlyOnce,
        retain: true,
        topic_name: "asdf".to_string(),
        pid: Some(PacketIdentifier(10)),
        payload: vec!['h' as u8, 'e' as u8, 'l' as u8, 'l' as u8, 'o' as u8],
    };
    let mut buffer = BytesMut::with_capacity(1024);
    let _ = encoder::encode(&Packet::Publish(packet), &mut buffer);
    let decoded = decoder::decode(&mut buffer).unwrap();
    match decoded {
        Some(Packet::Publish(_c)) => {
            assert!(true);
        }
        _ => assert!(false),
    }
}

#[test]
fn test_puback() {
    let packet = Packet::Puback(PacketIdentifier(19));
    let mut buffer = BytesMut::with_capacity(1024);
    let _ = encoder::encode(&packet, &mut buffer);
    let decoded = decoder::decode(&mut buffer).unwrap();
    match decoded {
        Some(Packet::Puback(_c)) => {
            assert!(true);
        }
        _ => assert!(false),
    }
}

#[test]
fn test_pubrec() {
    let packet = Packet::Pubrec(PacketIdentifier(19));
    let mut buffer = BytesMut::with_capacity(1024);
    let _ = encoder::encode(&packet, &mut buffer);
    let decoded = decoder::decode(&mut buffer).unwrap();
    match decoded {
        Some(Packet::Pubrec(_c)) => {
            assert!(true);
        }
        _ => assert!(false),
    }
}

#[test]
fn test_pubrel() {
    let packet = Packet::Pubrel(PacketIdentifier(19));
    let mut buffer = BytesMut::with_capacity(1024);
    let _ = encoder::encode(&packet, &mut buffer);
    let decoded = decoder::decode(&mut buffer).unwrap();
    match decoded {
        Some(Packet::Pubrel(_c)) => {
            assert!(true);
        }
        _ => assert!(false),
    }
}

#[test]
fn test_pubcomp() {
    let packet = Packet::PubComp(PacketIdentifier(19));
    let mut buffer = BytesMut::with_capacity(1024);
    let _ = encoder::encode(&packet, &mut buffer);
    let decoded = decoder::decode(&mut buffer).unwrap();
    match decoded {
        Some(Packet::PubComp(_c)) => {
            assert!(true);
        }
        _ => assert!(false),
    }
}

#[test]
fn test_subscribe() {
    let stopic = SubscribeTopic {
        topic_path: "a/b".to_string(),
        qos: QoS::ExactlyOnce,
    };
    let packet = Subscribe {
        pid: PacketIdentifier(345),
        topics: vec![stopic],
    };
    let mut buffer = BytesMut::with_capacity(1024);
    let _ = encoder::encode(&Packet::Subscribe(packet), &mut buffer);
    let decoded = decoder::decode(&mut buffer).unwrap();
    match decoded {
        Some(Packet::Subscribe(_c)) => {
            assert!(true);
        }
        _ => assert!(false),
    }
}

#[test]
fn test_suback() {
    let returnCode = SubscribeReturnCodes::Success(QoS::ExactlyOnce);
    let packet = Suback {
        pid: PacketIdentifier(12321),
        return_codes: vec![returnCode],
    };
    let mut buffer = BytesMut::with_capacity(1024);
    let _ = encoder::encode(&Packet::SubAck(packet), &mut buffer);
    let decoded = decoder::decode(&mut buffer).unwrap();
    match decoded {
        Some(Packet::SubAck(_c)) => {
            assert!(true);
        }
        _ => assert!(false),
    }
}

#[test]
fn test_unsubscribe() {
    let packet = Unsubscribe {
        pid: PacketIdentifier(12321),
        topics: vec!["a/b".to_string()],
    };
    let mut buffer = BytesMut::with_capacity(1024);
    let _ = encoder::encode(&Packet::UnSubscribe(packet), &mut buffer);
    let decoded = decoder::decode(&mut buffer).unwrap();
    match decoded {
        Some(Packet::UnSubscribe(_c)) => {
            assert!(true);
        }
        _ => assert!(false),
    }
}

#[test]
fn test_unsuback() {
    let packet = Packet::UnSubAck(PacketIdentifier(19));
    let mut buffer = BytesMut::with_capacity(1024);
    let _ = encoder::encode(&packet, &mut buffer);
    let decoded = decoder::decode(&mut buffer).unwrap();
    match decoded {
        Some(Packet::UnSubAck(_c)) => {
            assert!(true);
        }
        _ => assert!(false),
    }
}

#[test]
fn test_ping_req() {
    let mut buffer = BytesMut::with_capacity(1024);
    let _ = encoder::encode(&Packet::PingReq, &mut buffer);
    let decoded = decoder::decode(&mut buffer).unwrap();
    match decoded {
        Some(Packet::PingReq) => {
            assert!(true);
        }
        _ => assert!(false),
    }
}

#[test]
fn test_ping_resp() {
    let mut buffer = BytesMut::with_capacity(1024);
    let _ = encoder::encode(&Packet::PingResp, &mut buffer);
    let decoded = decoder::decode(&mut buffer).unwrap();
    match decoded {
        Some(Packet::PingResp) => {
            assert!(true);
        }
        _ => assert!(false),
    }
}

#[test]
fn test_disconnect() {
    let mut buffer = BytesMut::with_capacity(1024);
    let _ = encoder::encode(&Packet::Disconnect, &mut buffer);
    let decoded = decoder::decode(&mut buffer).unwrap();
    match decoded {
        Some(Packet::Disconnect) => {
            assert!(true);
        }
        _ => assert!(false),
    }
}