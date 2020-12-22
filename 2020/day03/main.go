package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
)

type Algorithm struct {
	right uint
	down  uint
}

func run_algorithm(m []string, alg Algorithm) uint {
	var right uint = 0
	var down uint = 0
	var tree_count uint = 0
	for down < uint(len(m)) {
		if m[down][right%uint(len(m[down]))] == '#' {
			tree_count += 1
		}
		down += alg.down
		right += alg.right
	}
	return tree_count
}

var ALGORITHMS = [5]Algorithm{
	Algorithm{right: 1, down: 1},
	Algorithm{right: 3, down: 1},
	Algorithm{right: 5, down: 1},
	Algorithm{right: 7, down: 1},
	Algorithm{right: 1, down: 2}}

func main() {
	file, err := os.Open("input")
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()
	scanner := bufio.NewScanner(file)
	lines := make([]string, 0, 0)
	for scanner.Scan() {
		lines = append(lines, scanner.Text())
	}
	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}

    var prod uint = 1
	for _, alg := range ALGORITHMS {
		prod *= run_algorithm(lines, alg)
	}
	fmt.Println(prod)
}
