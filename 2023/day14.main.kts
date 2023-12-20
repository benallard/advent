#!/usr/bin/env -S kotlinc -J-ea -script
println("ready")

enum class Spot(val d: Char) {
    EMPTY('.'),
    ROUNDED_ROCK('O'),
    CUBE_ROCK('#')
    ;

    companion object {
        fun of(c: Char): Spot {
            for (s in Spot.entries) {
                if (c == s.d) {
                    return s
                }
            }
            throw AssertionError(c)
        }
    }
}

enum class Direction(val vert: Boolean, val posDir: Boolean) {
    NORTH(true, true),
    WEST(false, true),
    SOUTH(true, false),
    EAST(false, false),
    ;
}

class Platform(height: Int, width: Int) {
    private val content = Array(height) { Array(width) { Spot.EMPTY } }
    private var currentRow = 0
    fun addRow(line: List<Spot>) {
        for ((idx, spot) in line.withIndex()) {
            if (spot != Spot.EMPTY) {
                content[currentRow][idx] = spot
            }
        }
        currentRow++
    }



    fun idx1(height: Int, width: Int): IntProgression {
        val r = when (vert) {
            true -> 0..height
            false -> 0..width
        }
        return if (!posDir) {
            r.reversed()
        } else {
            r
        }
    }

    fun idx2(height: Int, width: Int): IntProgression {
        return  when (vert) {
            true -> 0..width
            false -> 0..height
        }
    }

    fun before(x: Int, y: Int): List<Int> =
            idx1(y, x)
                    .onEach { print("idx1 $it ") }
                    .filter {
                        if (vert) {
                            if (posDir) {
                                it <= y
                            } else {
                                it >= y
                            }
                        } else {
                            if (posDir) {
                                it <= x
                            } else {
                                it >= x
                            }
                        }
                    }
                    .onEach {print("f $it ")}

    fun access(dir: Direction, x: Int, y: Int, idx: Int): Spot {
        return if (dir.vert) {
            content[idx][x]
        } else {
            content[y][idx]
        }
    }

    fun set(x: Int, y: Int, idx: Int, dir: Direction, what: Spot) {
        if (dir.vert) {
            content[idx][x] = what
        } else {
            content[y][idx] = what
        }
    }

    fun tiltNorth() {
        tilt(Direction.NORTH);
    }

    fun cycle() {
        for (dir in Direction.entries) { // yes, the order is already right
            tilt(dir)
        }
    }

    fun tilt(dir: Direction) {
        for (y in dir.idx1(content.size - 1, content[0].size - 1)) {
            for (x in dir.idx2(content.size - 1, content[0].size - 1)) {
                if (content[y][x] == Spot.ROUNDED_ROCK) {
                    move(x, y, dir)
                }
            }
        }
        print(); println()
    }

    private fun moveNorth(x: Int, y: Int) {
        // clear the spot while the rock rolls
        content[y][x] = Spot.EMPTY
        // find the last Fixed Block
        val min = (0..<y).lastOrNull { content[it][x] == Spot.CUBE_ROCK } ?: 0
        val idx = (min..y).first { content[it][x] == Spot.EMPTY }
        content[idx][x] = Spot.ROUNDED_ROCK
    }

    private fun move(x: Int, y: Int, dir: Direction) {
        println("Moving $x, $y")
        content[y][x] = Spot.EMPTY
        val min = lastCube(x, y, dir)
        println("Cube is $min")
        val idx = firstEmpty(x, y, min, dir)
        println("Moving to ... well ... $x, $y, $idx, $dir")
        dir.set(x, y, idx, content, Spot.ROUNDED_ROCK)
    }

    private fun lastCube(x: Int, y: Int, dir: Direction): Int =
            dir.before(x, y).lastOrNull { dir.access(content, x, y, it) == Spot.CUBE_ROCK } ?: 0

    private fun firstEmpty(x: Int, y: Int, min: Int, dir: Direction): Int =
            dir.before(x, y)
                    .filter { it >= min }
                    .first { dir.access(content, x, y, it) == Spot.EMPTY }

    fun northLoad(): Int {
        var res = 0
        for ((idx, row) in content.withIndex()) {
            res += row.count { it == Spot.ROUNDED_ROCK } * (content.size - idx)
        }
        return res
    }

    fun print() {
        for (line in content) {
            println(line.map { it.d }.joinToString(separator = ""))
        }
    }

}

val lines = generateSequence { readlnOrNull() }
        .toList()
val platform = Platform(lines.size, lines[0].length)
lines.forEach { line -> platform.addRow(line.map { Spot.of(it) }) }
if (false) {
    platform.tiltNorth()
    platform.print()
    val part1 = platform.northLoad()
    println("Part1: $part1")
}

platform.tilt(Direction.SOUTH)
platform.print()


//(0..3).forEach { _ -> platform.cycle() }
//
//val part2 = platform.northLoad()
//println("Part2: $part2")