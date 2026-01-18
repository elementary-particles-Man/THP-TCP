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
}
