use battlesnake_game_types::{
    compact_representation::{CellBoard4Snakes11x11, MoveEvaluatableGame},
    types::{build_snake_id_map, Move},
    wire_representation::Game,
};
use itertools::Itertools;
use serde::Deserialize;
use wasm_bindgen::prelude::*;

#[derive(Deserialize, Debug)]
struct WasmMoves {
    ID: String,
    Move: String,
}

impl WasmMoves {
    fn into_tuple(self) -> (String, Move) {
        let m = match self.Move.as_str() {
            "up" => Move::Up,
            "down" => Move::Down,
            "left" => Move::Left,
            "right" => Move::Right,
            _ => panic!("Invalid Move {}", self.Move),
        };

        (self.ID, m)
    }
}

#[wasm_bindgen]
pub fn evaluateMoves(board: &str, moves: &str) -> String {
    let board: Game = serde_json::from_str(board).unwrap();
    let moves: Vec<WasmMoves> = serde_json::from_str(moves).unwrap();
    let moves = moves.into_iter().map(|m| m.into_tuple()).collect_vec();

    let id_map = build_snake_id_map(&board);
    let moves = moves
        .into_iter()
        .map(|(sid, m)| (*id_map.get(&sid).unwrap(), m))
        .collect_vec();

    let compact = CellBoard4Snakes11x11::convert_from_game(board, &id_map).unwrap();

    println!("Made it to rust!\n{}\n{:?}", compact, moves);

    let new_board = compact.evaluate_moves(&moves);

    format!("Old Board\n{}\n\n\nNew Board!\n{}", compact, new_board)
}
