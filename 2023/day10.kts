#!/usr/bin/env -S kotlinc -J-ea -script
println("ready")

enum class Direction(val dx: Int, val dy: Int) {
    NORTH(0, -1),
    EAST(1, 0),
    SOUTH(0, 1),
    WEST(-1, 0),
    ;

    fun opposite(): Direction {
        return when (this) {
            NORTH -> SOUTH
            EAST -> WEST
            SOUTH -> NORTH
            WEST -> EAST
        }
    }
}

interface Tile {
    fun display(): Char

    fun ends(): Set<Direction>
}


fun fromDisplay(c: Char): Tile {
    for (pipe in Pipe.entries) {
        if (pipe.display() == c) {
            return pipe
        }
    }
    if (c == Ground.display()) {
        return Ground
    }
    if (c == Start.display()) {
        return Start
    }
    throw AssertionError(c)
}

enum class Pipe(
        private val display: Char,
        private val input: Direction,
        private val output: Direction,
) : Tile {
    VERT('|', Direction.NORTH, Direction.SOUTH),
    HOR('-', Direction.EAST, Direction.WEST),
    EL('L', Direction.NORTH, Direction.EAST),
    JAY('J', Direction.NORTH, Direction.WEST),
    SEVEN('7', Direction.SOUTH, Direction.WEST),
    EFF('F', Direction.SOUTH, Direction.EAST),
    ;

    override fun display(): Char = display

    override fun ends(): Set<Direction> = setOf(input, output)

    fun other(d: Direction): Direction = ends().first { it != d }

    companion object {
        fun of(d1: Direction, d2: Direction): Pipe {
            for (pipe in Pipe.entries) {
                if (pipe.ends().containsAll(listOf(d1, d2))) {
                    return pipe
                }
            }
            throw AssertionError("No pipe with ends: $d1, $d2")
        }
    }

}


object Ground : Tile {
    override fun display(): Char = '.'
    override fun ends(): Set<Direction> = emptySet()
}

object Start : Tile {
    override fun display(): Char = 'S'
    override fun ends(): Set<Direction> {
        throw AssertionError()
    }
}

class Map(private val map: List<List<Tile>>) {

    private val isLoop = Array(map.size) { Array(map[0].size) { false } }

    private val start = Coordinate(
            map.first() { it.contains(Start) }
                    .indexOfFirst { it == Start },
            map.indexOfFirst { it.contains(Start) })

    private fun get(c: Coordinate): Tile {
        if (c.y < 0 || c.y >= map.size
                || c.x < 0 || c.x >= map[c.y].size) {
            return Ground
        }
        if (c == start) {
            // complex stuff to replace start with a proper pipe
            val dir1 = Direction.entries.first { d -> get(start.toward(d)).ends().contains(d.opposite()) }
            val dir2 = Direction.entries.last { d -> get(start.toward(d)).ends().contains(d.opposite()) }
            return Pipe.of(dir1, dir2)
        }
        return map[c.y][c.x]
    }

    private fun setLoop(c: Coordinate) {
        isLoop[c.y][c.x] = true
    }

    /**
     * Walk the loop, returns the length and set the isLoop Property
     */
    fun length(): Int {
        setLoop(start)
        println("Start: $start: ${get(start).display()}")
        var current = start
        // choose a direction
        var dir = get(start).ends().first()
        var res = 0
        do {
            current = current.toward(dir)
            setLoop(current)
            println("Current: $current: ${get(current).display()}")
            dir = when (val tile = get(current)) {
                is Pipe -> tile.other(dir.opposite())
                is Start -> dir // will stop now anyway
                else -> throw AssertionError()
            }
            res++
        } while (current != start)
        return res
    }

    data class Coordinate(val x: Int, val y: Int) {
        fun toward(d: Direction): Coordinate = Coordinate(x + d.dx, y + d.dy)
    }

    private fun crossing(x: Int, y: Int): Boolean {
        if (!isLoop[y][x]) {
            return false
        }
        // Special case to differentiate F-J (yes) to F-7 (no)
        return when (val tile = get(Coordinate(x, y))) {
            is Pipe -> {
                if (tile == Pipe.VERT) {
                    return true
                }
                if (tile in listOf(Pipe.HOR, Pipe.JAY, Pipe.SEVEN)) {
                    return false
                }
                // First non HOR is the exit
                var exit = map[y].subList(x + 1, map[y].size).first { it != Pipe.HOR }
                if (exit == Start){
                    exit = get(start)
                }
                //println("$tile -> $exit = ${tile.other(Direction.EAST) !in exit.ends()}")
                return tile.other(Direction.EAST) !in exit.ends()
            }

            else -> false
        }
    }

    // https://en.wikipedia.org/wiki/Even–odd_rule
    private fun isInside(x: Int, y: Int): Boolean {
        if (isLoop[y][x]) {
            return false
        }
        //println("$y, ${isLoop[y].map { it.toString() }}")
        var countLoop = 0
        for (idx in x + 1..<map[y].size) {
            if (crossing(idx, y)) {
                //println("$idx, $y is a crossing")
                countLoop++
            }
        }
        return countLoop % 2 == 1
    }

    fun countInside(): Int {
        var res = 0
        for (y in map.indices) {
            for (x in map[y].indices) {
                if (isInside(x, y)) {
                    println("$x, $y is inside")
                    res++
                }
            }
        }
        return res
    }

}

val map = Map(generateSequence { readlnOrNull() }
        .map { s -> s.map { fromDisplay(it) }.toList() }
        .toList())

val part1 = map.length().div(2)
println("Part1: $part1")

println("Part2: ${map.countInside()}")
