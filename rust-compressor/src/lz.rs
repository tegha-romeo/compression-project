pub fn compress_lz(data: &[u8]) -> Vec<u8> {
    let mut result = Vec::new();
    let window_size = 20;
    let mut i = 0;

    while i < data.len() {
        let mut match_offset = 0;
        let mut match_length = 0;

        let start = if i > window_size { i - window_size } else { 0 };

        for j in start..i {
            let mut k = 0;
            while i + k < data.len()
                && data[j + k] == data[i + k]
                && k < 255
                && j + k < i
            {
                k += 1;
            }

            if k > match_length && k >= 3 {
                match_offset = i - j;
                match_length = k;
            }
        }

        if match_length >= 3 {
            result.push(0x01);
            result.push(match_offset as u8);
            result.push(match_length as u8);
            i += match_length;
        } else {
            result.push(0x00);
            result.push(data[i]);
            i += 1;
        }
    }

    result
}

pub fn decompress_lz(data: &[u8]) -> Vec<u8> {
    let mut result = Vec::new();
    let mut i = 0;

    while i < data.len() {
        let flag = data[i];
        if flag == 0x00 {
            result.push(data[i + 1]);
            i += 2;
        } else if flag == 0x01 {
            let offset = data[i + 1] as usize;
            let length = data[i + 2] as usize;
            let start = result.len() - offset;

            for j in 0..length {
                result.push(result[start + j]);
            }

            i += 3;
        } else {
            panic!("Invalid LZ77 flag: {}", flag);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lz_roundtrip() {
        let input = b"ABABABABABAB";
        let compressed = compress_lz(input);
        let decompressed = decompress_lz(&compressed);
        assert_eq!(input.to_vec(), decompressed);
    }
}
