#!/usr/bin/env -S kotlinc -J-ea -script
println("ready")

class Schematics(private val spots: List<List<Char>>) {

    fun part1(): Int {
        var res = 0
        for (y in spots.indices) {
            var prevDigit = false
            var start = -1
            for (x in spots[y].indices) {
                val isDigit = spots[y][x].isDigit()
                if (prevDigit != isDigit) {
                    if (isDigit) {
                        start = x
                    } else {
                        if (!isolated(y, start, x - 1)) {
                            val partNr = spots[y].subList(start, x).joinToString(separator = "").toInt()
                            res += partNr
                            //println("$x, $y: $partNr")
                        }
                    }
                }
                prevDigit = isDigit
            }
            if (prevDigit) {
                if (!isolated(y, start, spots[y].size)) {
                    res += spots[y].subList(start, spots[y].size).joinToString(separator = "").toInt()
                }
            }
        }
        return res
    }

    private fun isolated(line: Int, start: Int, end: Int): Boolean {
        for (y in line - 1..line + 1) {
            if (y < 0 || y >= spots.size) {
                continue
            }
            for (x in start - 1..end + 1) {
                if (x < 0 || x >= spots[y].size) {
                    continue
                }
                if (y == line && x >= start && x <= end) {
                    continue
                }
                if (spots[y][x] != '.') {
                    //println("$x, $y, ${spots[y][x]}")
                    return false
                }
            }

        }
        return true
    }

    fun part2(): Int {
        var res = 0
        for (y in spots.indices) {
            for (x in spots[y].indices) {
                if (spots[y][x] == '*') {
                    val gearRatio = ratio(x, y)
                    if (gearRatio != 0) {
                        res += gearRatio
                    }
                }
            }
        }
        return res
    }

    private fun ratio(aX: Int, aY: Int): Int {
        var res = 1
        var count = 0
        for (y in aY - 1..aY + 1) {
            if (y < 0 || y >= spots.size) {
                continue
            }
            var prevDigit = false
            for (x in aX - 1..aX + 1) {
                if (x < 0 || x >= spots[y].size) {
                    continue
                }
                if (y == aY && x == aX) {
                    prevDigit = false
                    continue
                }
                val isDigit = spots[y][x].isDigit()
                if (isDigit != prevDigit && isDigit) {
                    res *= readDigit(x, y)
                    count += 1
                }
                prevDigit = isDigit
            }
        }
        if (count == 2) {
            return res
        }
        return 0
    }

    private fun readDigit(end: Int, line: Int): Int {
        var start = 0
        for (x in end downTo 0) {
            if (!spots[line][x].isDigit()) {
                start = x + 1
                break
            }
        }
        var stop = spots[line].size - 1
        for (x in end..<spots[line].size) {
            if (!spots[line][x].isDigit()) {
                stop = x - 1
                break
            }
        }
        return spots[line].subList(start, stop + 1).joinToString(separator = "").toInt()
    }

}

fun readSchematics(): Schematics {
    val spots = ArrayList<List<Char>>()
    for (line in generateSequence { readlnOrNull() }) {
        spots.add(line.toList())
    }
    return Schematics(spots)
}

var schematic = readSchematics()

println("Part1: ${schematic.part1()}")
println("Part2: ${schematic.part2()}")