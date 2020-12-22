package main

import (
    "fmt"
)

func foo() (int, int) {
    return 1, 2
}

func bar(a, b int) int {
    return a + b
}

func main() {
    fmt.Println(bar(foo()))
}
