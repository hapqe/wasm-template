// import { threads } from 'wasm-feature-detect';
import * as Comlink from 'comlink';
function wrapExports(context) {
  return (name, ...params) => {
    const ret = context[name](...params);
    return ret;
  };
}

async function initHandler() {
  const handle = await import(
    '../pkg-parallel/wasm_template.js'
  );
  await handle.default();
  await handle.initThreadPool(navigator.hardwareConcurrency);
  let ret = {};
  Object.keys(handle).forEach(key => {
    // if the entry is a function
    if (typeof handle[key] === 'function') {
      ret[key] = (...params) => {
        return handle[key](...params);
      }
    }
  });

  return Comlink.proxy(ret)
}

Comlink.expose({
  handler: initHandler()
});
