
import { log, get_str_as_wasmstr } from "./common.js"
import { draw_rect } from "./canvas.js"
var wasm = null;
let canvas = document.getElementById("canvas");
let ctx = canvas.getContext("2d");

var initial_state = `
! 44p12.3.cells
! Nicolay Beluchenko
! https://conwaylife.com/wiki/44P12.3
! https://www.conwaylife.com/patterns/44p12.3.cells
.......O......
......OO......
.....OO.......
....O.........
...O.OOOO.....
..O.O....O....
.OO.O.OO.O..OO
OO..O.OO.O.OO.
....O....O.O..
.....OOOO.O...
.........O....
.......OO.....
......OO......
......O.......
`;

(async () => {
  let response = await fetch('wasm_game.wasm');
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
    let game = instance.exports.init(...get_str_as_wasmstr(instance,initial_state), 10 ,10 );
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
    }, 80);
  } catch (e) {
    console.log(e)
  }
})();