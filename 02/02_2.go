package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
)

func insert(a []int64, index int64, value int64) []int64 {
	if int64(len(a)) == index { // nil or empty slice or after last element
		return append(a, value)
	}
	a = append(a[:index+1], a[index:]...) // index < len(a)
	a[index] = value
	return a
}

func remove(slice []int64, s int64) []int64 {
	return append(slice[:s], slice[s+1:]...)
}

func deleteAndInsert(a []int64, index int64, value int64) []int64 {
	a = remove(a, index)
	return insert(a, index, value)
}

func addNumbers(a []int64, n1 int64, n2 int64, index int64) []int64 {
	var value int64 = n1 + n2
	return deleteAndInsert(a, index, value)
}

func multiplyNumbers(a []int64, n1 int64, n2 int64, index int64) []int64 {
	var value int64 = n1 * n2
	return deleteAndInsert(a, index, value)
}

func checkThis(a []int64, i int64, j int64) bool {
	tmp := make([]int64, len(a))
	copy(tmp, a)
	tmp = deleteAndInsert(tmp, 1, i)
	tmp = deleteAndInsert(tmp, 2, j)
	var n int = 0
	for n < len(tmp) {
		if n+1 < len(a) && n+2 < len(a) && tmp[n+1] < int64(len(a)) && tmp[n+2] < int64(len(a)) && tmp[n+3] < int64(len(a)) {
			switch tmp[n] {
			case 1:
				tmp = addNumbers(tmp, tmp[tmp[n+1]], tmp[tmp[n+2]], tmp[n+3])
			case 2:
				tmp = multiplyNumbers(tmp, tmp[tmp[n+1]], tmp[tmp[n+2]], tmp[n+3])
			case 99:
				n = len(a)
			}
		}
		n = n + 4
	}
	if tmp[0] == 19690720 {
		fmt.Println(i*100 + j)
		return true
	} else {
		return false
	}
}

func readInput(inp string) []int64 {
	f, err := os.Open(inp)

	if err != nil {
		log.Fatal(err)
	}

	defer f.Close()

	scanner := bufio.NewScanner(f)

	var s []string
	var ar []int64

	// read input as strings
	for scanner.Scan() {
		arr := scanner.Text()
		if err != nil {
			log.Fatal(err)
		}
		s = strings.Split(arr, ",")
	}

	// create array of ints from string read
	n := 0
	for n < len(s) {
		marks, err := strconv.ParseInt(s[n], 10, 0)
		if err != nil {
			fmt.Println("Error during conversion")
		}
		ar = append(ar, marks)
		n = n + 1
	}
	return ar
}

func read() []int64 {

	var inputArray []int64 = readInput("input.txt")

	var i int64 = 0
	var j int64 = 0
	for i <= 99 {
		j = 0
		for j <= 99 {
			if checkThis(inputArray, i, j) {
				return inputArray
			} else {
				j = j + 1
			}
		}
		i = i + 1
	}
	return inputArray[:3]

}

func main() {
	fmt.Println(read())
}
