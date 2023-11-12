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
pub const BOUNCE: f64 = 0.4;
pub const TRANSFER: f64 = 0.4;


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
   pub x_vec: f64,
   pub y_vec: f64,
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

            let x_vec = (balls[i].x_vec * FRICTION) + (x_mouse_vec * FORCE);
            let y_vec = (balls[i].y_vec * FRICTION) + (y_mouse_vec * FORCE);

            balls[i].x_vec = x_vec;
            balls[i].y_vec = y_vec;

            balls[i].x += x_vec;
            balls[i].y += y_vec;
            
        }

        for i in 0..balls.len() {

            for j in (i+1)..ubound {
                
                let distance: f64 = (balls[i].x - balls[j].x).hypot(balls[i].y - balls[j].y);

                let intersection = balls[i].radius + balls[j].radius - distance;

                if intersection > 0.0 {
                    
                    //reset ball to last position before collision
                    balls[i].x = balls[i].x - balls[i].x_vec;
                    balls[i].y = balls[i].y - balls[i].y_vec;
                    balls[j].x = balls[j].x - balls[j].x_vec;
                    balls[j].y = balls[j].y - balls[j].y_vec;

                    //calculate collision vector
                    let mut colvec_x = balls[i].x - balls[j].x;
                    let mut colvec_y = balls[i].y - balls[j].y;

                    //get amount of collision vector
                    let colvec_amount = (colvec_x).hypot(colvec_y).abs();
                    //normalize collision vector
                    colvec_x = colvec_x / colvec_amount;
                    colvec_y = colvec_y / colvec_amount;

                    //get amount auf (velocity) vector for each ball
                    let ball_i_amount = (balls[i].x_vec).hypot(balls[i].y_vec);
                    let ball_j_amount = (balls[j].x_vec).hypot(balls[j].y_vec);

                    //save new vector
                    balls[i].x_vec += (colvec_x * ball_i_amount) * BOUNCE + (colvec_x * ball_j_amount) * TRANSFER;
                    balls[i].y_vec += (colvec_y * ball_i_amount) * BOUNCE + (colvec_y * ball_j_amount) * TRANSFER;
                    balls[j].x_vec -= (colvec_x * ball_j_amount) * BOUNCE + (colvec_x * ball_i_amount) * TRANSFER;
                    balls[j].y_vec -= (colvec_y * ball_j_amount) * BOUNCE + (colvec_y * ball_i_amount) * TRANSFER;

                    //set new ball positions (with new vector)
                    balls[i].x += balls[i].x_vec;
                    balls[i].y += balls[i].y_vec;
                    balls[j].x += balls[j].x_vec;
                    balls[j].y += balls[j].y_vec;

                }
            }
        }

        for i in 0..balls.len() {

            (balls[i].x, balls[i].x_vec) = 
            reflect(balls[i].x, balls[i].x_vec, balls[i].radius, width - balls[i].radius);

            (balls[i].y, balls[i].y_vec) = 
            reflect(balls[i].y, balls[i].y_vec, balls[i].radius, height - balls[i].radius);

            draw_ball(context, balls[i].x, balls[i].y, balls[i].radius, &balls[i].color);
            
        }

    }

    pub fn add(&mut self) {
        let radius = fastrand::i32(5..50) as f64;
        let color = random_color();
        let ball_count = self.balls.len();
        let values: [f64; 4];

        match ball_count {
            0 => values = [700.0, 200.0, 0.0, 0.0],
            1 => values = [100.0, 200.0, 7.0, 0.0],
            2 => values = [700.0, 300.0, -7.0, 0.0],
            3 => values = [100.0, 300.0, 5.0, 0.0],
            _=> values = [50.0, 50.0, 5.0, 5.0],
        }

        let ball = Ball {x: values[0], y: values[1], x_vec: values[2], y_vec: values[3], radius, color};
        self.balls.push(ball);
    }
    // pub fn add(&mut self) {
    //     let radius = fastrand::i32(5..50) as f64;
    //     let color = random_color();
    //     let x_vec = fastrand::i32(1..15) as f64;
    //     let y_vec = fastrand::i32(1..15) as f64;
    //     let ball = Ball {x: 50.0, y: 50.0, x_vec, y_vec, radius, color};
    //     self.balls.push(ball);
    // }

}

fn reflect(mut pos: f64, mut vec: f64, radius: f64, border:f64) -> (f64, f64) {

    if pos < radius {           //collision with low border (left or up)
        pos = radius - (pos - radius);
        vec = -vec;
    } else if  pos > border {   //collision with high border (right or bottom)
        pos = border - (pos - border);
        vec = -vec;
    } else {                    //no collision
        pos = pos;
    }

    (pos, vec)    
}
    

fn random_color() -> String {
    let r = fastrand::i32(1..200);
    let g = fastrand::i32(1..200);
    let b = fastrand::i32(1..200);
    format!("rgb({r},{g},{b})")
}



    