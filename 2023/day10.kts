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

enum class Pipe(private val display: Char,
                private val input: Direction,
                private val output: Direction) : Tile {
    VERT('|', Direction.NORTH, Direction.SOUTH),
    HOR('-', Direction.EAST, Direction.WEST),
    EL('L', Direction.NORTH, Direction.EAST),
    JAY('J', Direction.NORTH, Direction.WEST),
    SEVEN('7', Direction.SOUTH, Direction.WEST),
    EFF('F', Direction.SOUTH, Direction.EAST),
    ;

    override fun display(): Char = display

    override fun ends(): Set<Direction> = setOf(input, output)

    fun other(d: Direction): Direction = ends().first{it != d}
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

    private fun start(): Coordinate {
        for ((idx, line) in map.withIndex()) {
            val x = line.indexOf(Start)
            if (x != -1) {
                return Coordinate(x, idx)
            }
        }
        throw AssertionError()
    }

    private fun get(c: Coordinate): Tile = map[c.y][c.x]

    fun length(): Int {
        val start = start()
        println("Start: $start: ${get(start).display()}")
        var current = start
        // choose a direction
        var dir = Direction.entries.first { d -> get(start.toward(d)).ends().contains(d.opposite()) }
        var res = 0
        do {
            current = current.toward(dir)
            println("Current: $current: ${get(current).display()}")
            dir = when(val tile = get(current)){
                is Pipe -> tile.other(dir.opposite())
                is Start -> dir // will stop now anyway
                else -> throw AssertionError()
            }
            res++
        } while (current != start)
        return res
    }

    data class Coordinate(val x: Int, val y: Int){
        fun toward(d : Direction): Coordinate = Coordinate(x + d.dx, y + d.dy)
    }

}

val map = Map(generateSequence { readlnOrNull() }
        .map { s -> s.map { fromDisplay(it) }.toList() }
        .toList())

val part1 = map.length().div(2)
println("Part1: $part1")
