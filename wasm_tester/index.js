import init, {evaluateMoves as rustEval} from "../wasm/rust/rust_wasm.js";
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
  // if (!isEqual(a.hazards.sort(coordCompare), b.hazards.sort(coordCompare))) {
  //   return false;
  // }

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
  let board = document.getElementById("board").value;
  let moves = document.getElementById("moves").value;

  let goResult = JSON.parse(window.goEval(board, moves));
  let rustResult = JSON.parse(rustEval(board, moves));

  console.log(compareBoards(goResult, rustResult.board));
}

document.addEventListener("DOMContentLoaded", function () {
  // Your code goes here
  document.getElementById("go-btn").addEventListener("click", clickedGoEval);
  document
    .getElementById("rust-btn")
    .addEventListener("click", clickedRustEval);
  document
    .getElementById("compare-btn")
    .addEventListener("click", clickedCompare);
});

