
function make_environment(env) {
    return new Proxy(env, {
        get(target, prop, receiver) {
            if (!env.hasOwnProperty(prop)) {
                return (...args) => {console.error("NOT IMPLEMENTED: "+prop, args)}
            }
            return env[prop];
        }
    });
}

WebAssembly.instantiateStreaming(fetch('./pkg/wasm_game_bg.wasm'), {
    "env": make_environment({
        "fill_rect_wasm": (x, y, w, h, r, g, b, a) => {
            r = Math.floor(r*255).toString(16).padStart(2, 0);
            g = Math.floor(g*255).toString(16).padStart(2, 0);
            b = Math.floor(b*255).toString(16).padStart(2, 0);
            a = Math.floor(a*255).toString(16).padStart(2, 0);
            ctx.fillStyle = '#'+r+g+b+a;
            ctx.fillRect(x, y, w, h);
        },
        "fabs": Math.abs,
        "powf": Math.pow,
        "random_get_zero_to_one": Math.random,
        "random_get_within_range": (a, b) => a + Math.random()*(b - a),
    })
}).then(wasmModule => {
}
)
.catch(console.error)