fn get_input() -> &'static str {
    // string compiled to our program - the data pointed to by the reference lives for the entire lifetime of the running program.
    return "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";
}

fn main() {
    let ans = get_input()
        .lines()
        .enumerate()
        .flat_map(|(idx, line)| {
            return line.chars().nth(idx * 3 % line.len());
        })
        .filter(|x| *x == '#')
        .count();
    print!("{:?}", ans)
}
