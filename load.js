
import { log, get_str_as_wasmstr } from "./common.js"
import { draw_rect } from "./canvas.js"
import { patterns } from "./patterns/index.js";
let canvas = document.getElementById("canvas");
let ctx = canvas.getContext("2d");

canvas.setAttribute("width", window.innerWidth);
canvas.setAttribute("height", window.innerHeight);
let selectedPattern = new URL(window.location.href).searchParams.get("pattern");
let selectedCellSize = new URL(window.location.href).searchParams.get("cellsize");
let selectedRefreshRate = new URL(window.location.href).searchParams.get("refreshms");
let patternSelect = document.getElementById("patterns");
let cellSizeInput = document.getElementById("cellsize");
let cellSizeOutput = document.getElementById("cellsize-output");
let refreshIntervalInput = document.getElementById("refreshInterval");
let refreshIntervalOutput = document.getElementById("refreshInterval-output");
var cellSize = selectedCellSize ? selectedCellSize * 1 :  5;
cellSizeInput.value = cellSize;

var refreshIntervalInMs = selectedRefreshRate ? selectedRefreshRate * 1 : 120;

refreshIntervalInput.value = refreshIntervalInMs;

function center_pattern(pattern) {
  let lines = pattern.split("\n")
    .filter(l => !(l.startsWith("!") || l == ""));
  let max_length = lines.length;
  let max_width = lines.map(l => l.length)
    .reduce((p, c) => p > c ? p : c);

  let col_offset = (((window.innerWidth / cellSize) - max_width) / 2).toFixed(0);
  let row_offset = (((window.innerHeight / cellSize) - max_length) / 2).toFixed(0);
  return [row_offset, col_offset];
}


patterns.slice(0).forEach(p => {
  let option = document.createElement("option");
  option.text = p.replace(".cells", "");
  option.value = p;
  patternSelect.appendChild(option);
})

let rand = () => Math.floor(Math.random() * 2);

let current_pattern = "";

for (let i = 0; i < window.innerHeight/cellSize; i++) {
  for (let j = 0; j < window.innerWidth/cellSize; j++) {
    current_pattern += rand() == 1 ? "O" : ".";
  }
  current_pattern += "\n";

}
(async () => {
  let response = await fetch('wasm_game.wasm');
  let bytes = await response.arrayBuffer();
  let { instance } = await WebAssembly.instantiate(bytes, {
    "env": {
      "print": (ptr, len) => log(instance.exports.memory.buffer, ptr, len),
      "fillRect": (x, y, w, h, r, g, b, a) => draw_rect(ctx, x, y, w, h, r, g, b, a)
    }

  });

  console.log(instance.exports.memory);

  if (selectedPattern && patterns.indexOf(selectedPattern + ".cells") >= 0) {
    patternSelect.value = selectedPattern + ".cells";
    let resp = await fetch("patterns/" + selectedPattern + ".cells");
    current_pattern = await resp.text();
  }

  center_pattern(current_pattern);
  let game = instance.exports.init(window.innerWidth, window.innerHeight - 50, cellSize, 0);
  instance.exports.add_pattern(game, ...get_str_as_wasmstr(instance, current_pattern),
    ...center_pattern(current_pattern)
  );

  patternSelect.addEventListener("change", async (e) => {
    let resp = await fetch("patterns/" + e.target.value);
    let pattern = await resp.text();
    instance.exports.reset(game);
    center_pattern(pattern);
    instance.exports.add_pattern(game, ...get_str_as_wasmstr(instance, pattern),
      ...center_pattern(pattern)
    );
    current_pattern = pattern;
    let newURL = new URL(window.location.href)
    newURL.searchParams.set("pattern", e.target.value.replace(".cells", ""))
    window.history.pushState({}, null, newURL.toString());
  });

  cellSizeInput.addEventListener("change", async (e) => {
    console.log("changing cell size to ", e.target.value);
    cellSize = e.target.value;
    cellSizeOutput.value = cellSize;
    instance.exports.change_cell_size(game, cellSize);
    center_pattern(current_pattern);
    instance.exports.add_pattern(game, ...get_str_as_wasmstr(instance, current_pattern),
      ...center_pattern(current_pattern)
    );
    let newURL = new URL(window.location.href)
    newURL.searchParams.set("cellsize", e.target.value)
    window.history.pushState({}, null, newURL.toString());
  });

  refreshIntervalInput.addEventListener("change", async (e) => {
    console.log("changing cell size to ", e.target.value);
    refreshIntervalInMs = e.target.value;
    refreshIntervalOutput.value = refreshIntervalInMs;
    let newURL = new URL(window.location.href);
    newURL.searchParams.set("refreshms", e.target.value);
    window.location.href = newURL.toString();
  });

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
}, refreshIntervalInMs);
}) ();