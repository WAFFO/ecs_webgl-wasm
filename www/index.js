// update this file name with name in Cargo.toml
import('./wasm/ecs_webgl_wasm.js')
  .then(webgl => webgl.run())
  .catch(console.error);