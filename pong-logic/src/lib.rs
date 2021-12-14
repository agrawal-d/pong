use std::cmp;

use rand::Rng;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::wasm_bindgen;

pub static BALL_DELTA_PER_STEP: i32 = 1;
pub static PADDLE_HEIGHT: i32 = 100;
pub static PADDLE_WIDTH: i32 = 10;
pub static BASE_WIDTH: i32 = 1200;
pub static BASE_HEIGHT: i32 = 600;
pub static BALL_RADIUS: i32 = 10;
pub static MAX_LIVES: i32 = 5;
pub static TICK_DELAY: i32 = 10;

/** State of the Pong game, shared by both players */
#[wasm_bindgen]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct State {
    p1_paddle: i32,
    p2_paddle: i32,
    p1_lives: i32,
    p2_lives: i32,
    ball_x_speed: i32,
    ball_y_speed: i32,
    ball_x: i32,
    ball_y: i32,
    step: i32,
    last_special_event: Option<String>,
}
#[wasm_bindgen]
impl State {
    #[wasm_bindgen(constructor)]
    pub fn new() -> State {
        State {
            p1_paddle: chaos(BASE_HEIGHT / 2),
            p2_paddle: chaos(BASE_HEIGHT / 2),
            p1_lives: MAX_LIVES,
            p2_lives: MAX_LIVES,
            ball_x_speed: State::base_ball_speed(),
            ball_y_speed: State::base_ball_speed(),
            ball_x: BASE_WIDTH / 2,
            ball_y: BASE_HEIGHT / 2,
            step: 0,
            last_special_event: None,
        }
    }

    /** Calculate the state at time `Tn+1` from the state at `Tn` */
    pub fn next_state(&self, p1_paddle_delta: i32, p2_paddle_delta: i32) -> State {
        let s0 = self;
        let mut s1 = s0.to_owned();
        s1.step += 1;

        // Make changes to get next state ( ignoring collisions and boundaries )

        s1.p1_paddle += p1_paddle_delta;
        s1.p2_paddle += p2_paddle_delta;
        s1.p1_paddle = cmp::max(s1.p1_paddle, 0);
        s1.p1_paddle = cmp::min(s1.p1_paddle, BASE_HEIGHT);
        s1.p2_paddle = cmp::max(s1.p2_paddle, 0);
        s1.p2_paddle = cmp::min(s1.p2_paddle, BASE_HEIGHT);

        let ball_dx = s0.ball_x_speed * BALL_DELTA_PER_STEP;
        let ball_dy = s0.ball_y_speed * BALL_DELTA_PER_STEP;

        s1.ball_x = s0.ball_x + ball_dx;
        s1.ball_y = s0.ball_y + ball_dy;

        let left_collision = (s1.ball_x - BALL_RADIUS < PADDLE_WIDTH)
            && (s1.ball_y - s1.p1_paddle).abs() * 2 < PADDLE_HEIGHT;
        let right_collision = s1.ball_x + BALL_RADIUS > BASE_WIDTH - PADDLE_WIDTH
            && (s1.ball_y - s1.p2_paddle).abs() * 2 < PADDLE_HEIGHT;

        let top_collision = s1.ball_y < 0;
        let bottom_collision = s1.ball_y > BASE_HEIGHT;

        // Collision with a paddle
        if left_collision || right_collision {
            // debug!("Paddle Collision");
            s1.ball_x_speed = chaos(-s0.ball_x_speed);
            s1.ball_y_speed = chaos(s1.ball_y_speed);
            s1.last_special_event = Some(String::from("PADDLE_COLLISION"));

            if left_collision {
                s1.ball_x = PADDLE_WIDTH * 2;
            } else {
                s1.ball_x = BASE_WIDTH - PADDLE_WIDTH * 2;
            }
        // Top or bottom collision
        } else if top_collision || bottom_collision {
            // debug!("Top/Bottom Collision");

            s1.ball_y_speed = chaos(-s1.ball_y_speed);
            s1.ball_x_speed = chaos(s1.ball_x_speed);
            s1.last_special_event = Some(String::from("EDGE_COLLISION"));

            if top_collision {
                s1.ball_y = 0;
            } else {
                s1.ball_y = BASE_HEIGHT;
            }
        // One of the players died
        } else if s1.ball_x <= 5 || s1.ball_x > BASE_WIDTH {
            if s1.ball_x <= 5 {
                s1.p1_lives -= 1;
            } else {
                s1.p2_lives -= 1;
            }

            // debug!("Die Collision");
            s1.ball_x = chaos(BASE_WIDTH / 2);
            s1.ball_y = chaos(BASE_HEIGHT / 2);
            s1.ball_x_speed = State::base_ball_speed();
            s1.ball_y_speed = State::base_ball_speed();
            s1.last_special_event = Some(String::from("PLAYER_DIE"));
        } else {
            // debug!("No collision");
            s1.last_special_event = None;
        }

        return s1;
    }

    /** Calculate the state at time `Tn` from state at `T0` given `n` inputs */
    pub fn final_state(&self, p1_paddle_deltas: Vec<i32>, p2_paddle_deltas: Vec<i32>) -> State {
        let mut s = self.to_owned();
        assert_eq!(p1_paddle_deltas.len(), p2_paddle_deltas.len());

        let iterations = p1_paddle_deltas.len();
        for i in 0..iterations {
            let next_state = s.next_state(p1_paddle_deltas[i], p2_paddle_deltas[i]);
            s = next_state;
        }

        s
    }

    fn base_ball_speed() -> i32 {
        6
        // let speed_x: i32 = rand::thread_rng().gen_range(-BALL_DELTA_PER_STEP/2, BALL_DELTA_PER_STEP/2);
        // let speed_y = sqrtf64(speed_x*speed_x - )
    }
}

fn chaos(num: i32) -> i32 {
    if num.abs() <= 2 {
        return num;
    }

    let small_val = ((num as f32 * 0.2) as i32).abs();
    let diff = rand::thread_rng().gen_range(-small_val, small_val + 1);
    let res = num + diff;
    res
}
