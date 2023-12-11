#!/usr/bin/env -S kotlinc -J-ea -script
println("ready")

class Card(val nr: Int, val winnings: Set<Int>, val have: Set<Int>) {

    fun points(): Int {
        var res = 0
        for (winning in winnings){
            if (have.contains(winning)){
                if (res == 0){
                    res = 1
                } else {
                    res *= 2
                }
            }
        }
        return res
    }

    fun matchingNr(): Int {
        return have.count { winnings.contains(it) }
    }

}

fun readCard(aLine: String): Card {
    val parts = aLine.split(": ")
    val number = parts[0].substring(5).trim().toInt()
    val numbers = parts[1].split(" | ")

    return Card(number,
            numbers[0].split(" +".toRegex()).filter { it.isNotEmpty() }.map { it.toInt() }.toSet(),
            numbers[1].split(" +".toRegex()).filter { it.isNotEmpty() }.map { it.toInt() }.toSet())
}

assert(readCard("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53").points() == 8)
assert(readCard("Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19").points() == 2)
assert(readCard("Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1").points() == 2)
assert(readCard("Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83").points() == 1)
assert(readCard("Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36").points() == 0)
assert(readCard("Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11").points() == 0)

var part1 = 0
val cards = ArrayList<Card>()
for (line in generateSequence { readlnOrNull() }) {
    val card = readCard(line)
    part1 += card.points()
    cards.add(card)
}

println("Part1: $part1")
val extras = IntArray(cards.size){ 1 }
for ((idx, card) in cards.withIndex()){
    for (win in 1..card.matchingNr()){
        extras[idx + win] += extras[idx]
    }
}
//println(extras.map { it.toString() })
println("Part2: ${extras.sum()}")
