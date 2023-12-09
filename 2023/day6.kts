#!/usr/bin/env -S kotlinc -J-ea -script
println("ready")

class Race(val duration: Int, val maxDist: Int) {

    fun part1(): Int {
        val min = (0..duration).first { (duration - it) * it > maxDist }
        val max = (0..duration).last { (duration - it) * it > maxDist }
        //println ("$max - $min")
        return max - min + 1 // the famous post +1
    }

}

assert(Race(7, 9).part1() == 4)
assert(Race(15, 40).part1() == 8)
assert(Race(30, 200).part1() == 9)

val races = ArrayList<Race>()

val times = readln().substring("Time: ".length).split(" +".toRegex()).filter { it.isNotEmpty() }.map { it.toInt() }.toList()
val distances = readln().substring("Distance: ".length).split(" +".toRegex()).filter { it.isNotEmpty() }.map { it.toInt() }.toList()

assert(times.size == distances.size)

for ((idx, time) in times.withIndex()) {
    races.add(Race(time, distances[idx]))
}

var part1 = 1
for (race in races) {
    println(race.part1())
    part1 *= race.part1()
}
println("Part1: $part1")