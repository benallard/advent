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

var total = 0

for (line in generateSequence{readLine()}){
    total += extractValues(line);
}
println("Part1: $total")
