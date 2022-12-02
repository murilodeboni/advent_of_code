package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
)

func read() []string {
	f, err := os.Open("input.txt")

	if err != nil {
		log.Fatal(err)
	}
	defer f.Close()

	var initialState []string

	scanner := bufio.NewScanner(f)

	for scanner.Scan() {
		line := scanner.Text()
		var lineChars []string
		for i := 0; i < len(line); i++ {
			char := string(line[i])
			lineChars = append(lineChars, char)
		}
		initialState = append(initialState, line)
	}
	return initialState

}

func main() {
	var result = read()
	fmt.Println(result)
}
