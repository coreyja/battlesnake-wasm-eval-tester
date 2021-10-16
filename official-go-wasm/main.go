package main

import (
    "fmt"
    "encoding/json"
    "syscall/js"
    "github.com/BattlesnakeOfficial/rules"
)

func toInt(arg interface {}) int32 {
    return int32(arg.(float64))
}

func toPointArray(arr []interface {}) []rules.Point  {
    var positions []rules.Point;

    for _, f := range arr {
        fMap := f.(map[string]interface{})
        positions = append(positions, rules.Point {
            X: toInt(fMap["x"]),
            Y: toInt(fMap["y"]),
        })
    }

    return positions
}

func boardFromJson(boardStr string) rules.BoardState {
   var object map[string]interface{};
   json.Unmarshal([]byte(boardStr), &object)

    board := object["board"].(map[string]interface{})
    food := board["food"].([]interface {})
    hazards := board["hazards"].([]interface {})

    snakes := board["snakes"].([]interface {})

    var typedSnakes []rules.Snake;
    for _, s := range snakes {
        sMap := s.(map[string]interface{})

        typedSnakes = append(typedSnakes, rules.Snake {
            ID: sMap["id"].(string),
            Health: toInt(sMap["health"]),
            Body: toPointArray(sMap["body"].([]interface {})),
        })
    }

    foodPos := toPointArray(food);


   return rules.BoardState {
       Turn: toInt(object["turn"]),
       Height: toInt(board["height"]),
       Width: toInt(board["width"]),
       Food: foodPos,
       Hazards: toPointArray(hazards),
       Snakes: typedSnakes,
   }
}

func boardToJson(board rules.BoardState) string {
    food := []map[string]int32{}
    for _, f := range board.Food {
        var foodItem = make(map[string]int32)
        foodItem["x"] = f.X
        foodItem["y"] = f.Y

        food = append(food, foodItem)
    }

    hazard := []map[string]int32{}
    for _, f := range board.Hazards {
        var hazardItem = make(map[string]int32)
        hazardItem["x"] = f.X
        hazardItem["y"] = f.Y

        hazard = append(hazard, hazardItem)
    }

    var snakes []map[string]interface{}
    for _, s := range board.Snakes {
        if s.EliminatedCause == rules.NotEliminated {
            var snakeItem = make(map[string]interface{})
            snakeItem["id"] = s.ID
            snakeItem["health"] = s.Health
            snakeItem["death"] = s.EliminatedCause

            var snakeBody []map[string]int32
            for _, b := range s.Body {
                var bodyItem = make(map[string]int32)
                bodyItem["x"] = b.X
                bodyItem["y"] = b.Y

                snakeBody = append(snakeBody, bodyItem)
            }

            snakeItem["body"] = snakeBody

            snakes = append(snakes, snakeItem)
        }
    }

    var boardMap = make(map[string]interface{})
    boardMap["food"] = food
    boardMap["hazards"] = hazard
    boardMap["height"] = board.Height
    boardMap["width"] = board.Width
    boardMap["turn"] = board.Turn
    boardMap["snakes"] = snakes

    str, _ := json.Marshal(boardMap)

    return string(str)
}

func evaluateMoves(this js.Value, args []js.Value) interface{} {
    movesString := args[1].String()
    boardString := args[0].String()

    standard := rules.StandardRuleset{
        FoodSpawnChance:     0,
        MinimumFood:         0,
        HazardDamagePerTurn: 0,
    }

    board := boardFromJson(boardString)

    var moves []rules.SnakeMove;

    constructMovesErr := json.Unmarshal([]byte(movesString), &moves)
    if constructMovesErr != nil {
        fmt.Println("Couldn't moves from input board state", constructMovesErr, args[0].String())
        return ""
    }

    new_board, evalErr := standard.CreateNextBoardState(&board, moves)
    if evalErr != nil {
        fmt.Println("Couldn't evaluateMoves", evalErr)
        return ""
    }

    boardStr := boardToJson(*new_board)
    return string(boardStr)
}

func main() {
    fmt.Println("Program Started")
    c := make(chan struct{}, 0)
    js.Global().Set("evaluateMoves", js.FuncOf(evaluateMoves))
    <-c
}
