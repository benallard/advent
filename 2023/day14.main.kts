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

    fun after(value: Int, limit: Int): Boolean {
        return when (posDir) {
            true -> value >= limit
            false -> value <= limit
        }
    }

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


    fun idx1(dir: Direction): IntProgression {
        val r = when (dir.vert) {
            true -> content.indices
            false -> content[0].indices
        }
        return if (!dir.posDir) {
            r.reversed()
        } else {
            r
        }
    }

    fun idx2(dir: Direction): IntProgression {
        return when (dir.vert) {
            true -> content[0].indices
            false -> content.indices
        }
    }

    fun before(dir: Direction, x: Int, y: Int): List<Int> =
            idx1(dir)
                    //.onEach { print("idx1 $it ") }
                    .filter {
                        if (dir.vert) {
                            if (dir.posDir) {
                                it <= y
                            } else {
                                it >= y
                            }
                        } else {
                            if (dir.posDir) {
                                it <= x
                            } else {
                                it >= x
                            }
                        }
                    }
    //.onEach { print("f $it ") }

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
        for (y in idx1(dir)) {
            for (x in idx2(dir)) {
                if (content[y][x] == Spot.ROUNDED_ROCK) {
                    move(x, y, dir)
                }
            }
        }
        //print(); println()
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
        //println("Moving $x, $y")
        content[y][x] = Spot.EMPTY
        val min = lastCube(x, y, dir)
        //println("Cube is $min")
        val idx = firstEmpty(x, y, min, dir)
        //println("Moving to ... well ... $x, $y, $idx, $dir")
        set(x, y, idx, dir, Spot.ROUNDED_ROCK)
    }

    private fun lastCube(x: Int, y: Int, dir: Direction): Int =
            before(dir, x, y).lastOrNull { access(dir, x, y, it) == Spot.CUBE_ROCK } ?: before(dir, x, y).first()

    private fun firstEmpty(x: Int, y: Int, min: Int, dir: Direction): Int =
            before(dir, x, y)
                    .filter { dir.after(it, min) }
                    .first { access(dir, x, y, it) == Spot.EMPTY }

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

    fun dump(): String = content.joinToString(separator = "\n") { line ->
        line.joinToString(separator = "") {
            it.d.toString()
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

//platform.tilt(Direction.SOUTH)
//platform.print()

class Cache {
    private val states = ArrayList<String>()
    private val values = ArrayList<Int>()

    fun addState(state: String, value: Int) {
        states.add(state)
        values.add(value)
    }

    fun stateKnown(state: String) = states.contains(state)

    fun calculateRest(amount: Int, state: String): Int {
        val idx = states.indexOf(state)
        val period = states.size - idx
        val rest = amount - states.size
        return values[idx + (rest % period)]
    }

}

val cache = Cache()

for (idx in 0..<1000000000) {
    if (cache.stateKnown(platform.dump())) {
        val part2 = cache.calculateRest(1000000000, platform.dump())
        println("Part2: $part2")
        break;
    }

    cache.addState(platform.dump(), platform.northLoad())
    platform.cycle()
    //println("$idx: ${platform.northLoad()}")
    //platform.print();println();
}
