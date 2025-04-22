pub fn compress_rle(data: &[u8]) -> Vec<u8> {
    let mut result = Vec::new();
    let mut i = 0;

    while i < data.len() {
        let byte = data[i];
        let mut count = 1;

        while i + count < data.len() && data[i + count] == byte && count < 255 {
            count += 1;
        }

        result.push(byte);
        result.push(count as u8);
        i += count;
    }

    result
}

pub fn decompress_rle(data: &[u8]) -> Vec<u8> {
    let mut result = Vec::new();
    let mut i = 0;

    while i < data.len() {
        let byte = data[i];
        let count = data[i + 1];
        result.extend(std::iter::repeat(byte).take(count as usize));
        i += 2;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rle_roundtrip() {
        let input = b"AAABBBCCCCCDDDDE";
        let compressed = compress_rle(input);
        let decompressed = decompress_rle(&compressed);
        assert_eq!(input.to_vec(), decompressed);
    }
}
