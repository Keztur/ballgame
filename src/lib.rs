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
        // let ubound = balls.len();
        
        for i in 0..balls.len() {

            let x_vec = (balls[i].x - balls[i].x_last) * FRICTION;
            let y_vec = (balls[i].y - balls[i].y_last) * FRICTION;
            let x_mouse_vec = x_mouse_vec * FORCE;
            let y_mouse_vec = y_mouse_vec * FORCE;

            let x_possible_new = balls[i].x + x_vec + x_mouse_vec;
            let y_possible_new = balls[i].y + y_vec + y_mouse_vec;

            reflect("x", x_possible_new, &mut balls[i], x_mouse_vec, width);
            reflect("y", y_possible_new, &mut balls[i], y_mouse_vec, height);
            
            // collision(self.balls, balls[i].radius, &mut balls[i].x, &mut balls[i].y);
            
            // for j in 0..ubound {
            //     let distance: f64 = ((balls[i].x - balls[j].x).exp2() + (balls[i].y - balls[j].y).exp2()).sqrt();
                
            //     if distance < balls[i].radius + balls[j].radius {
            //         balls[i].x += 0.1 * (balls[j].x - balls[i].x);
            //         balls[i].y += 0.1 * (balls[j].y - balls[i].y);
        
            //     }
            // }

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

fn reflect(axis: &str, new_pos: f64, ball: &mut Ball, mouse_vec: f64, screen:f64) {
        
    let border = screen - ball.radius;
    let pos;
    let pos_last;

    if axis == "x" {
        pos = &mut ball.x;
        pos_last = &mut ball.x_last;
    } else {
        pos = &mut ball.y;
        pos_last = &mut ball.y_last;
    }

    if new_pos < ball.radius {
        *pos_last = ball.radius + (ball.radius - *pos);
        *pos = ball.radius - (new_pos - ball.radius) + mouse_vec;
    } else if  new_pos > border {
        *pos_last = border + (border - *pos);
        *pos = border - (new_pos - border) + mouse_vec;
    } else {
        *pos_last = *pos;
        *pos = new_pos;
    }

}
    

fn random_color() -> String {
    let r = fastrand::i32(1..200);
    let g = fastrand::i32(1..200);
    let b = fastrand::i32(1..200);
    format!("rgb({r},{g},{b})")
}


// fn collision(balls: Vec<Ball>, radius: f64, x: &mut f64, y: &mut f64)  {

//     for i in 0..balls.len() {
//         let distance: f64 = ((*x - balls[i].x).exp2() + (*y - balls[i].y).exp2()).sqrt();
        
//         if distance < radius + balls[i].radius {
//             *x += 0.1 * (balls[i].x - *x);
//             *y += 0.1 * (balls[i].y - *y);

//         }
//     }
// }
    