#!/usr/bin/env -S kotlinc -J-ea -script
println("ready")

enum class Spot(private val d: Char) {
    ASH('.'),
    ROCK('#'),
    ;

    companion object {
        fun of(c: Char): Spot {
            for (s in Spot.entries) {
                if (c == s.d) {
                    return s
                }
            }
            throw AssertionError(c)
        }
    }
}

class MirrorMap(private val spots: List<List<Spot>>) {
    val valid = spots.map { it.size }.distinct().size == 1
    fun part1(): Int {
        val part1hor = part1hor()
        if (part1hor != 0) {
            println("Hor: $part1hor")
            return 100 * part1hor
        }
        return part1ver()
    }

    private fun part1hor(): Int {
        for (idx in 1..<spots.size) {
            var res = true
            var iidx = 0
            while (res && ((idx - iidx - 1) >= 0) && ((idx + iidx) < spots.size)) {
                //println("$idx, $iidx, ${idx - iidx - 1}, ${idx + iidx}, ${spots[idx - iidx - 1] == spots[idx + iidx]}")
                res = res && spots[idx - iidx - 1] == spots[idx + iidx]
                iidx += 1
            }
            if (res) {
                return idx
            }
        }
        return 0
    }

    private fun part1ver(): Int {
        for (idx in 1..<spots[0].size) {
            var res = true
            var iidx = 0
            while (res && ((idx - iidx - 1) >= 0) && ((idx + iidx) < spots[0].size)) {
                res = res && cmpver(idx - iidx - 1, idx + iidx)
                iidx += 1
            }
            if (res) {
                return idx
            }
        }
        return 0
    }

    private fun cmpver(idxa: Int, idxb: Int): Boolean {
        val cola = spots.map { it[idxa] }
        val colb = spots.map { it[idxb] }
        return cola == colb
    }

}

fun readMap(): MirrorMap? {
    val spots = ArrayList<List<Spot>>();
    for (line in generateSequence { readlnOrNull() }) {
        if (line.isEmpty()) {
            break
        }
        spots.add(line
                .map { Spot.of(it) }
                .toList())
    }
    if (spots.isEmpty()) {
        return null
    }
    return MirrorMap(spots)
}

val part1 = generateSequence { readMap() }
        .map { it.part1() }
        .sum()
println("Part1: $part1")
