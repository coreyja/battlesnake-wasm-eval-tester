const init = async () => {
  if (WebAssembly) {
    // WebAssembly.instantiateStreaming is not currently available in Safari
    if (WebAssembly && !WebAssembly.instantiateStreaming) { // polyfill
      WebAssembly.instantiateStreaming = async (resp, importObject) => {
        const source = await (await resp).arrayBuffer();
        return await WebAssembly.instantiate(source, importObject);
      };
    }

    const go = new Go();
    const result = await WebAssembly.instantiateStreaming(fetch("../wasm/official.wasm"), go.importObject)
    go.run(result.instance);

    return window.evaluateMoves;
  } else {
    console.log("WebAssembly is not supported in your browser")
  }
};

export { init };
