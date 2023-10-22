import { Universe, Cell } from "../../../rust/pkg/wasm_game_of_life";
import { memory } from "../../../rust/pkg/wasm_game_of_life_bg.wasm";

const CELL_SIZE = 10; // px
const GRID_COLOR = "#e5e5e5";
const DEAD_COLOR = "#FFFFFF";
const ALIVE_COLOR = "#000000";

// Construct the universe, and get its width and height.
const vw = Math.max(
  document.documentElement.clientWidth || 0,
  window.innerWidth || 0
);
const vh = Math.max(
  document.documentElement.clientHeight || 0,
  window.innerHeight || 0
);
let width = Math.floor(vw / (CELL_SIZE + 1));
let height = Math.floor(vh / (CELL_SIZE + 1));
const universe = Universe.new(width, height);

window.addEventListener("resize", (event) => {
  const vw = Math.max(
    document.documentElement.clientWidth || 0,
    window.innerWidth || 0
  );
  const vh = Math.max(
    document.documentElement.clientHeight || 0,
    window.innerHeight || 0
  );
  width = Math.floor(vw / (CELL_SIZE + 1));
  height = Math.floor(vh / (CELL_SIZE + 1));
  universe.resize(width, height);
  canvas.height = height * (CELL_SIZE + 1) + 1;
  canvas.width = width * (CELL_SIZE + 1) + 1;
  pause();
});

// Give the canvas room for all of our cells and a 1px border
// around each of them.
const canvas = document.getElementById("game-of-life-canvas");
canvas.height = height * (CELL_SIZE + 1) + 1;
canvas.width = width * (CELL_SIZE + 1) + 1;

const ctx = canvas.getContext("2d");

let animationId = null;

const renderLoop = () => {
  drawGrid();
  drawCells();

  universe.tick();

  animationId = requestAnimationFrame(renderLoop);
};

const isPaused = () => {
  return animationId === null;
};

const playPauseButton = document.getElementById("play-pause");
const playPauseIcon = document.getElementById("play-pause-icon");

const play = () => {
  playPauseIcon.className = "bi bi-pause-fill";
  renderLoop();
};

const pause = () => {
  playPauseIcon.className = "bi bi-play-fill";
  drawGrid();
  drawCells();
  cancelAnimationFrame(animationId);
  animationId = null;
};

playPauseButton.addEventListener("click", (event) => {
  if (isPaused()) {
    play();
  } else {
    pause();
  }
});

const resetButton = document.getElementById("reset");
resetButton.addEventListener("click", (event) => {
  universe.reset();
  pause();
  drawGrid();
  drawCells();
});

const randomButton = document.getElementById("random");
randomButton.addEventListener("click", (event) => {
  universe.random();
  drawGrid();
  drawCells();
});

const speedSlider = document.getElementById("speed");
speedSlider.addEventListener("input", (event) => {
  universe.set_speed(event.target.value);
});

canvas.addEventListener("click", (event) => {
  const boundingRect = canvas.getBoundingClientRect();

  const scaleX = canvas.width / boundingRect.width;
  const scaleY = canvas.height / boundingRect.height;

  const canvasLeft = (event.clientX - boundingRect.left) * scaleX;
  const canvasTop = (event.clientY - boundingRect.top) * scaleY;

  const row = Math.min(Math.floor(canvasTop / (CELL_SIZE + 1)), height - 1);
  const col = Math.min(Math.floor(canvasLeft / (CELL_SIZE + 1)), width - 1);

  universe.toggle_cell(row, col);

  drawGrid();
  drawCells();
});

const drawGrid = () => {
  ctx.beginPath();
  ctx.strokeStyle = GRID_COLOR;

  // Vertical lines.
  for (let i = 0; i <= width; i++) {
    ctx.moveTo(i * (CELL_SIZE + 1) + 1, 0);
    ctx.lineTo(i * (CELL_SIZE + 1) + 1, (CELL_SIZE + 1) * height + 1);
  }

  // Horizontal lines.
  for (let j = 0; j <= height; j++) {
    ctx.moveTo(0, j * (CELL_SIZE + 1) + 1);
    ctx.lineTo((CELL_SIZE + 1) * width + 1, j * (CELL_SIZE + 1) + 1);
  }

  ctx.stroke();
};

const getIndex = (row, column) => {
  return row * width + column;
};

const drawCells = () => {
  const cellsPtr = universe.cells();
  const cells = new Uint8Array(memory.buffer, cellsPtr, width * height);

  ctx.beginPath();

  for (let row = 0; row < height; row++) {
    for (let col = 0; col < width; col++) {
      const idx = getIndex(row, col);

      ctx.fillStyle = cells[idx] === Cell.Dead ? DEAD_COLOR : ALIVE_COLOR;

      ctx.fillRect(
        col * (CELL_SIZE + 1) + 1,
        row * (CELL_SIZE + 1) + 1,
        CELL_SIZE,
        CELL_SIZE
      );
    }
  }

  ctx.stroke();
};

drawGrid();
drawCells();
play();
