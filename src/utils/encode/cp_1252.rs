use encoding_rs::WINDOWS_1252;

fn to_cp1252(s: &str) -> Vec<u8> {
    let (encoded, _, _) = WINDOWS_1252.encode(s);
    encoded.into_owned()
}
