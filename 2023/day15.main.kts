#!/usr/bin/env -S kotlinc -J-ea -script
println("ready")

fun HASH(input: String): Int {
    var value = 0
    for (c in input) {
        value += c.code
        value *= 17
        value %= 256
    }
    return value
}

println("HASH**2 = ${HASH("HASH")}")

val instr = readln().split(",")
val part1 = instr.sumOf { HASH(it) }
println("Part1: $part1")

class Box {
    private val lenses = ArrayList<Pair<String, Int>>()

    fun add(label: String, focal: Int) {
        val idx = lenses.indexOfFirst { it.first == label }

        if (idx != -1) {
            lenses[idx] = Pair(label, focal)
        } else {
            lenses.add(Pair(label, focal))
        }
    }

    fun remove(label: String) =
            lenses.removeIf { it.first == label }

    fun power(boxNr: Int): Int =
            lenses.withIndex()
                    .sumOf { (boxNr + 1) * (it.index + 1) * it.value.second }

}

val boxes = Array<Box>(256) { Box() }

for (it in instr) {
    val label = it.split("[=-]".toRegex())[0]
    val box = HASH(label)
    val op = it[label.length]
    when (op) {
        '-' -> boxes[box].remove(label)
        '=' -> boxes[box].add(label, it.last().digitToInt())
    }
}

val part2 = boxes.withIndex()
        .sumOf { it.value.power(it.index) }
println("Part2: $part2")