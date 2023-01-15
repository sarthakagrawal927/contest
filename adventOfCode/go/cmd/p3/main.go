package main

import "strings"

func getInput() string {
	return `..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#`
}

type Thing = int

const (
	Tree Thing = iota
	Snow
)

func main() {
	treeCount := 0
	for idx, line := range strings.Split(getInput(), "\n") {
		if string(line[(idx*3)%len(line)]) == "#" {
			treeCount += 1
		}
	}
	print(treeCount)
}
