package main

import (
	"fmt"
	"github.com/vncsalencar/advent-of-code/util"
	"strings"
)

func main() {
	input := util.GetInput()
	lines := strings.Split(input, "\n")

	var part1 int32 = 0
	var part2 int32 = 0
	// A      B       C
	// X      Y       Z
	// rock paper   scissors
	moves1 := map[string]int32{
		"A X": 1 + 3,
		"A Y": 2 + 6,
		"A Z": 3 + 0,
		"B X": 1 + 0,
		"B Y": 2 + 3,
		"B Z": 3 + 6,
		"C X": 1 + 6,
		"C Y": 2 + 0,
		"C Z": 3 + 3,
	}

	moves2 := map[string]int32{
		"A X": 3 + 0,
		"A Y": 1 + 3,
		"A Z": 2 + 6,
		"B X": 1 + 0,
		"B Y": 2 + 3,
		"B Z": 3 + 6,
		"C X": 2 + 0,
		"C Y": 3 + 3,
		"C Z": 1 + 6,
	}

	for _, line := range lines {
		part1 += moves1[line]
		part2 += moves2[line]
	}

	fmt.Println("# Day 2: Rock Paper Scissors")
	fmt.Println("Part 1 =", part1)
	fmt.Println("Part 2 =", part2)
}
