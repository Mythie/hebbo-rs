pub fn encode(int: i32, num_bytes: i32) -> Vec<u8> {
    let mut result: Vec<u8> = Vec::with_capacity(num_bytes as usize);

    for i in 1..=num_bytes {
        let bit_shift = (num_bytes - i) * 6;

        let byte = (int >> bit_shift) & 63;

        let byte = byte + 64;

        result.push(byte as u8)
    }

    result
}

pub fn decode<T: AsRef<[u8]>>(bytes: T) -> i32 {
    let mut result = 0;

    for (iteration, byte) in bytes.as_ref().iter().rev().enumerate() {
        let mut num = *byte as i32 - 64;

        if iteration > 0 {
            num = (num as f64 * f64::powf(64.0, iteration as f64)) as i32;
        }

        result += num;
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_decode_init_crypto_header() {
        assert_eq!(decode(b"CN"), 206);
    }

    #[test]
    fn test_encode_session_params_header() {
        assert_eq!(encode(257, 2), b"DA");
    }
}
