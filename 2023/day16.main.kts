#!/usr/bin/env -S kotlinc -J-ea -script
println("ready")

enum class Direction(val horizontal: Boolean, private val posDir: Boolean) {
    NORTH(false, false),
    EAST(true, true),
    SOUTH(false, true),
    WEST(true, false),
    ;

    val dx: Int = if (horizontal) {
        if (posDir) 1 else -1
    } else 0

    val dy: Int = if (horizontal) 0 else {
        if (posDir) 1 else -1
    }
}

data class Coordinate(val x: Int, val y: Int) {
    fun apply(dir: Direction) = Coordinate(x + dir.dx, y + dir.dy)
}

interface Spot {
    val display: Char

    fun transform(input: Direction): Set<Direction>
}

object Empty : Spot {
    override val display = '.'

    override fun transform(input: Direction): Set<Direction> = setOf(input)
}

enum class Mirror(c: Char, private val pairs: Set<Set<Direction>>) : Spot {
    M1('/', setOf(
            setOf(Direction.EAST, Direction.NORTH),
            setOf(Direction.WEST, Direction.SOUTH))),
    M2('\\', setOf(
            setOf(Direction.EAST, Direction.SOUTH),
            setOf(Direction.WEST, Direction.NORTH))),
    ;

    override val display: Char = c

    override fun transform(input: Direction): Set<Direction> {
        for (pair in pairs) {
            if (pair.contains(input)) {
                return setOf(pair.first { it != input })
            }
        }
        throw AssertionError()
    }
}

enum class Splitter(c: Char, private val horizontal: Boolean) : Spot {
    HOR('-', true),
    VERT('|', false),
    ;

    override val display: Char = c

    override fun transform(input: Direction): Set<Direction> {
        if (input.horizontal == horizontal) {
            return setOf(input)
        }
        return when (horizontal) {
            true -> setOf(Direction.EAST, Direction.WEST)
            false -> setOf(Direction.NORTH, Direction.SOUTH)
        }
    }

}

fun toSpot(c: Char): Spot =
        listOf(Empty, Mirror.M1, Mirror.M2, Splitter.HOR, Splitter.VERT)
                .first { it.display == c }


class Grid(width: Int, height: Int) {
    private val structure: Array<Array<Spot>> = Array(height) { Array(width) { Empty } }
    private val light = Array(height) { Array(width) { mutableSetOf<Direction>() } }

    fun init(content: List<String>) {
        content.withIndex()
                .forEach { line ->
                    line.value
                            .map { toSpot(it) }
                            .withIndex()
                            .filter { it.value != Empty }
                            .forEach { structure[line.index][it.index] = it.value }
                }
    }

    fun propagate(coordinate: Coordinate, direction: Direction) {
        light[coordinate.y][coordinate.x].add(direction)
        val output = structure[coordinate.y][coordinate.x].transform(direction)
        for (out in output) {
            val next = coordinate.apply(out)
            if (outside(next)) {
                continue
            }
            if (!light[next.y][next.x].contains(out)) {
                propagate(next, out)
            }
        }
    }

    private fun outside(coordinate: Coordinate) =
            coordinate.x < 0 || coordinate.x >= structure[0].size
                    || coordinate.y < 0 || coordinate.y >= structure.size

    fun energy() = light.flatMap { it.asIterable() }.count { it.size != 0 }

}

val lines = generateSequence { readlnOrNull() }.toList()
val grid = Grid(lines[0].length, lines.size)
grid.init(lines)

grid.propagate(Coordinate(0, 0), Direction.EAST)
val part1 = grid.energy()
println("Part1: $part1")