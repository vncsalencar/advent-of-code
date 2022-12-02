package util

import (
	"io/ioutil"
	"strings"
)

func GetInput(filename string) string {
	path := "./input/" + filename
	file, err := ioutil.ReadFile(path)

	if err != nil {
		panic(err)
	}

	return strings.TrimRight(string(file), "\n")
}
