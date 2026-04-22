use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Header {
    pub message_type: u8,
    pub payload_length: u32,
    pub timestamp_ns: u64,
    pub message_id: [u8; 16],
    pub emotion_tag: u8,
    pub emotion_score: u8,
    pub intent: u8,
}

#[derive(Debug)]
pub enum TranslateError {
    InvalidLength,
    Utf8Error,
}

impl fmt::Display for TranslateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TranslateError::InvalidLength => write!(f, "invalid length"),
            TranslateError::Utf8Error => write!(f, "utf8 decode error"),
        }
    }
}

pub struct Frame {
    pub header: Header,
    pub payload: Vec<u8>,
}

pub const TOKEN_LIVE_STATE_DELTA: u8 = 0x0b;
pub const FRAME_HEADER_LEN: usize = 4;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LiveStateDelta {
    pub state_id: u16,
    pub delta_seq: u16,
    pub intent: Option<u8>,
    pub confidence: Option<u8>,
    pub urgency: Option<u8>,
    pub risk: Option<u8>,
    pub affect_tag: Option<u8>,
    pub affect_score: Option<u8>,
    pub contradiction_pressure: Option<u8>,
    pub novelty: Option<u8>,
    pub trust_delta: Option<i8>,
    pub repair_request: Option<u8>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PayloadComparison {
    pub text_bytes: usize,
    pub json_bytes: usize,
    pub thp_tcp_bytes: usize,
}

pub fn encode_header(header: &Header) -> Vec<u8> {
    let mut out = Vec::with_capacity(1 + 4 + 8 + 16 + 1 + 1 + 1);
    out.push(header.message_type);
    out.extend_from_slice(&header.payload_length.to_be_bytes());
    out.extend_from_slice(&header.timestamp_ns.to_be_bytes());
    out.extend_from_slice(&header.message_id);
    out.push(header.emotion_tag);
    out.push(header.emotion_score);
    out.push(header.intent);
    out
}

pub fn encode_phase0_frame(token: u8, flags: u8, payload: &[u8]) -> Result<Vec<u8>, TranslateError> {
    if payload.len() > u16::MAX as usize {
        return Err(TranslateError::InvalidLength);
    }
    let mut out = Vec::with_capacity(FRAME_HEADER_LEN + payload.len());
    out.push(token);
    out.push(flags);
    out.extend_from_slice(&(payload.len() as u16).to_be_bytes());
    out.extend_from_slice(payload);
    Ok(out)
}

pub fn encode_live_state_delta(delta: &LiveStateDelta) -> Vec<u8> {
    let mut presence: u16 = 0;
    let mut values = Vec::with_capacity(10);
    push_opt_u8(&mut presence, &mut values, 0, delta.intent);
    push_opt_u8(&mut presence, &mut values, 1, delta.confidence);
    push_opt_u8(&mut presence, &mut values, 2, delta.urgency);
    push_opt_u8(&mut presence, &mut values, 3, delta.risk);
    push_opt_u8(&mut presence, &mut values, 4, delta.affect_tag);
    push_opt_u8(&mut presence, &mut values, 5, delta.affect_score);
    push_opt_u8(&mut presence, &mut values, 6, delta.contradiction_pressure);
    push_opt_u8(&mut presence, &mut values, 7, delta.novelty);
    if let Some(v) = delta.trust_delta {
        presence |= 1 << 8;
        values.push(v as u8);
    }
    push_opt_u8(&mut presence, &mut values, 9, delta.repair_request);

    let mut out = Vec::with_capacity(6 + values.len());
    out.extend_from_slice(&delta.state_id.to_be_bytes());
    out.extend_from_slice(&delta.delta_seq.to_be_bytes());
    out.extend_from_slice(&presence.to_be_bytes());
    out.extend_from_slice(&values);
    out
}

pub fn encode_live_state_delta_frame(delta: &LiveStateDelta) -> Vec<u8> {
    let payload = encode_live_state_delta(delta);
    encode_phase0_frame(TOKEN_LIVE_STATE_DELTA, 0, &payload)
        .expect("live state delta payload fits in u16")
}

pub fn compare_payloads(text: &str, json: &str, delta: &LiveStateDelta) -> PayloadComparison {
    PayloadComparison {
        text_bytes: text.as_bytes().len(),
        json_bytes: json.as_bytes().len(),
        thp_tcp_bytes: encode_live_state_delta_frame(delta).len(),
    }
}

pub fn decode_header(bytes: &[u8]) -> Result<Header, TranslateError> {
    if bytes.len() < 32 {
        return Err(TranslateError::InvalidLength);
    }
    let message_type = bytes[0];
    let payload_length = u32::from_be_bytes([bytes[1], bytes[2], bytes[3], bytes[4]]);
    let timestamp_ns = u64::from_be_bytes([
        bytes[5], bytes[6], bytes[7], bytes[8], bytes[9], bytes[10], bytes[11], bytes[12],
    ]);
    let mut message_id = [0u8; 16];
    message_id.copy_from_slice(&bytes[13..29]);
    let emotion_tag = bytes[29];
    let emotion_score = bytes[30];
    let intent = bytes[31];
    Ok(Header {
        message_type,
        payload_length,
        timestamp_ns,
        message_id,
        emotion_tag,
        emotion_score,
        intent,
    })
}

pub fn english_to_frame(text: &str) -> Frame {
    let payload = text.as_bytes().to_vec();
    let header = Header {
        message_type: 0x20,
        payload_length: payload.len() as u32,
        timestamp_ns: 0,
        message_id: [0u8; 16],
        emotion_tag: 0,
        emotion_score: 0,
        intent: 0,
    };
    Frame { header, payload }
}

pub fn frame_to_english(frame: &Frame) -> Result<String, TranslateError> {
    String::from_utf8(frame.payload.clone()).map_err(|_| TranslateError::Utf8Error)
}

fn push_opt_u8(presence: &mut u16, values: &mut Vec<u8>, bit: u8, value: Option<u8>) {
    if let Some(v) = value {
        *presence |= 1 << bit;
        values.push(v);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn header_roundtrip() {
        let h = Header {
            message_type: 1,
            payload_length: 4,
            timestamp_ns: 42,
            message_id: [7u8; 16],
            emotion_tag: 2,
            emotion_score: 200,
            intent: 3,
        };
        let enc = encode_header(&h);
        let dec = decode_header(&enc).unwrap();
        assert_eq!(h, dec);
    }

    #[test]
    fn english_roundtrip() {
        let frame = english_to_frame("hi");
        let text = frame_to_english(&frame).unwrap();
        assert_eq!(text, "hi");
    }

    #[test]
    fn live_state_delta_is_compact() {
        let delta = LiveStateDelta {
            state_id: 7,
            delta_seq: 42,
            intent: Some(3),
            confidence: Some(220),
            urgency: Some(180),
            risk: Some(90),
            affect_tag: Some(4),
            affect_score: Some(170),
            contradiction_pressure: Some(12),
            novelty: Some(40),
            trust_delta: Some(-3),
            repair_request: None,
        };
        let frame = encode_live_state_delta_frame(&delta);
        assert_eq!(frame.len(), 19);
        assert_eq!(frame[0], TOKEN_LIVE_STATE_DELTA);
    }

    #[test]
    fn payload_comparison_shows_reduction() {
        let text = "I think this market move is moderately risky but the confidence is high and urgency is elevated.";
        let json = r#"{"state_id":7,"delta_seq":42,"intent":3,"confidence":220,"urgency":180,"risk":90,"affect_tag":4,"affect_score":170,"contradiction_pressure":12,"novelty":40,"trust_delta":-3}"#;
        let delta = LiveStateDelta {
            state_id: 7,
            delta_seq: 42,
            intent: Some(3),
            confidence: Some(220),
            urgency: Some(180),
            risk: Some(90),
            affect_tag: Some(4),
            affect_score: Some(170),
            contradiction_pressure: Some(12),
            novelty: Some(40),
            trust_delta: Some(-3),
            repair_request: None,
        };
        let cmp = compare_payloads(text, json, &delta);
        assert!(cmp.thp_tcp_bytes < cmp.text_bytes);
        assert!(cmp.thp_tcp_bytes < cmp.json_bytes);
    }
}
