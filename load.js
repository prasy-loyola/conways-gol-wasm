
import { log, get_str_as_wasmstr } from "./common.js"
import { draw_rect } from "./canvas.js"
import { patterns } from "./patterns/index.js";
let canvas = document.getElementById("canvas");
let ctx = canvas.getContext("2d");

canvas.setAttribute("width", window.innerWidth);
canvas.setAttribute("height", window.innerHeight);
let selectedPattern = new URL(window.location.href).searchParams.get("pattern");
let patternSelect = document.getElementById("patterns");

const CELL_SIZE = 14;

function center_pattern(pattern) {
  let lines = pattern.split("\n")
    .filter(l => !(l.startsWith("!") || l == ""));
  let max_length = lines.length;
  let max_width = lines.map(l => l.length)
    .reduce((p, c) => p > c ? p : c);

  let col_offset = (((window.innerWidth / CELL_SIZE) - max_width) / 2).toFixed(0);
  let row_offset = (((window.innerHeight / CELL_SIZE) - max_length) / 2).toFixed(0);
  return [row_offset, col_offset];
}


patterns.slice(0).forEach(p => {
  let option = document.createElement("option");
  option.text = p.replace(".cells", "");
  option.value = p;
  patternSelect.appendChild(option);
})

let rand = () => Math.floor(Math.random() * 2);

let initial_state = "";

for (let i = 0; i < 200; i++) {
  for (let j = 0; j < 200; j++) {
    initial_state += rand() == 1 ? "O" : ".";
  }
  initial_state += "\n";

}
(async () => {
  let response = await fetch('wasm_game.wasm');
  let bytes = await response.arrayBuffer();
  let { instance } = await WebAssembly.instantiate(bytes, {
    "env": {
      "alert": (ptr, len) => log(instance.exports.memory.buffer, ptr, len),
      "fillRect": (x, y, w, h, r, g, b, a) => draw_rect(ctx, x, y, w, h, r, g, b, a)
    }

  });

  console.log(instance.exports.memory);

  if (selectedPattern && patterns.indexOf(selectedPattern + ".cells") >= 0) {
    patternSelect.value = selectedPattern + ".cells";
    let resp = await fetch("patterns/" + selectedPattern + ".cells");
    initial_state = await resp.text();
  }

  center_pattern(initial_state);
  let game = instance.exports.init(window.innerWidth, window.innerHeight - 50, CELL_SIZE, 1);
  instance.exports.add_pattern(game, ...get_str_as_wasmstr(instance, initial_state),
    ...center_pattern(initial_state)
  );

  patternSelect.addEventListener("change", async (e) => {
    let resp = await fetch("patterns/" + e.target.value);
    let pattern = await resp.text();
    instance.exports.reset(game);
    center_pattern(pattern);
    instance.exports.add_pattern(game, ...get_str_as_wasmstr(instance, pattern),
      ...center_pattern(pattern)
    );

    let newURL = new URL(window.location.href)
    newURL.searchParams.set("pattern", e.target.value.replace(".cells", ""))
    window.history.pushState({}, null, newURL.toString());
  })

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
  }, 120);
})();