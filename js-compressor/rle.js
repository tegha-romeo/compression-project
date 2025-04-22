// RLE: Byte + Count
function compress(inputBuffer) {
    const result = [];
    const data = inputBuffer;
    let i = 0;

    while (i < data.length) {
        const byte = data[i];
        let count = 1;

        while (i + count < data.length && data[i + count] === byte && count < 255) {
            count++;
        }

        result.push(byte);
        result.push(count);
        i += count;
    }

    return Buffer.from(result);
}

function decompress(inputBuffer) {
    const result = [];
    const data = inputBuffer;
    let i = 0;

    while (i < data.length) {
        const byte = data[i];
        const count = data[i + 1];
        for (let j = 0; j < count; j++) {
            result.push(byte);
        }
        i += 2;
    }

    return Buffer.from(result);
}

module.exports = { compress, decompress };
