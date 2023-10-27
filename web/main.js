import init, { create_game } from "../pkg/crab_gl.js";

const CANVAS_ID = "triangle";
const FIXED_TIMESTEP = 0.01;
let renderCanvas = document.getElementById(CANVAS_ID);
let gameState;
let date;
let accumulator = 0.;

init().then( () => {
    date = new Date();
    gameState = create_game(CANVAS_ID);
    renderCanvas.addEventListener("click", e =>
        gameState.click(e.offsetX, e.offsetY));
    requestAnimationFrame(onFrame)
});

function onFrame() {
    requestAnimationFrame(onFrame);
    let now = new Date();
    let elapsed = (now - date) / 1000;
    accumulator += elapsed;
    while (accumulator >= FIXED_TIMESTEP) {
        gameState.on_frame(FIXED_TIMESTEP);
        accumulator -= FIXED_TIMESTEP;
    }
    date = now;
}

const colorChangerForm = document.getElementById("color-changer");
colorChangerForm.addEventListener("submit", (e) => {
  e.preventDefault();
  const color = [
    clampRGBValue(e.target.elements.red.value),
    clampRGBValue(e.target.elements.green.value),
    clampRGBValue(e.target.elements.blue.value),
    1.0,
  ];
});

function clampRGBValue(value) {
  return parseFloat((parseFloat(value) / 255 || 0).toFixed(2));
}