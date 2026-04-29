import init, { Game } from "./game.js";

let last = performance.now();
let game;


async function start() {
    await init();

    try {
        game = new Game("screen", null);
        console.log("Game created");
    } catch (e) {
        console.error("Failed to create Game:", e);
        return;
    }

    function tick(now) {
        const dt = (now - last) / 1000;
        last = now;

        game.frame(dt);
        requestAnimationFrame(tick);
    }

    requestAnimationFrame(tick);
}

start();
