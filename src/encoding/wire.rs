pub const MAX_BYTES: usize = 6;

pub fn encode_i32(mut num: i32) -> Vec<u8> {
    let mut result: Vec<u8> = Vec::with_capacity(MAX_BYTES);

    let negative_mask = if num >= 0 { 0 } else { 4 };

    num = num.abs();

    result.push(64 + (num & 3) as u8);

    num >>= 2;

    while num != 0 {
        result.push(64 + (num & 63) as u8);

        num >>= MAX_BYTES;
    }

    // This will either work or need to be changed with a `shift` call
    result[0] = result[0] | (result.len() << 3) as u8 | negative_mask;

    result.shrink_to_fit();

    result
}

pub fn encode_bool(boolean: bool) -> Vec<u8> {
    if boolean {
        encode_i32(1)
    } else {
        encode_i32(0)
    }
}

pub fn decode_i32<T: AsRef<[u8]>>(bytes: T) -> (i32, usize) {
    let bytes = bytes.as_ref();

    if bytes.is_empty() {
        return (0, 0);
    }

    let mut result = (bytes[0] & 3) as i32;

    let is_negative = bytes[0] & 4 == 4;

    let bytes_length = (bytes[0] >> 3 & 7) as usize;

    let mut shift_by = 2;

    if bytes_length > 1 {
        bytes[1..bytes_length]
            .iter()
            .enumerate()
            .for_each(|(index, byte)| {
                result |= ((*byte & 63) << shift_by) as i32;

                shift_by = 2 + 6 * (index + 1);
            });
    }

    if is_negative {
        result *= -1;
    }

    (result, bytes_length)
}
