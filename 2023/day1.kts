#!/usr/bin/env -S kotlinc -J-ea -script
println("ready")

fun extractValues(aLine: CharSequence): Int {
    val first = aLine.first { c -> c.isDigit() }
    val last = aLine.last { it.isDigit() }
    return first.digitToInt() * 10 + last.digitToInt()
}

assert(extractValues("1abc2") == 12)
assert(extractValues("pqr3stu8vwx") == 38)
assert(extractValues("a1b2c3d4e5f") == 15)
assert(extractValues("treb7uchet") == 77)

fun extractValues2(aLine: String): Int {
    return extractValues(aLine.windowedSequence(5, partialWindows = true).map { w ->
        w
                .replace("^one".toRegex(), "1")
                .replace("^two".toRegex(), "2")
                .replace("^three".toRegex(), "3")
                .replace("^four".toRegex(), "4")
                .replace("^five".toRegex(), "5")
                .replace("^six".toRegex(), "6")
                .replace("^seven".toRegex(), "7")
                .replace("^eight".toRegex(), "8")
                .replace("^nine".toRegex(), "9")
                .get(0)
    }.joinToString(""))
}

assert(extractValues2("two1nine") == 29)
assert(extractValues2("eightwothree") == 83)
assert(extractValues2("abcone2threexyz") == 13)
assert(extractValues2("xtwone3four") == 24)
assert(extractValues2("4nineeightseven2") == 42)
assert(extractValues2("zoneight234") == 14)
assert(extractValues2("7pqrstsixteen") == 76)


var part1 = 0
var part2 = 0

for (line in generateSequence { readlnOrNull() }) {
    part1 += extractValues(line)
    part2 += extractValues2(line)
}

println("Part1: $part1")
println("Part2: $part2")
