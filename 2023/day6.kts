#!/usr/bin/env -S kotlinc -J-ea -script
import kotlin.math.floor
import kotlin.math.log10
import kotlin.math.pow

println("ready")

class Race(val duration: Long, val maxDist: Long) {

    fun part1(): Long {
        val min = (0..duration).first { (duration - it) * it > maxDist }
        val max = (0..duration).last { (duration - it) * it > maxDist }
        //println ("$max - $min")
        return max - min + 1 // the famous post +1
    }
}

assert(Race(7, 9).part1() == 4L)
assert(Race(15, 40).part1() == 8L)
assert(Race(30, 200).part1() == 9L)

val races = ArrayList<Race>()

val times = readln().substring("Time: ".length).split(" +".toRegex()).filter { it.isNotEmpty() }.map { it.toLong() }.toList()
val distances = readln().substring("Distance: ".length).split(" +".toRegex()).filter { it.isNotEmpty() }.map { it.toLong() }.toList()

assert(times.size == distances.size)

for ((idx, time) in times.withIndex()) {
    races.add(Race(time, distances[idx]))
}

var part1 = 1L
for (race in races) {
    println(race.part1())
    part1 *= race.part1()
}
println("Part1: $part1")

// build a megarace.
val megatime = times.reversed().reduce{res, a -> (res + (a * 10.toDouble().pow(1+ floor(log10(res.toDouble()))))).toLong() }
println("megatime: $megatime")
val megadistance = distances .reversed().reduce{res, a -> (res + (a * 10.toDouble().pow(1+ floor(log10(res.toDouble()))))).toLong() }
println("megadistance: $megadistance")

val megarace = Race(megatime, megadistance)
println("Part2: ${megarace.part1()}")

