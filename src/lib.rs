#![allow(unused)]

static BASE64_CHAR_TABLE: &[u8] =
    "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/=".as_bytes();

// 编码
pub fn b2a(str: &str) -> String {
    String::from_utf8(base64_encode(&str.as_bytes())).unwrap()
}

pub fn base64_encode(source: &[u8]) -> Vec<u8> {
    let mut buffer = vec![];
    for i in 0..source.len() {
        if i % 3 == 0 {
            if i < source.len() - 2 {
                encode_group(&[source[i], source[i + 1], source[i + 2]], 0)
            } else if i == source.len() - 2 {
                encode_group(&[source[i], source[i + 1], 0], 1)
            } else {
                // i == source.len() - 1
                encode_group(&[source[i], 0, 0], 2)
            }
            .iter()
            .for_each(|&c| buffer.push(c));
        }
    }
    buffer
}

fn encode_group(chars: &[u8; 3], lack_num: i32) -> [u8; 4] {
    let [a, b, c] = chars;
    let encode_index_char1 = (a >> 2) as usize;
    let encode_index_char2 = (((a & 0b00000011) << 4) | (b >> 4)) as usize;
    let encode_index_char3 = if lack_num < 2 {
        ((b & 0b00001111) << 2) | (c >> 6)
    } else {
        64
    } as usize;
    let encode_index_char4 = if lack_num < 1 { c & 0b00111111 } else { 64 } as usize;
    [
        BASE64_CHAR_TABLE[encode_index_char1],
        BASE64_CHAR_TABLE[encode_index_char2],
        BASE64_CHAR_TABLE[encode_index_char3],
        BASE64_CHAR_TABLE[encode_index_char4],
    ]
}

// 解码
pub fn a2b(encoded_str: &str) -> String {
    String::new()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_str() {
        assert_eq!(b2a("hello world").as_str(), "aGVsbG8gd29ybGQ=");
    }
    #[test]
    fn test_binary() {
        let b = std::fs::read("./hashiqi.jpeg").unwrap();
        let s = std::fs::read("./base64.txt").unwrap();
        assert_eq!(base64_encode(&b), s);
    }
}
