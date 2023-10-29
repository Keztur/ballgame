use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;
use std::vec::Vec;

use lazy_static::lazy_static; // 1.4.0
use std::sync::Mutex;

lazy_static! {
    static ref BALLS: Mutex<Balls> = Mutex::new(Balls::new());
}

pub const FRICTION: f64 = 0.994;  
pub const FORCE: f64 = 0.1;


#[wasm_bindgen]
pub fn run_sim(x_mouse_vec: f64, y_mouse_vec: f64, width: f64, height:f64, context: &CanvasRenderingContext2d) {
    let mut balls = BALLS.lock().unwrap();
    balls.simulate(x_mouse_vec, y_mouse_vec, width, height, context);
}

#[wasm_bindgen]
pub fn add_ball() {
    let mut balls = BALLS.lock().unwrap();
    balls.add();
}

//reduce to integer or f32

fn draw_ball(context: &CanvasRenderingContext2d, x: f64, y: f64, radius: f64, color: &String) {
    context.begin_path();
    context.arc(x, y, radius, 0.0, 6.2831).unwrap();
    context.set_fill_style(&color.into());   
    context.fill();
    context.close_path();
}

pub struct Ball {
   pub x: f64,
   pub y: f64,
   pub x_last: f64,
   pub y_last: f64,
   pub radius: f64,
   pub color: String,
}

pub struct Balls {
    pub balls: Vec<Ball>,
}   

impl Balls {
    
    pub fn new() -> Self {
        let balls = Vec::new();
        Self {
            balls,
        }
    }

    pub fn simulate(&mut self, x_mouse_vec: f64, y_mouse_vec: f64, width: f64, height: f64, context: &CanvasRenderingContext2d ) {

        for ball in self.balls.iter_mut() {

            let x_vec = (ball.x - ball.x_last) * FRICTION;
            let y_vec = (ball.y - ball.y_last) * FRICTION;
            let x_mouse_vec = x_mouse_vec * FORCE;
            let y_mouse_vec = y_mouse_vec * FORCE;

            let x_possible_new = ball.x + x_vec + x_mouse_vec;
            let y_possible_new = ball.y + y_vec + y_mouse_vec;

            let x_border = width - ball.radius;
            
            if x_possible_new < ball.radius {
                ball.x_last = ball.radius + (ball.radius - ball.x);
                ball.x = ball.radius - (x_possible_new - ball.radius) + x_mouse_vec;
            } else if x_possible_new > x_border {
                ball.x_last = x_border + (x_border - ball.x);
                ball.x = x_border - (x_possible_new - x_border) + x_mouse_vec;
            } else {
                ball.x_last = ball.x;
                ball.x = x_possible_new;
            }
            
            let y_border = height - ball.radius;

            if y_possible_new < ball.radius {
                ball.y_last = ball.radius + (ball.radius - ball.y);
                ball.y = ball.radius - (y_possible_new - ball.radius) + y_mouse_vec;
            } else if  y_possible_new > y_border {
                ball.y_last = y_border + (y_border - ball.y);
                ball.y = y_border - (y_possible_new - y_border) + y_mouse_vec;
            } else {
                ball.y_last = ball.y;
                ball.y = y_possible_new;
            }

            draw_ball(context, ball.x, ball.y, ball.radius, &ball.color);
            
        }
    }

    pub fn add(&mut self) {
        let radius = fastrand::i32(5..50);
        let color = random_color();
        let ball = Ball {x: 50.0, y: 50.0, x_last: 50.0, y_last: 50.0, radius: radius as f64, color};
        self.balls.push(ball);
    }

}


fn random_color() -> String {
    let r = fastrand::i32(1..200);
    let g = fastrand::i32(1..200);
    let b = fastrand::i32(1..200);
    format!("rgb({r},{g},{b})")
}
