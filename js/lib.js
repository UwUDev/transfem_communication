const CHARSET = [":3", "ε:", ">:3", "ε:<", ":<", ">:", ">~<", ">-<"];

function encode(bytes) {
    let result = "";
    let buffer = 0;
    let bufferBits = 0;

    bytes = Array.from(bytes);
    bytes.forEach((byte) => {
        buffer = (buffer << 8) | byte;
        bufferBits += 8;
        while (bufferBits >= 3) {
            bufferBits -= 3;
            let index = (buffer >> bufferBits) & 0x07;
            result += CHARSET[index] + ' ';
        }
    });
    if (bufferBits > 0) {
        result += CHARSET[buffer << (3 - bufferBits)];
    }
    return result.trim();
}
function encodeStr(message) {
    const bytes = new TextEncoder().encode(message);
    return encode(bytes);
}

function decode(message) {
    let result = [];
    let buffer = 0;
    let bufferBits = 0;
    const map = new Map(CHARSET.map((char, index) => [char, index]));

    message.split(' ').forEach((char) => {
        if (char === "") return;
        if (!map.has(char)) throw new Error("Invalid character in input");

        buffer = (buffer << 3) | map.get(char);
        bufferBits += 3;

        if (bufferBits >= 8) {
            bufferBits -= 8;
            result.push((buffer >> bufferBits) & 0xFF);
            buffer &= (1 << bufferBits) - 1;
        }
    });

    if (bufferBits > 0) {
        result.push(buffer & ((1 << bufferBits) - 1));
    }

    return result;
}

function decodeStr(message) {
    const bytes = decode(message);
    return new TextDecoder().decode(new Uint8Array(bytes));
}