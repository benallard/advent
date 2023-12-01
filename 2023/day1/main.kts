#!/usr/bin/env -S kotlinc -script

fun extractValues(aLine: String) : Int{
    val first = aLine.first { c -> c.isDigit() }
    val last = aLine.last { it.isDigit() }
    return first.digitToInt() * 10 + last.digitToInt()
}

assert(extractValues("1abc2") == 12)
assert(extractValues("pqr3stu8vwx") == 38)
assert(extractValues("a1b2c3d4e5f") == 15)
assert(extractValues("treb7uchet") == 77)

fun extractValues2(aLine: String): Int {
    return extractValues(
        aLine
            .replace("one", "1")
            .replace("two", "2")
            .replace("three", "3")
            .replace("four", "4")
            .replace("five", "5")
            .replace("six", "6")
            .replace("seven", "7")
            .replace("eight", "8")
            .replace("nine", "9")
    )
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

for (line in generateSequence{readLine()}){
    part1 += extractValues(line);
    part2 += extractValues2(line);
}

println("Part1: $part1")
println("Part2: $part2")
