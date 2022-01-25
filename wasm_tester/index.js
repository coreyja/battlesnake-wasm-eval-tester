import init, {evaluateMovesWire as rustEval, displayGame, randomGame } from "../wasm/rust/rust_wasm.js";
init("../wasm/rust/rust_wasm_bg.wasm")

var isEqual = require("lodash/isEqual");
var sortBy = require("lodash/sortBy");
const zip = require("lodash/zip");
const map = require('lodash/map');

function coordCompare(a, b) {
  return a.x - b.x + (a.y - b.y) * 100;
}

function compareBoards(a, b) {
  if (!isEqual(a.food.sort(coordCompare), b.food.sort(coordCompare))) {
    return false;
  }
  if (!isEqual(a.hazards.sort(coordCompare), b.hazards.sort(coordCompare))) {
    return false;
  }

  let aSnakesSorted = sortBy(a.snakes, ["id"]);
  let bSnakesSorted = sortBy(b.snakes, ["id"]);
  let zippedSnakes = zip(aSnakesSorted, bSnakesSorted);


  for (let i = 0; i < zippedSnakes.length; i++) {
    const [snakeA, snakeB] = zippedSnakes[i];

    if (snakeA.health != snakeB.health) return false;
    if (!isEqual(snakeA.body.sort(coordCompare), snakeB.body.sort(coordCompare))) return false;

  }

  return true;
}

function clickedGoEval() {
  let board = document.getElementById("board").value;
  let moves = document.getElementById("moves").value;

  let result = window.goEval(board, moves);

  document.getElementById("result").value = result;
}

function clickedRustEval() {
  let board = document.getElementById("board").value;
  let moves = document.getElementById("moves").value;

  let result = rustEval(board, moves);

  document.getElementById("result").value = result;
}

function clickedCompare() {
  try {
    let board = document.getElementById("board").value;
    let moves = document.getElementById("moves").value;

    document.getElementById("before").value = displayGame(board);

    const rawGoResult = window.goEval(board, moves);
    let goResult = JSON.parse(rawGoResult);
    document.getElementById("goResult").value = JSON.stringify(goResult, null, 4);

    const rustRawResult = rustEval(board, moves);
    let rustResult = JSON.parse(rustRawResult);
    document.getElementById("rustResult").value = displayGame(rustRawResult);

    const result = compareBoards(goResult, rustResult.board);

    if (result) {
      document.body.style.background = '#00FF00';
    } else {
      document.body.style.background = '#ff0000';
    }

    return result;
  } catch (e) {
    document.body.style.background = '#ff0000';
    throw e;
  }
}

const MOVES = ['up', 'down', 'left', 'right'];

const randomMove = () => {
  const chosen_move_index = Math.floor(Math.random() * 4);
  const chosen_move = MOVES[chosen_move_index];

  return chosen_move;
};

const singleFuzz = () => {
    const count = parseInt(document.getElementById("counter").innerHTML);

    const r = randomGame();
    const game = JSON.parse(r);
    const moves = game.board.snakes.map((x) => x.id).map((id) => ({ ID: id, Move: randomMove()}))

    document.getElementById("board").value = r;
    document.getElementById("moves").value = JSON.stringify(moves);
    document.getElementById("before").value = ""
    document.getElementById("rustResult").value = ""
    document.getElementById("goResult").value = ""

    const result = clickedCompare();

    document.getElementById("counter").innerHTML = (count + 1);

    return result;
};

function doSingleFuzzInBackground() {
    setTimeout(() => {
        let result = singleFuzz();

      if (result) {
        doSingleFuzzInBackground()
      }
    },
      0);
}

document.addEventListener("DOMContentLoaded", function () {
  document.getElementById("go-btn").addEventListener("click", clickedGoEval);
  document
    .getElementById("rust-btn")
    .addEventListener("click", clickedRustEval);
  document
    .getElementById("compare-btn")
    .addEventListener("click", clickedCompare);

  document.getElementById("gen-random-game").addEventListener("click", () => {
    const r = randomGame();

    document.getElementById("board").value = r;
    document.getElementById("result").value = displayGame(r);
  });

  document.getElementById("gen-random-moves").addEventListener("click", () => {
    const game = JSON.parse(document.getElementById("board").value);

    const moves = game.board.snakes.map((x) => x.id).map((id) => ({ ID: id, Move: randomMove()}))

    document.getElementById("moves").value = JSON.stringify(moves);
  });

  document.getElementById("run-fuzz").addEventListener("click", () => {
    singleFuzz();
  });

  document.getElementById("run-fuzzes").addEventListener("click", () => {
    doSingleFuzzInBackground();
  });
});



