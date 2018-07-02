#![feature(proc_macro, wasm_custom_section, wasm_import_module)]

extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

const TICK: f64 = 17.0;

#[derive(Debug)]
pub struct GameState {
    dimensions: Dimensions,
    snake: Coordinate,
    snake_direction: Direction,
}

#[derive(Debug)]
pub struct Dimensions {
    width: u32,
    height: u32,
}

#[derive(Debug)]
pub struct Coordinate {
    x: u32,
    y: u32,
}

#[derive(Debug)]
pub enum Direction {
    East,
    West,
}

#[wasm_bindgen]
extern "C" {
    pub type CanvasRenderingContext2D;

    #[wasm_bindgen(method, setter = fillStyle)]
    pub fn set_fill_style(this: &CanvasRenderingContext2D, fill_style: &str);

    #[wasm_bindgen(method, js_name = fillRect)]
    pub fn fill_rect(this: &CanvasRenderingContext2D, x: u32, y: u32, width: u32, height: u32);

    #[wasm_bindgen(method, js_name = clearRect)]
    pub fn clear_rect(this: &CanvasRenderingContext2D, x: u32, y: u32, width: u32, height: u32);

    // #[wasm_bindgen(js_namespace = console)]
    // fn log(s: &str);

    #[wasm_bindgen(js_namespace = Date)]
    fn now() -> f64;
}

#[wasm_bindgen]
pub struct Game {
    state: GameState,
}

#[wasm_bindgen]
impl Game {
    pub fn new(width: u32, height: u32) -> Game {
        Game {
            state: GameState {
                dimensions: Dimensions { width, height },
                snake: Coordinate { x: 0, y: 0 },
                snake_direction: Direction::East,
            },
        }
    }

    pub fn tick(&mut self, ctx: &CanvasRenderingContext2D) -> f64 {
        let tick_begin = now();
        let state = &mut self.state;

        ctx.clear_rect(0, 0, state.dimensions.width, state.dimensions.height);
        ctx.set_fill_style("rgb(200, 200, 200)");
        ctx.fill_rect(state.snake.x, state.snake.y + 100, 30, 30);

        // log(&format!("{:#?}", state));

        match state.snake_direction {
            Direction::East => {
                state.snake.x += 5;
            }
            Direction::West => {
                state.snake.x -= 5;
            }
        }

        if state.snake.x + 30 >= state.dimensions.width {
            state.snake_direction = Direction::West;
        }

        if state.snake.x <= 0 {
            state.snake_direction = Direction::East;
        }

        let delta = now() - tick_begin;
        let mut sleep_duration = TICK - delta;

        if sleep_duration < 0.0 {
            sleep_duration = 0.0;
        }

        sleep_duration
    }
}
