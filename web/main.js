import init, { Game } from "./game.js";

let last = performance.now();
let game;

async function start() {
    await init();

    // pass WebSocket URL or null
    game = new Game("screen", null);

    function tick(now) {
        const dt = (now - last) / 1000;
        last = now;

        game.frame(dt);
        requestAnimationFrame(tick);
    }

    requestAnimationFrame(tick);
}

start();
