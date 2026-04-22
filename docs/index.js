//@ts-check

// 1. Import the WASM glue code from your pkg folder
/** @type {import('./try_rs_tictactoe')} */
import init, { solve_tictactoe, check_game_status } from "./try_rs_tictactoe.js";

const levels = ["easy", "medium", "hard"];
let level = levels[Math.floor(Math.random() * levels.length)];

let boardState = ["", "", "", "", "", "", "", "", ""];
let gameOver = false;

const levelEl = document.getElementById("level");
const boardEl = document.getElementById("board");
const statusEl = document.getElementById("status");
const resetBtnEl = document.getElementById("reset");

if (levelEl) levelEl.innerText = level;

if (resetBtnEl) resetBtnEl.onclick = resetGame;

function getBoardEl() {
  if (!boardEl) throw new Error("Board element not found!");
  return boardEl;
}

function getStatusEl() {
  if (!statusEl) throw new Error("Status element not found!");
  return statusEl;
}

async function start() {
  // Initialize the WASM module
  await init();
  renderBoard();
}

function renderBoard() {
  const boardEl = getBoardEl();

  boardEl.innerHTML = "";

  boardState.forEach((cell, i) => {
    const btn = document.createElement("button");
    btn.className =
      "w-24 h-24 bg-slate-800 flex items-center justify-center text-4xl font-bold rounded hover:bg-slate-600 transition-colors";
    btn.innerText = cell;
    btn.onclick = () => handleMove(i);
    boardEl.appendChild(btn);
  });
}

/**
 * @param {number} index
 */
async function handleMove(index) {
  if (boardState[index] !== "" || gameOver) return;

  // Human Move (X)
  boardState[index] = "X";
  renderBoard();

  const statusEl = getStatusEl();

  // AI Move (O)
  statusEl.innerText = "AI is thinking...";

  // 2. Convert current state to your Rust "3/3/3 x" format
  const rustString = convertToRustString("o");

  // 3. Call your Rust Engine!
  // solve_tictactoe returns [row, col]
  const [row, col] = solve_tictactoe(rustString, "hard");

  const aiIndex = row * 3 + col;
  boardState[aiIndex] = "O";

  renderBoard();
  statusEl.innerText = "Your Turn (X)";

  await syncStatus(); // Check if AI won
}

/**
 * @param {string} nextPlayer
 */
function convertToRustString(nextPlayer) {
  // This transforms the flat JS array into your "3/1o1/3 x" format
  let rows = [];
  for (let i = 0; i < 9; i += 3) {
    let rowStr = "";
    let empty = 0;
    for (let j = 0; j < 3; j++) {
      const val = boardState[i + j];
      if (val === "") {
        empty++;
      } else {
        if (empty > 0) rowStr += empty;
        rowStr += val.toLowerCase();
        empty = 0;
      }
    }
    if (empty > 0) rowStr += empty;
    rows.push(rowStr);
  }
  return `${rows.join("/")} ${nextPlayer}`;
}

async function syncStatus() {
  // We send the state to Rust to see if the game is over
  const status = check_game_status(convertToRustString("x"));
  const statusEl = getStatusEl();

  if (status === "X_WINS") {
    statusEl.innerText = "🎉 You Won!";
    alert("Victory! You beat the machine!");
    gameOver = true;
  } else if (status === "O_WINS") {
    statusEl.innerText = "🤖 AI Won!";
    alert("Defeat! The AI outsmarted you.");
    gameOver = true;
  } else if (status === "DRAW") {
    statusEl.innerText = "🤝 It's a Draw!";
    alert("It's a tie!");
    gameOver = true;
  } else {
    statusEl.innerText = "Your Turn (X)";
    return false;
  }
  return true;
}

function resetGame() {
  level = levels[Math.floor(Math.random() * levels.length)];
  if (levelEl) levelEl.innerText = level;
  boardState = ["", "", "", "", "", "", "", "", ""];
  gameOver = false;
  const statusEl = getStatusEl();
  statusEl.innerText = "Your Turn (X)";
  renderBoard();
}

start();
