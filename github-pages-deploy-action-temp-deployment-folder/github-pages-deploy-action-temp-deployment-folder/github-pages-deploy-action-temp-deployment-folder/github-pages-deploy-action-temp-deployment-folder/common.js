function read_mem_as_string(memory, ptr, len) {
    let char_codes = [];
    //console.log(typeof(memory),memory);
    let arr = new Int8Array(memory);
    for (let i = 0; i < len; i++) {
        char_codes.push(arr[ptr + i]);
    }
    return String.fromCharCode(...char_codes);
}

function get_str_as_wasmstr(wasm, str){

    let malloc = wasm.exports.alloc;
    let memory = wasm.exports.memory;
    let buffer = memory.buffer;
    let utf8Encode = new TextEncoder();


    let jsArr = utf8Encode.encode(str);
    let len = jsArr.length;
    let wasmArrPtr =  malloc(len);
    buffer = memory.buffer;
    let wasmArr = new Uint8Array(buffer,wasmArrPtr, len);

    wasmArr.set(jsArr);
    return [wasmArrPtr, len];
}

function log(memory) {
    if (arguments.length == 3) {
        console.log(read_mem_as_string(...arguments));
    }
}

export {log, get_str_as_wasmstr };