(function(window, document, undefined) {
  // Let's goooo!
  runWASM('/target/wasm32-unknown-unknown/release/examples/circle.gc.wasm', 'main');

  /**
   * Run the given WASM URL using the given entry-point function
   */
  function runWASM(wasmUrl, entryPoint) {
    const wasm = {};
    const imports = {
      env: {
        cos(x) { return Math.cos(x); },
        sin(x) { return Math.sin(x); },
        now() { return Date.now(); },
        _log(message_ptr) {
          const message = copyStringFromWasm(wasm, message_ptr);
          console.log(message);
        },
        send_query(str_ptr) {
          const str = copyStringFromWasm(wasm, str_ptr);
          console.log(str);
          const response = createWasmString(wasm, '{"TurtleState":{"pen":{"enabled":true,"thickness":1.0,"color":{"red":0.0,"green":0.0,"blue":0.0,"alpha":1.0}},"fill_color":{"red":0.0,"green":0.0,"blue":0.0,"alpha":1.0},"position":{"x":0.0,"y":0.0},"heading":1.5707963267948966,"speed":{"Value":10},"visible":true}}');
          return response;
        },
      },
    };
    WebAssembly.instantiateStreaming(fetch(wasmUrl), imports).then(wasmModule => {
      console.log("Successfully loaded WASM module!");
      window.wasmModule = wasmModule;

      // Automatically pass along the exported functions
      Object.keys(wasmModule.instance.exports).forEach((key) => {
        wasm[key] = wasmModule.instance.exports[key];
      });
      wasm[entryPoint]();
    }).catch((err) => console.error(err));
  }

  /**
  * Copy C string from WASM into a JavaScript string
  *
  * C strings are NULL terminated, so this function will continue to read memory
  * until it reaches the NULL terminator '\0'. If no NULL terminator is found,
  * an exception will be thrown.
  */
  function copyStringFromWasm(wasm, ptr) {
    // Code adapted from: https://github.com/WasmBlock/WasmBlock/blob/bc5959dd7b0d0d5f5ed4033b149591574bad68b6/wasmblock.js#L31

    const orig_ptr = ptr;
    const collectCString = function* () {
      const memory = new Uint8Array(wasm.memory.buffer);
      while (memory[ptr] !== 0) {
        if (memory[ptr] === undefined) {
          throw new Error("Tried to read undef mem");
        }
        yield memory[ptr];
        ptr += 1;
      }
    }

    const buffer_as_u8 = new Uint8Array(collectCString())
    const utf8Decoder = new TextDecoder("UTF-8");
    const buffer_as_utf8 = utf8Decoder.decode(buffer_as_u8);
    // Free the allocated string memory once we are done with it
    wasm.dealloc_str(orig_ptr);
    return buffer_as_utf8;
  }

  /**
   * Creates a new NULL-terminated string in the WebAssembly memory for the
   * given string. Space is allocated for the length of the given string + 1
   * and then the entire string is encoded and stored as UTF-8 with a
   * NULL-terminator at the end.
   */
  function createWasmString(module, str) {
    // Code adapted from: https://github.com/WasmBlock/WasmBlock/blob/bc5959dd7b0d0d5f5ed4033b149591574bad68b6/wasmblock.js#L71

    const utf8Encoder = new TextEncoder("UTF-8");
    const string_buffer = utf8Encoder.encode(str);
    const len = string_buffer.length;
    const ptr = module.alloc(len+1);

    const memory = new Uint8Array(module.memory.buffer);
    for (let i = 0; i < len; i++) {
      memory[ptr+i] = string_buffer[i];
    }

    memory[ptr+len] = 0;

    return ptr;
  }

  //////////////////////////////// Turtle ////////////////////////////////
  //TODO: Refactor this into its own separate module

  function toRadians(degrees) {
    return degrees * Math.PI / 180.0;
  }

  function defaultTurtle() {
    return {
      pen: {
        enabled: true,
        thickness: 1.0,
        color: {
          red: 0.0,
          green: 0.0,
          blue: 0.0,
          alpha: 1.0,
        },
      },
      fill_color: {
        red: 0.0,
        green: 0.0,
        blue: 0.0,
        alpha: 1.0,
      },
      position: {
        x: 0.0,
        y: 0.0,
      },
      heading: toRadians(90.0),
      speed: {"Value", 10},
      visible: true,
    };
  }

  function defaultDrawing() {
    return {
      title: 'Turtle',
      background: {
        red: 255.0,
        green: 255.0,
        blue: 255.0,
        alpha: 1.0,
      },
      center: {
        x: 0.0,
        y: 0.0,
      },
      width: 800,
      height: 600,
      maximized: false,
      fullscreen: false,
    };
  }
})(window, document);
