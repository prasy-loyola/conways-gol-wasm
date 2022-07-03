
import { log, get_str_as_wasmstr } from "./common.js"
import { draw_rect } from "./canvas.js"
var wasm = null;
let canvas = document.getElementById("canvas");
let ctx = canvas.getContext("2d");

(async () => {
  let response = await fetch('target/wasm32-unknown-unknown/release/wasm_game.wasm');
  let bytes = await response.arrayBuffer();
  let { instance } = await WebAssembly.instantiate(bytes, {
    "env": {
      "alert": (ptr, len) => log(instance.exports.memory.buffer, ptr, len),
      "fillRect": (x, y, w, h, r, g, b, a) => draw_rect(ctx, x, y, w, h, r, g, b, a)
    }

  });

  wasm = instance;
  console.log(wasm.exports.memory);
  try {
    let game = instance.exports.init();
    console.log(game);
    window.requestAnimationFrame(() =>
      instance.exports.render(game)
    );
    setInterval(() => {
      window.requestAnimationFrame(
        () => {

          instance.exports.update(game);
          instance.exports.render(game);
        }


      );
    }, 1000);
  } catch (e) {
    console.log(e)
  }
})();