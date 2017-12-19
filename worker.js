// hack to provide something we can override once we have a wasm instance
function web_debug_log_delegate(ptr) {
    console.log("uninitialized debug log delegate called");
}

function web_update_canvas_delegate() {
    console.log("uninitialized canvas delegate called");
}

let wasmEnv = {
    env: {
        cos:                   Math.cos,
        sin:                   Math.sin,
        pow:                   Math.pow,
        web_current_timestamp: () => {
            return performance.now()
        },
        web_debug_log:         (ptr) => {
            web_debug_log_delegate(ptr)
        },
        web_prng:              () => {
            const min = 0;
            const max = 4294967295; // u32 max

            return Math.floor(Math.random() * (max - min + 1)) + min;
        },
        web_update_canvas:     () => {
            web_update_canvas_delegate();
        }
    }
};

function copyCStr(wasmInstance, ptr) {
    const collectCString = function* () {
        let memory = new Uint8Array(wasmInstance.memory.buffer);
        // ptr is relative to wasm heap, so must read from that buffer
        while (memory[ptr] !== 0) {
            if (memory[ptr] === undefined) {
                throw new Error("Tried to read undef mem")
            }
            yield memory[ptr];
            ptr += 1;
        }
    };

    const buffer_as_u8 = new Uint8Array(collectCString());
    const utf8Decoder = new TextDecoder("UTF-8");
    return utf8Decoder.decode(buffer_as_u8)
}

onmessage = (e) => {
    if (e.data['type'] === 'start') {
        const exampleName = e.data['exampleName'];

        console.log("Starting example " + exampleName);

        fetch(`target/wasm32-unknown-unknown/release/examples/${exampleName}.wasm`)
            .then(response => response.arrayBuffer())
            // Rust lacks a wasm cos, so provide JS's
            .then(bytes => WebAssembly.instantiate(bytes, wasmEnv))
            .then(results => results.instance)
            .then(instance => {
                    const wasmTurtle = {};
                    wasmTurtle.alloc = instance.exports.alloc;
                    wasmTurtle.dealloc = instance.exports.dealloc;
                    wasmTurtle.memory = instance.exports.memory;
                    wasmTurtle.start = instance.exports.web_turtle_start;

                    // override inner logging bits
                    web_debug_log_delegate = (ptr) => {
                        console.log(copyCStr(wasmTurtle, ptr));
                    };

                    return wasmTurtle;
                }
            ).then(wasmTurtle => {

            const width = e.data['width'];
            const height = e.data['height'];

            let byteSize = width * height * 4;
            const pointer = wasmTurtle.alloc(byteSize);
            const pixelArray = new Uint8ClampedArray(wasmTurtle.memory.buffer, pointer, byteSize);

            web_update_canvas_delegate = () => {
                // copy pixel data
                const copy = Uint8ClampedArray.from(pixelArray);

                postMessage({
                    'type':   'updateCanvas',
                    'pixels': copy
                });

            };

            wasmTurtle.start(pointer, width, height);
            console.log(`Done with ${exampleName}`)
        }).catch(e => {
            console.log("uh oh: " + e);
        });
    }
};

