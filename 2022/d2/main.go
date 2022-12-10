package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
)

func main() {
	file, err := os.Open("input.txt")
	if err != nil {
		log.Fatal(err)
	}

	var score1, score2 int

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		score1 += part1(scanner.Text())
		score2 += part2(scanner.Text())
	}

	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}

	fmt.Println(score1)
	fmt.Println(score2)
}

func part1(round string) (score int) {
	switch round {
	case "A X":
		score += 4
	case "A Y":
		score += 8
	case "A Z":
		score += 3
	case "B X":
		score += 1
	case "B Y":
		score += 5
	case "B Z":
		score += 9
	case "C X":
		score += 7
	case "C Y":
		score += 2
	case "C Z":
		score += 6
	}
	return
}

func part2(round string) (score int) {
	switch round {
	case "A X":
		score += 3
	case "A Y":
		score += 4
	case "A Z":
		score += 8
	case "B X":
		score += 1
	case "B Y":
		score += 5
	case "B Z":
		score += 9
	case "C X":
		score += 2
	case "C Y":
		score += 6
	case "C Z":
		score += 7
	}
	return
}