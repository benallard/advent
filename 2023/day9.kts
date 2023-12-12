#!/usr/bin/env -S kotlinc -J-ea -script
println("ready")

fun predict(values: List<Int>): Int {
    if (values.all{it == 0}){
        return 0
    }
    return values.last() + predict(values.windowed(2)
            .map { w -> w[1] - w[0] })
}

val res1 = predict(listOf(0,3,6,9,12,15))
assert(res1 == 18)

val part1 = generateSequence { readlnOrNull() }.map { it.split(" +".toRegex())
        .map { it.toInt() }
        .toList()}
        .map{predict(it)}
        .sum()
println("Part1: $part1")


