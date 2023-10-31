import init, { run_sim, add_ball } from "../pkg/ballgame.js";

const canvas = document.getElementById("myCanvas") as HTMLCanvasElement;
const ctx = canvas.getContext("2d")!;

let x_mouse = 0;
let y_mouse = 0;
let x_lastmouse = 0;
let y_lastmouse = 0;

let width = 10
let height = 10

resizeCanvas()

// let field = [0, 0]

// let forcePos = [{x: 314, y: 384}, {x: 628, y: 384}]
// let mouseDown = false

// let OffsetX = 0
// let OffsetY = 0

// start game render cycle
init().then(() => {
    setInterval(simulation, 10)
})


// ### FUNCTIONS ######################################
function simulation() {

    ctx.clearRect(0, 0, canvas.width, canvas.height)
  
    let xMouseVec = x_mouse - x_lastmouse
    let yMouseVec = y_mouse - y_lastmouse

    run_sim(xMouseVec, yMouseVec, width, height, ctx)

    x_lastmouse = x_mouse
    y_lastmouse = y_mouse

}



// function drawCenter(color: string, x: number, y: number, size: number) {
//     ctx.beginPath();
//     ctx.arc(x, y, size, 0, Math.PI*2);
//     ctx.fillStyle = color;
//     ctx.fill();
//     ctx.closePath();
// }


// function inRadius(ax:number, bx:number, ay:number, by:number, radius:number) {

//     let distance = Math.sqrt(Math.pow(ax-bx, 2) + Math.pow(ay-by, 2))
//     // console.log(Math.round(distance))
//     return  distance < radius

// }

// function ForceField(center_x:number, center_y:number, x_new:number, y_new:number, x:number, y:number, center_force:number, size:number) {

//     drawCenter('#539CAE', center_x, center_y, size)

//     if (inRadius(x, center_x, y, center_y, size)) {
//         x_new += center_force * (center_x-x)
//         y_new += center_force * (center_y-y)
//     }

//     return [x_new, y_new]
// }


// function reflect() {

//     if (x < ballRadius || x > width - ballRadius) {
//         x_invert = -1
//     } else {
//         x_invert = 1
//     }
    
//     if (y < ballRadius || y > height - ballRadius) {
//         y_invert = -1
//     } else {
//         y_invert = 1
//     }

// }


// move ball
window.addEventListener("mousemove", (evt) => {
   
    x_mouse = evt.clientX
    y_mouse = evt.clientY

}, false);

window.addEventListener('resize', resizeCanvas, false);

function resizeCanvas() {
    canvas.width = window.innerWidth
    canvas.height = window.innerHeight

    width = canvas.width
    height = canvas.height
}

window.addEventListener("keydown", (event) => {
    if (event.key === ' ') {
        add_ball()
        let xMouseVec = Math.random() * 30
        let yMouseVec = Math.random() * 30
        run_sim(xMouseVec, yMouseVec, width, height, ctx)
    }
});

  
// window.addEventListener("mousemove", (evt) => {
   
//     let mousePos = getMousePos(canvas, evt);

//     if (mouseDown) {
//         if (inRadius(mousePos.x, forcePos[0].x, mousePos.y, forcePos[0].y, 50)) {
//             forcePos[0].x = mousePos.x + OffsetX
//             forcePos[0].y = mousePos.y + OffsetY  
//         } else if (inRadius(mousePos.x, forcePos[1].x, mousePos.y, forcePos[1].y, 50)) {
//             forcePos[1].x = mousePos.x + OffsetX
//             forcePos[1].y = mousePos.y + OffsetY  
//         } 
//     } 

//     x_lastmouse = x_mouse
//     y_lastmouse = y_mouse
    
//     x_mouse = evt.clientX
//     y_mouse = evt.clientY


// }, false);



// canvas.addEventListener("mousedown", (evt) => {

//     let mousePos = getMousePos(canvas, evt);
    
//     if (inRadius(mousePos.x, forcePos[0].x, mousePos.y, forcePos[0].y, 50)) {
//         OffsetX = forcePos[0].x - mousePos.x
//         OffsetY = forcePos[0].y - mousePos.y
//     } else if (inRadius(mousePos.x, forcePos[1].x, mousePos.y, forcePos[1].y, 50)) {
//         OffsetX = forcePos[1].x - mousePos.x
//         OffsetY = forcePos[1].y - mousePos.y
//     } 

//     mouseDown = true

// }, false);

// canvas.addEventListener("mouseup", function (evt) {
//     mouseDown = false
// }, false);

// //Get Mouse Position
// function getMousePos(canvas: HTMLCanvasElement, evt:MouseEvent) {
//     let rect = canvas.getBoundingClientRect();
//     return {
//         x: evt.clientX - rect.left,
//         y: evt.clientY - rect.top
//     };
// }