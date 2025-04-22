const fs = require('fs');
const { compress: rleCompress, decompress: rleDecompress } = require('./rle');
const { compress: lzCompress, decompress: lzDecompress } = require('./lz');

const args = process.argv.slice(2);

if (args.length !== 4) {
    console.error("Usage: compress|decompress <input> <output> --rle|--lz");
    process.exit(1);
}

const [mode, inputPath, outputPath, algoFlag] = args;

const input = fs.readFileSync(inputPath);

let result;
if (mode === 'compress' && algoFlag === '--rle') {
    result = rleCompress(input);
} else if (mode === 'decompress' && algoFlag === '--rle') {
    result = rleDecompress(input);
} else if (mode === 'compress' && algoFlag === '--lz') {
    result = lzCompress(input);
} else if (mode === 'decompress' && algoFlag === '--lz') {
    result = lzDecompress(input);
} else {
    console.error("Invalid mode or algorithm flag");
    process.exit(1);
}

fs.writeFileSync(outputPath, result);
console.log(`${mode}ion complete.`);
