package main

import (
	"fmt"
	"github.com/vncsalencar/advent-of-code/util"
	"sort"
	"strconv"
	"strings"
)

func main() {
	input := util.GetInput()
	lines := strings.Split(input, "\n")

	index := 0
	elves := make([]int64, 1)

	var biggest int64 = 0

	for _, line := range lines {
		if line == "" {
			if elves[index] > biggest {
				biggest = elves[index]
			}

			index++
			elves = append(elves, 0)
			continue
		}

		if calorie, err := strconv.ParseInt(line, 10, 64); err == nil {
			elves[index] += calorie
		}
	}

	fmt.Println("# Day 1: Calorie Counting")
	fmt.Println("Part 1 =", biggest)
	sort.Slice(elves, func(i, j int) bool { return elves[i] > elves[j] })

	topThree := elves[0] + elves[1] + elves[2]
	fmt.Println("Part 2 =", topThree)
}
