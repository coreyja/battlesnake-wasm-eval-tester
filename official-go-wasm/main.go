package main

import (
    "fmt"
    "syscall/js"
)

func sayHello(this js.Value, args []js.Value) interface{} {
    fmt.Println("Hello ", args[0])
    return "FooBar"
}

func main() {
    fmt.Println("Program Started")
    c := make(chan struct{}, 0)
    js.Global().Set("sayHello", js.FuncOf(sayHello))
    <-c
}
