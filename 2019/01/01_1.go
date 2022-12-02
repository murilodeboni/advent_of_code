package main

import (
	"bufio"
	"fmt"
	"log"
	"math"
	"os"
	"strconv"
)

func calculateFuel(n int) int {
	var tmp float64

	tmp = float64(n)

	return int(math.Floor(tmp/3) - 2)
}

func read() int {
	f, err := os.Open("input.txt")

	if err != nil {
		log.Fatal(err)
	}

	defer f.Close()

	scanner := bufio.NewScanner(f)

	var tmp int = 0

	for scanner.Scan() {
		n, err := strconv.Atoi(scanner.Text())
		if err != nil {
			log.Fatal(err)
		}
		tmp = tmp + calculateFuel(n)
	}
	return tmp

}

func main() {
	fmt.Println(read())
}
