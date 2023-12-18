#!/usr/bin/env -S kotlinc -J-ea -script
println("ready")

// Recursively reduce the solution space one character at a time

fun count(line: String, groups: List<Int>, current: Int = 0): Int {
    if (line.isEmpty()) {
        // end of branch: 1 only if no group (and no current)
        return if (groups.isEmpty()){
            1
        } else {
            0
        }
    }
    var res = 0
    val branches = when (line[0]) {
        '?' -> listOf('.', '#')
        else -> listOf(line[0])
    }
    for (char in branches){
        if (char == '#'){
            // simply go one ahead
            res += count(line.substring(1), groups, current + 1)
        } else {
            // can only be '.'
            if (current != 0){
                // we just closed a group
                // go forward if it matched
                if (groups.isNotEmpty() && groups[0] == current){
                    res += count(line.substring(1), groups.subList(1, groups.size))
                }
            } else {
                res += count(line.substring(1), groups)
            }
        }
    }
    return res
}

val lines = generateSequence { readlnOrNull() }
        .map { line ->
            val parts = line.split(" ")
            val groups = parts[1].split(",").map { it.toInt() }.toList()
            Pair(parts[0], groups)
        }
        .toList()

// Small trick with the .
val part1 = lines.sumOf { count(it.first + '.', it.second) }
println("Part1: $part1")