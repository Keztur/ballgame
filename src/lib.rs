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

        let balls = &mut self.balls;
        let ubound = balls.len();
        
        for i in 0..balls.len() {

            let x_vec = (balls[i].x - balls[i].x_last) * FRICTION;
            let y_vec = (balls[i].y - balls[i].y_last) * FRICTION;
            let x_mouse_vec = x_mouse_vec * FORCE;
            let y_mouse_vec = y_mouse_vec * FORCE;

            balls[i].x_last = balls[i].x;
            balls[i].y_last = balls[i].y;
            balls[i].x += x_vec + x_mouse_vec;
            balls[i].y += y_vec + y_mouse_vec;
            
        }

        for i in 0..balls.len() {
            for j in (i+1)..ubound {
                if i != j {   
                    
                    let distance: f64 = (balls[i].x - balls[j].x).hypot(balls[i].y - balls[j].y);
    
                    let intersection = balls[i].radius + balls[j].radius - distance;

                    if intersection > 0.0 {

                        // balls[i].x_last = balls[i].x;
                        // balls[i].y_last = balls[i].y;
                        
                        // balls[j].x_last = balls[j].x;
                        // balls[j].y_last = balls[j].y;

                        let x_delta = (balls[i].x - balls[i].x_last) + (balls[j].x - balls[j].x_last);
                        let y_delta = (balls[i].y - balls[i].y_last) + (balls[j].y - balls[j].y_last);

                        balls[i].x = balls[i].x_last + x_delta;
                        balls[i].y = balls[i].y_last + y_delta;

                        balls[j].x = balls[j].x_last - x_delta;
                        balls[j].y = balls[j].y_last - y_delta;

                        //invert direction (funny)
                        // let mut last = balls[i].x_last;
                        // balls[i].x_last = balls[i].x;
                        // balls[i].x = last;
                        // last = balls[i].y_last;
                        // balls[i].y_last = balls[i].y;
                        // balls[i].y = last;

                        // last = balls[j].x_last;
                        // balls[j].x_last = balls[j].x;
                        // balls[j].x = last;
                        // last = balls[j].y_last;
                        // balls[j].y_last = balls[j].y;
                        // balls[j].y = last;
                    }
    
                }
            }
        }

        for i in 0..balls.len() {

            (balls[i].x, balls[i].x_last) = 
            reflect(balls[i].x_last, balls[i].x, balls[i].radius, width - balls[i].radius);

            (balls[i].y, balls[i].y_last) = 
            reflect(balls[i].y_last, balls[i].y, balls[i].radius, height - balls[i].radius);

            draw_ball(context, balls[i].x, balls[i].y, balls[i].radius, &balls[i].color);
            
        }

    }

    pub fn add(&mut self) {
        let radius = fastrand::i32(5..50);
        let color = random_color();
        let ball = Ball {x: 50.0, y: 50.0, x_last: 50.0, y_last: 50.0, radius: radius as f64, color};
        self.balls.push(ball);
    }

}

fn reflect(mut pos: f64, new_pos: f64, radius: f64, border:f64) -> (f64, f64) {

    let pos_last: f64;    

    if new_pos < radius {           //collision with low border (left or up)
        pos_last = radius + (radius - pos);
        pos = radius - (new_pos - radius);
    } else if  new_pos > border {   //collision with high border (right or bottom)
        pos_last = border + (border - pos);
        pos = border - (new_pos - border);
    } else {                        //no collision
        pos_last = pos;
        pos = new_pos;
    }

    (pos, pos_last)    
}
    

fn random_color() -> String {
    let r = fastrand::i32(1..200);
    let g = fastrand::i32(1..200);
    let b = fastrand::i32(1..200);
    format!("rgb({r},{g},{b})")
}



    