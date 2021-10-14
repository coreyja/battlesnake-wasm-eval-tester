use std::collections::HashMap;

use battlesnake_game_types::{
    compact_representation::{CellBoard4Snakes11x11, MoveEvaluatableGame},
    types::{
        build_snake_id_map, FoodGettableGame, HealthGettableGame, Move, SnakeBodyGettableGame,
        SnakeId,
    },
    wire_representation::{BattleSnake, Board, Game, NestedGame, Ruleset, Settings},
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

fn build_game_from_compact(
    compact: &CellBoard4Snakes11x11,
    id_map: &HashMap<String, SnakeId>,
) -> Game {
    let snakes: Vec<BattleSnake> = id_map
        .iter()
        .map(|(name, sid)| {
            let body = compact
                .get_snake_body_vec(sid)
                .into_iter()
                .map(|p| p.into_position(CellBoard4Snakes11x11::width()))
                .collect_vec();

            BattleSnake {
                id: name.to_string(),
                head: body.first().unwrap().clone(),
                actual_length: Some(body.len() as i32),
                health: compact.get_health_i64(sid) as i32,
                body: body.into(),
                name: name.to_string(),
                shout: None,
            }
        })
        .collect_vec();
    let you_id = id_map.iter().find(|(k, v)| v.0 == 0).unwrap().0;
    let you = snakes.iter().find(|s| &s.id == you_id).unwrap().clone();

    let height = 11;
    let width = 11;

    let food = compact.get_all_food_as_positions();
    let hazards = compact.get_all_hazards_as_positions();

    let board = Board {
        snakes,
        height,
        width,
        food,
        hazards,
    };

    Game {
        board,
        you,
        turn: 0,
        game: NestedGame {
            id: "faked".to_owned(),
            ruleset: Ruleset {
                name: "standard".to_owned(),
                version: "1".to_owned(),
                settings: None,
            },
        },
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

    let new_game = build_game_from_compact(&new_board, &id_map);

    let new_game_json = serde_json::to_string(&new_game).unwrap();

    new_game_json
}
