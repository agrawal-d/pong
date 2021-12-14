import * as wasm from "pong-client";

const DELTA_PER_KEYPRESS = 10;
const TICK_DELAY = 10;
const BASE_WIDTH = 1200;
const BASE_HEIGHT = 600;
const BALL_RADIUS = 10;
const PADDLE_HEIGHT = 100;
const PADDLE_WIDTH = 10;
const PAUSE_TIMEOUT = 2000;

class Pong {
    // P1 - W S
    // P2 - I J

    constructor() {
        this.p1_delta = 0;
        this.p2_delta = 0;
        this.bgMusic = new Audio("./music/bg.mp3");
        this.handleKeys();

        this.canvas = document.getElementById("canvas");
        this.ctx = this.canvas.getContext("2d");

        // document.addEventListener("keypress", (e) => this.handleKeyPress(e));
        this.state = wasm.get_initial_state();
        console.log(this.state);
        wasm.init();
        this.play();
        requestAnimationFrame((ts) => this.render(ts));
    }

    handleKeys() {
        this.pressedKeys = {};
        window.onkeyup = (e) => {
            this.pressedKeys[e.key] = false;
        };
        window.onkeydown = (e) => {
            this.pressedKeys[e.key] = true;
        };
    }

    startTickLoop() {
        return setInterval(() => {
            this.handleKeyPress();
            this.state = wasm.get_next_state(
                this.state,
                this.p1_delta,
                this.p2_delta
            );

            const event = this.state.last_special_event;
            if (event === "EDGE_COLLISION") {
                // this.playEdgeHitMusic();
            } else if (event === "PLAYER_DIE") {
                this.pause();
                if (this.state.p1_lives == 0 || this.state.p2_lives == 0) {
                    this.finished = true;
                    this.playEndMusic();
                } else {
                    setTimeout(() => {
                        this.play();
                    }, PAUSE_TIMEOUT);
                    this.playDieMusic();
                }
            } else if (event === "PADDLE_COLLISION") {
                this.playPaddleHitMusic();
            } else {
                // console.log("No event");
            }

            this.p1_delta = 0;
            this.p2_delta = 0;
        }, TICK_DELAY);
    }

    render(ts) {
        const gameState = this.state;
        const ctx = this.ctx;

        ctx.clearRect(0, 0, BASE_WIDTH, BASE_HEIGHT);

        ctx.fillStyle = "160, 61, 217";

        if (this.finished === true) {
            ctx.font = "48px sans-serif";
            ctx.fillStyle = "rgb(196, 235, 101)";
            ctx.textAlign = "center";
            ctx.fillText("Game Over!", 600, 300);

            return;
        }

        //paddles
        ctx.fillRect(
            0,
            gameState.p1_paddle - PADDLE_HEIGHT / 2,
            PADDLE_WIDTH,
            PADDLE_HEIGHT
        );
        ctx.fillRect(
            BASE_WIDTH - 10,
            gameState.p2_paddle - PADDLE_HEIGHT / 2,
            PADDLE_WIDTH,
            PADDLE_HEIGHT
        );

        if (this.paused === true) {
            ctx.font = "48px sans-serif";
            ctx.fillStyle = "rgb(96, 235, 101)";
            ctx.textAlign = "center";
            ctx.fillText("Get Ready!", 600, 300);
        } else {
            ctx.fillStyle = "rgb(255, 255, 255)";

            ctx.beginPath();
            ctx.arc(
                gameState.ball_x,
                gameState.ball_y,
                BALL_RADIUS,
                0,
                2 * Math.PI,
                false
            );
            ctx.fillStyle = "white";
            ctx.fill();
        }

        // Write score etc.

        ctx.font = "20px sans-serif";
        ctx.fillStyle = "rgb(255,255,20)";
        ctx.textAlign = "left";
        ctx.fillText(`${"❤️ ".repeat(this.state.p1_lives)}`, 0, 30);
        ctx.textAlign = "right";
        ctx.fillText(`${"❤️ ".repeat(this.state.p2_lives)}`, BASE_WIDTH, 30);

        requestAnimationFrame((ts) => this.render(ts));
    }

    handleKeyPress() {
        if (this.pressedKeys["w"]) {
            this.p1_delta -= DELTA_PER_KEYPRESS;
        } else if (this.pressedKeys["s"]) {
            this.p1_delta += DELTA_PER_KEYPRESS;
        } else if (this.pressedKeys["i"]) {
            this.p2_delta -= DELTA_PER_KEYPRESS;
        } else if (this.pressedKeys["k"]) {
            this.p2_delta += DELTA_PER_KEYPRESS;
        }
    }

    pause() {
        this.bgMusic.pause();
        this.paused = true;
        clearInterval(this.tickLoop);
    }

    play() {
        this.bgMusic.play();
        this.bgMusic.loop = true;
        this.paused = false;
        this.tickLoop = this.startTickLoop();
    }

    playBgMusic() {
        this.bgMusic.loop = true;
        m.play();
    }

    pauseBgMusic() {
        this.bgMusic.pause();
    }

    playPaddleHitMusic() {
        const m = new Audio("./music/paddle-hit.wav");
        m.play();
    }

    playEndMusic() {
        const m = new Audio("./music/end.wav");
        m.play();
    }

    playDieMusic() {
        const m = new Audio("./music/die.wav");
        m.play();
    }

    playEdgeHitMusic() {
        const m = new Audio("./music/edge-hit.wav");
        m.play();
    }
}

const startBtn = document.getElementById("start-game");
startBtn.addEventListener("click", () => {
    console.log("Starting game...");
    const p = new Pong();
});
