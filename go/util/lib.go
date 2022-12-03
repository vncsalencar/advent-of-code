package util

import (
	"io/ioutil"
	"strings"
)

func GetInput() string {
	path := "./input.txt"
	file, err := ioutil.ReadFile(path)

	if err != nil {
		panic(err)
	}

	return strings.TrimRight(string(file), "\n")
}
