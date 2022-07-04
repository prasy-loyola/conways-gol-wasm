
import { log, get_str_as_wasmstr } from "./common.js"
import { draw_rect } from "./canvas.js"
import { patterns } from "./patterns/index.js";
let canvas = document.getElementById("canvas");
let ctx = canvas.getContext("2d");

canvas.setAttribute("width", window.innerWidth);
canvas.setAttribute("height", window.innerHeight);
let selectedPattern = new URL(window.location.href).searchParams.get("pattern");
let patternSelect = document.getElementById("patterns");
if (screen.lockOrientation) {

  screen.lockOrientation('landscape');
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
  let game = instance.exports.init(window.innerWidth, window.innerHeight - 50);

  instance.exports.add_pattern(game,...get_str_as_wasmstr(instance, initial_state), 1, 1);

  patternSelect.addEventListener("change", async (e) => {
    //console.log(e.target.value)
    let resp = await fetch("patterns/" + e.target.value);
    let pattern = await resp.text();
    instance.exports.reset(game);
    instance.exports.add_pattern(game, ...get_str_as_wasmstr(instance, pattern), 10, 10);
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
  }, 80);
})();