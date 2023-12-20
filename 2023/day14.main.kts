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

    fun tiltNorth() {
        for (y in content.indices) {
            for (x in content[y].indices) {
                if (content[y][x] == Spot.ROUNDED_ROCK) {
                    moveNorth(x, y)
                }
            }
        }
    }

    private fun moveNorth(x: Int, y: Int) {
        // clear the spot while the rock rolls
        content[y][x] = Spot.EMPTY
        // find the
        val min = (0..<y).lastOrNull { content[it][x] == Spot.CUBE_ROCK } ?: 0
        val idx = (min..y).first { content[it][x] == Spot.EMPTY }
        content[idx][x] = Spot.ROUNDED_ROCK
    }

    fun northLoad(): Int {
        var res = 0
        for ((idx, row) in content.withIndex()){
            res += row.count { it == Spot.ROUNDED_ROCK } * (content.size - idx)
        }
        return res
    }

    fun print(){
        for (line in content){
            println(line.map { it.d }.joinToString(separator = ""))
        }
    }

}

val lines = generateSequence { readlnOrNull() }
        .toList()
val platform = Platform(lines.size, lines[0].length)
lines.forEach { line -> platform.addRow(line.map { Spot.of(it) }) }

platform.tiltNorth()
platform.print()
val part1 = platform.northLoad()
println("Part1: $part1")
