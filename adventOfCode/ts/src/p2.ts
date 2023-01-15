function getInput(): string {
    return `0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2`
}

type Point = {
    x: Number,
    y: Number
}

type Line = {
    p1: Point
    p2: Point
}

function isStraight(line: Line): boolean {
    console.log(line);

    return line.p1.x === line.p2.x || line.p2.y === line.p1.y
}

function parsePoint(line: string): Point {
    const [x, y] = line.split(",");
    return {
        x: +x, y: +y
    }
}

function parseLine(line: string): Line {
    const [x, y] = line.split(" -> ");
    return {
        p1: parsePoint(x),
        p2: parsePoint(y)
    }
}

const out = getInput()
    .split('\n')
    .map(parseLine)
    .filter(isStraight)


console.log({ out })