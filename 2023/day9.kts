
fun predict1(values: List<Int>): Int {
    if (values.all { it == 0 }) {
        return 0
    }
    return values.last() + predict1(values.windowed(2)
            .map { w -> w[1] - w[0] })
}

fun predict2(values: List<Int>): Int {
    if (values.all { it == 0 }) {
        return 0
    }
    return values.first() - predict2(values.windowed(2)
            .map { w -> w[1] - w[0] })
}

val res1 = predict2(listOf(10, 13, 16, 21, 30, 45))
assert(res1 == 5)


val input = generateSequence { readlnOrNull() }.map { s ->
    s.split(" +".toRegex())
            .map { it.toInt() }
            .toList()
}.toList()

val part1 = input.sumOf { predict1(it) }
println("Part1: $part1")

val part2 = input.sumOf { predict2(it) }
println("Part2: $part2")

