use thp_tcp_translator::{english_to_frame, encode_header, frame_to_english};

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
}
