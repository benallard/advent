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