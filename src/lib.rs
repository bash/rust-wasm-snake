#![feature(proc_macro, wasm_custom_section, wasm_import_module, crate_in_paths)]

extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

const TICK: f64 = 17.0;
pub(crate) mod board;
pub(crate) mod entity;
pub(crate) mod game;

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

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    #[wasm_bindgen(js_namespace = Date)]
    fn now() -> f64;
}

#[wasm_bindgen]
#[derive(Debug)]
pub enum KeyboardButton {
    Up,
    Down,
    Left,
    Right,
    Escape,
}

#[wasm_bindgen]
pub struct Game {
    dimensions: Dimensions,
    snake: Coordinate,
    snake_direction: Direction,
    ctx: CanvasRenderingContext2D,
}

#[wasm_bindgen]
impl Game {
    pub fn new(width: u32, height: u32, ctx: CanvasRenderingContext2D) -> Game {
        Game {
            dimensions: Dimensions { width, height },
            snake: Coordinate { x: 0, y: 0 },
            snake_direction: Direction::East,
            ctx,
        }
    }

    pub fn on_key_press(&mut self, button: KeyboardButton) {
        match button {
            KeyboardButton::Left => {
                self.snake_direction = Direction::West;
            }
            KeyboardButton::Right => {
                self.snake_direction = Direction::East;
            }
            _ => {
                log(&format!("Unsupported keyboard input {:?}", button));
            }
        }
    }

    pub fn tick(&mut self) -> f64 {
        let tick_begin = now();

        self.ctx
            .clear_rect(0, 0, self.dimensions.width, self.dimensions.height);
        self.ctx.set_fill_style("rgb(200, 200, 200)");
        self.ctx.fill_rect(self.snake.x, self.snake.y + 100, 30, 30);

        // log(&format!("{:#?}", self.snake.x));

        if self.snake.x + 30 >= self.dimensions.width {
            self.snake_direction = Direction::West;
        }

        if self.snake.x <= 0 {
            self.snake_direction = Direction::East;
        }

        match self.snake_direction {
            Direction::East => {
                self.snake.x += 5;
            }
            Direction::West => {
                self.snake.x -= 5;
            }
        }

        let delta = now() - tick_begin;
        let mut sleep_duration = TICK - delta;

        if sleep_duration < 0.0 {
            sleep_duration = 0.0;
        }

        sleep_duration
    }
}
