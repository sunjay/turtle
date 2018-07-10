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
        cos: (x) => Math.cos(x),
        sin: (x) => Math.sin(x),
        now: () => Date.now(),
        send_query(raw_str) {
          const str = copyCStr(wasm, raw_str);
          console.log(str);
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
  function copyCStr(wasm, ptr) {
    let orig_ptr = ptr;
    const collectCString = function* () {
      let memory = new Uint8Array(wasm.memory.buffer);
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
})(window, document);
