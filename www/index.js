// Import the WebAssembly memory at the top of the file.
import { memory } from "tetris/tetris_bg";
import { Direction, Tetris } from "tetris";

const CELL_SIZE = 20;   //px
const WIDTH = 10;
const HEIGHT = 20;
const TICK_DELAY = 400;

let tetris = Tetris.new(WIDTH, HEIGHT);

const canvas = document.getElementById("tetris-canvas");
canvas.width = (CELL_SIZE + 2) * WIDTH + 2;
canvas.height = (CELL_SIZE + 2) * HEIGHT + 2;

const ctx = canvas.getContext('2d');

const getIndex = (row, column) => {
  return row * WIDTH + column;
};

const draw = () => {
  const cellsPtr = tetris.draw();
  const cells = new Uint8Array(memory.buffer, cellsPtr, WIDTH * HEIGHT);

  ctx.beginPath();

  for (let row = 0; row < HEIGHT; row++) {
    for (let col = 0; col < WIDTH; col++) {
      const idx = getIndex(row, col);
      ctx.fillStyle = cells[idx] ? "#63b323": "#9b6928";
      ctx.fillRect(
        col * (CELL_SIZE + 2) + 2,
        row * (CELL_SIZE + 2) + 2,
        CELL_SIZE,
        CELL_SIZE
      )
    }
  }

  ctx.stroke();
}

const restart = () => {
  tetris = Tetris.new(WIDTH, HEIGHT);
}

let tick_delay = TICK_DELAY;
let last_tick = null;

const renderLoop = (timestamp) => {
  if (!last_tick) {
    last_tick = timestamp;
  }

  let progress = timestamp - last_tick;

  if (progress > tick_delay) {
    last_tick = timestamp;
    let status = tetris.tick();
    
    if(!status) restart()
  }

  draw();
  requestAnimationFrame(renderLoop);
}

document.addEventListener("keydown", e => {
  switch (e.code) {
    case "ArrowLeft":
      tetris.shift(Direction.Left);
      break;
    case "ArrowRight":
      tetris.shift(Direction.Right);
      break;
    case "ArrowUp":
      tetris.rotate();
      break;
    case "ArrowDown":
      tick_delay = TICK_DELAY/10;
      break;
    default:
      break;
  }
});


document.addEventListener('keyup', (e) => {
  if(e.code == "ArrowDown") {
    tick_delay = TICK_DELAY;
  }
});

renderLoop()
