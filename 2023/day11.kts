#!/usr/bin/env -S kotlinc -J-ea -script
import kotlin.math.max
import kotlin.math.min

println("ready")

enum class Pixel(val display: Char) {
    EMPTY('.'),
    GALAXY('#'),
    ;
}

fun toPixel(char: Char): Pixel {
    for (p in Pixel.entries) {
        if (char == p.display) {
            return p
        }
    }
    throw AssertionError(char)
}

class Image(private val content: List<List<Pixel>>) {

    val galaxies = content.withIndex()
            .flatMap { (y, line) ->
                line.withIndex()
                        .filter { it.value == Pixel.GALAXY }
                        .map { Coordinate(it.index, y) }
            }
            .toList()

    init {
        println("${galaxies.size} Galaxies")
    }

    private fun emptyRow(y: Int): Boolean = content[y]
            .filterNot { it == Pixel.EMPTY }
            .isEmpty()

    private fun emptyCol(x: Int): Boolean = content.indices
            .map { content[it][x] }
            .filterNot { it == Pixel.EMPTY }
            .isEmpty()

    fun distance(a: Coordinate, b: Coordinate, factor: Int = 2): Long {
        val lat = (a xTo b).map {
            if (emptyCol(it)) {
                factor
            } else {
                1
            }.toLong()
        }.sum()
        val lon = (a yTo b).map {
            if (emptyRow(it)) {
                factor
            } else {
                1
            }.toLong()
        }.sum()
        println("$a -> $b : $lat + $lon = ${lat + lon}")
        return lat + lon
    }

    data class Coordinate(val x: Int, val y: Int) {
        infix fun xTo(other: Coordinate): IntRange = min(this.x, other.x)..<max(this.x, other.x)
        infix fun yTo(other: Coordinate): IntRange = min(this.y, other.y)..<max(this.y, other.y)
    }

}

fun <T> pairs(list: List<T>): List<Pair<T, T>> {
    return list.withIndex()
            .flatMap { (idx, t1) ->
                list.subList(idx + 1, list.size)
                        .map { Pair(t1, it) }
            }
}

val image = Image(generateSequence { readlnOrNull() }
        .map { s ->
            s.map { toPixel(it) }
                    .toList()
        }
        .toList())

val gPairs = pairs(image.galaxies)
println("${gPairs.size} pairs")
val part1 = gPairs.map { image.distance(it.first, it.second) }.sum()
println("Part1: $part1")
val part2 = gPairs.map { image.distance(it.first, it.second, 1000000) }.sum()
println("Part2: $part2")
