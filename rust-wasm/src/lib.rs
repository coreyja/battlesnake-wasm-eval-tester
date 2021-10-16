use wasm_bindgen::prelude::*;

use std::collections::{HashMap, VecDeque};

use battlesnake_game_types::{
    compact_representation::{CellBoard4Snakes11x11, MoveEvaluatableGame},
    types::*,
    wire_representation::{BattleSnake, Board, Game, NestedGame, Position, Ruleset, Settings},
};
use itertools::Itertools;
use serde::Deserialize;

use rand::seq::IteratorRandom;
use rand::{prelude::*, thread_rng, Rng};

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    // The `console.log` is quite polymorphic, so we can bind it with multiple
    // signatures. Note that we need to use `js_name` to ensure we always call
    // `log` in JS.
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    // Multiple arguments too!
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}

macro_rules! console_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[derive(Deserialize, Debug)]
struct WasmMoves {
    ID: String,
    Move: String,
}

impl WasmMoves {
    fn into_tuple(self) -> (String, Move) {
        let m = match self.Move.as_str() {
            "Up" | "up" => Move::Up,
            "Down" | "down" => Move::Down,
            "Left" | "left" => Move::Left,
            "Right" | "right" => Move::Right,
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
        .filter(|(_name, sid)| compact.is_alive(sid))
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
    let you_id = id_map.iter().find(|(k, v)| v.0 == 0).map(|x| x.0).unwrap();
    let you = snakes
        .iter()
        .find(|s| &s.id == you_id)
        .cloned()
        .unwrap_or_else(|| BattleSnake {
            id: you_id.to_owned(),
            head: Position { x: 0, y: 0 },
            actual_length: None,
            health: 0,
            body: VecDeque::new(),
            name: you_id.to_owned(),
            shout: None,
        });

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

fn random_square_for_head(rng: &mut ThreadRng, g: &Game) -> Option<Position> {
    let width_range = (0..g.get_width()).collect_vec();
    let height_range = (0..g.get_height()).collect_vec();
    let ranges = [width_range, height_range];
    let multi = ranges.iter().multi_cartesian_product();
    multi
        .map(|pos| Position {
            x: *pos[0] as i32,
            y: *pos[1] as i32,
        })
        .filter(|p| !g.position_is_snake_body(*p))
        .choose(rng)
}

fn random_snake(rng: &mut ThreadRng, id: &str, g: &Game) -> Option<BattleSnake> {
    let health = rng.gen_range(1..=100);
    let length: i32 = rng.gen_range(3..20);

    let head = random_square_for_head(rng, g)?;

    let mut body: VecDeque<Position> = VecDeque::with_capacity(length as usize);
    body.push_front(head);

    while body.len() < length as usize {
        if let Some(next_body) = g
            .neighbors(body.back().unwrap())
            .into_iter()
            .filter(|p| !body.contains(p) && !g.position_is_snake_body(*p))
            .choose(rng)
        {
            body.push_back(next_body);
        } else {
            break;
        }
    }

    if body.len() < 3 {
        return None;
    }

    Some(BattleSnake {
        id: id.to_owned(),
        name: id.to_owned(),
        health,
        actual_length: Some(length),
        shout: None,
        head,
        body,
    })
}

fn random_game() -> Game {
    let mut rng = thread_rng();

    let nested_game = NestedGame {
        id: "faked".to_owned(),
        ruleset: Ruleset {
            name: "standard".to_owned(),
            version: "1".to_owned(),
            settings: None,
        },
    };

    let mut game = Game {
        game: nested_game,
        turn: 0,
        board: Board {
            width: 11,
            height: 11,
            food: vec![],
            hazards: vec![],
            snakes: vec![],
        },
        you: BattleSnake {
            id: "".to_owned(),
            body: VecDeque::new(),
            actual_length: None,
            health: 0,
            name: "".to_owned(),
            shout: None,
            head: Position { x: 0, y: 0 },
        },
    };

    let number_of_snakes: i8 = rng.gen_range(1..=4);
    // let number_of_snakes: i8 = 2;

    for i in 0..number_of_snakes {
        if let Some(s) = random_snake(&mut rng, &format!("{}", i), &game) {
            game.board.snakes.push(s);
        } else {
            break;
        }
    }

    if let Some(you) = game.board.snakes.get(0) {
        game.you = you.clone();
    }

    game
}

#[wasm_bindgen]
pub fn randomGame() -> String {
    let game = random_game();

    serde_json::to_string(&game).unwrap()
}

#[wasm_bindgen]
pub fn displayGame(board: &str) -> String {
    let g: Game = serde_json::from_str(board).unwrap();
    let id_map = build_snake_id_map(&g);
    let compact = CellBoard4Snakes11x11::convert_from_game(g, &id_map).unwrap();
    format!("{}", compact)
}

#[wasm_bindgen]
pub fn evaluateMoves(board: &str, moves: &str) -> String {
    console_error_panic_hook::set_once();

    let board: Game = serde_json::from_str(board).unwrap();
    let moves: Vec<WasmMoves> = serde_json::from_str(moves).unwrap();
    let moves = moves.into_iter().map(|m| m.into_tuple()).collect_vec();

    let id_map = build_snake_id_map(&board);
    let moves = moves
        .into_iter()
        .map(|(sid, m)| (*id_map.get(&sid).unwrap(), m))
        .collect_vec();

    let compact = CellBoard4Snakes11x11::convert_from_game(board, &id_map).unwrap();

    let new_board = compact.evaluate_moves(&moves);

    let new_game = build_game_from_compact(&new_board, &id_map);

    let new_game_json = serde_json::to_string(&new_game).unwrap();

    new_game_json
}
