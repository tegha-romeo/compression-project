function compress(data) {
    const result = [];
    const windowSize = 20;

    let i = 0;
    while (i < data.length) {
        let matchOffset = 0;
        let matchLength = 0;

        const start = Math.max(0, i - windowSize);

        for (let j = start; j < i; j++) {
            let k = 0;
            while (
                i + k < data.length &&
                data[j + k] === data[i + k] &&
                k < 255 &&
                j + k < i
            ) {
                k++;
            }

            if (k >= 3 && k > matchLength) {
                matchOffset = i - j;
                matchLength = k;
            }
        }

        if (matchLength >= 3) {
            result.push(0x01, matchOffset, matchLength);
            i += matchLength;
        } else {
            result.push(0x00, data[i]);
            i++;
        }
    }

    return Buffer.from(result);
}

function decompress(data) {
    const result = [];

    let i = 0;
    while (i < data.length) {
        const flag = data[i];
        if (flag === 0x00) {
            result.push(data[i + 1]);
            i += 2;
        } else if (flag === 0x01) {
            const offset = data[i + 1];
            const length = data[i + 2];
            const start = result.length - offset;
            for (let j = 0; j < length; j++) {
                result.push(result[start + j]);
            }
            i += 3;
        } else {
            throw new Error("Invalid LZ token");
        }
    }

    return Buffer.from(result);
}

module.exports = { compress, decompress };
