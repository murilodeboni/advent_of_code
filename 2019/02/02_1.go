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

func read() []int64 {
	f, err := os.Open("input.txt")

	if err != nil {
		log.Fatal(err)
	}

	defer f.Close()

	scanner := bufio.NewScanner(f)

	var s []string
	var a []int64

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
		switch n {
		case 1:
			a = append(a, 12)
		case 2:
			a = append(a, 2)
		default:
			a = append(a, marks)
		}
		n = n + 1
	}

	n = 0
	for n < len(a) {
		switch a[n] {
		case 1:
			a = addNumbers(a, a[a[n+1]], a[a[n+2]], a[n+3])
		case 2:
			a = multiplyNumbers(a, a[a[n+1]], a[a[n+2]], a[n+3])
		case 99:
			fmt.Println(a[n])
		}
		n = n + 4
	}

	return a
}

func main() {
	fmt.Println(read()[0])
}
