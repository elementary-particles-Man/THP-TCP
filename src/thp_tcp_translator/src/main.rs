use thp_tcp_translator::{
    compare_payloads, encode_live_state_delta_frame, english_to_frame, encode_header,
    frame_to_english, LiveStateDelta,
};

fn main() {
    let input = std::env::args().skip(1).collect::<Vec<_>>().join(" ");
    if input.is_empty() {
        eprintln!("usage: thp_tcp_translator <text>");
        std::process::exit(1);
    }

    let frame = english_to_frame(&input);
    let header_bytes = encode_header(&frame.header);
    println!("header_len={} payload_len={}", header_bytes.len(), frame.payload.len());

    match frame_to_english(&frame) {
        Ok(text) => println!("roundtrip={}", text),
        Err(err) => {
            eprintln!("error: {err}");
            std::process::exit(1);
        }
    }

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
    let live = encode_live_state_delta_frame(&delta);
    let json = r#"{"state_id":7,"delta_seq":42,"intent":3,"confidence":220,"urgency":180,"risk":90,"affect_tag":4,"affect_score":170,"contradiction_pressure":12,"novelty":40,"trust_delta":-3}"#;
    let cmp = compare_payloads(&input, json, &delta);
    println!(
        "live_state_delta_bytes={} text_bytes={} json_bytes={} thp_tcp_bytes={}",
        live.len(),
        cmp.text_bytes,
        cmp.json_bytes,
        cmp.thp_tcp_bytes
    );
}
