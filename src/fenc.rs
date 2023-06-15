use chardetng::EncodingDetector;
use encoding_rs::*;
use encoding_rs_io::DecodeReaderBytesBuilder;
use std::result::Result;

pub fn to_utf8(text: &[u8]) -> Result<String, std::string::FromUtf8Error> {
    // First, we try to detect the encoding using chardetng
    let mut detector = chardetng::EncodingDetector::new();
    detector.feed(text, true);
    let encoding = detector.guess(None, true);

    // Then we create a decoder for the detected encoding
    let mut decoder = DecodeReaderBytesBuilder::new()
        .encoding(Some(encoding))
        .build(text);

    // Now we collect the decoded bytes into a UTF-8 string
    let mut decoded = String::new();
    let _ = std::io::Read::read_to_string(&mut decoder, &mut decoded);

    // Return the decoded string
    Ok(decoded)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_utf8() {
        // Test UTF-8 encoded string
        let utf8_text = b"Hello, world!";
        assert_eq!(to_utf8(utf8_text).unwrap(), "Hello, world!");

        // Test ISO-8859-1 (Latin1) encoded string
        let latin1_text = [72, 101, 108, 108, 111, 44, 32, 119, 111, 114, 108, 100, 33]; // "Hello, world!" in ISO-8859-1
        assert_eq!(to_utf8(&latin1_text).unwrap(), "Hello, world!");

        // Test Shift JIS encoded Japanese string
        // "こんにちは世界" in Shift JIS
        let shift_jis_text = [
            130, 81, 130, 83, 130, 85, 130, 87, 130, 89, 144, 44, 164, 170,
        ];
        assert_eq!(to_utf8(&shift_jis_text).unwrap(), "こんにちは世界");
    }
}
