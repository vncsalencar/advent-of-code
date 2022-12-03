package main

import (
	"fmt"
	"github.com/vncsalencar/advent-of-code/util"
	"strings"
)

func main() {
	// 97 - 122, 65 - 90
	// fmt.Printf("%d - %d, %d - %d", int('a'), int('z'), int('A'), int('Z'))
	runeToPriority := func(r rune) int {
		runeValue := int(r)
		if runeValue >= 97 {
			return runeValue - 96
		}
		return runeValue - 64 + 26
	}

	getRucksackPriority := func(s1, s2 string) int {
		for _, char := range s1 {
			if strings.ContainsRune(s2, char) {
				priority := runeToPriority(char)
				return priority
			}
		}
		panic(fmt.Sprintf("No duplicate found for rucksack '%s%s'", s1, s2))
	}

	input := util.GetInput()
	lines := strings.Split(input, "\n")

	priorities := 0
	for _, line := range lines {
		half := len(line) / 2
		priorities += getRucksackPriority(line[:half], line[half:])
	}

	getRucksackGroupPriority := func(s1, s2, s3 string) int {
		for _, char := range s1 {
			if strings.ContainsRune(s2, char) && strings.ContainsRune(s3, char) {
				priority := runeToPriority(char)
				return priority
			}
		}
		panic(fmt.Sprintf("No duplicate found for rucksack group \n%s\n%s\n%s\n", s1, s2, s3))
	}

	groupPriorities := 0
	for index := range lines {
		if (index+1)%3 == 0 {
			groupPriorities += getRucksackGroupPriority(lines[index-2], lines[index-1], lines[index])
		}
	}

	fmt.Println("# Day 3: Rucksack Reorganization")
	fmt.Println("Part 1 =", priorities)
	fmt.Println("Part 2 =", groupPriorities)
}
