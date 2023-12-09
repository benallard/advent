#!/usr/bin/env -S kotlinc -J-ea -script
println("ready")

enum class Card(val display: Char) {
    AS('A'),
    KING('K'),
    QUEEN('Q'),
    J('J'),
    T('T'),
    NINE('9'),
    EIGHT('8'),
    SEVEN('7'),
    SIX('6'),
    FIVE('5'),
    FOUR('4'),
    THREE('3'),
    TWO('2'),
    ;
}

fun readCard(v: Char): Card {
    for (card in Card.entries) {
        if (card.display == v) {
            return card
        }
    }
    throw IllegalArgumentException(v.toString())
}

class Hand(val cards: Array<Card>, val bid: Int) : Comparable<Hand> {

    enum class HandType {
        FIVE,
        FOUR,
        HOUSE,
        THREE,
        PAIRS,
        PAIR,
        HIGH,
        ;
    }

    fun getType(): HandType {
        if (cards.distinct().count() == 1) {
            return HandType.FIVE
        }
        val occurrences = cards.groupBy(Card::name)
                .values
                .map { it.count() }
        if (occurrences.contains(4)) {
            return HandType.FOUR
        }
        if (occurrences.containsAll(listOf(3, 2))) {
            return HandType.HOUSE
        }
        if (occurrences.contains(3)) {
            return HandType.THREE
        }
        if (occurrences.count { it == 2 } == 2) {
            return HandType.PAIRS
        }
        if (occurrences.contains(2)) {
            return HandType.PAIR
        }
        return HandType.HIGH
    }

    fun card1(): Card = cards[0]
    fun card2(): Card = cards[1]
    fun card3(): Card = cards[2]
    fun card4(): Card = cards[3]
    fun card5(): Card = cards[4]

    override fun compareTo(other: Hand): Int = compareBy(
            Hand::getType,
            Hand::card1,
            Hand::card2,
            Hand::card3,
            Hand::card4,
            Hand::card5
    ).compare(this, other)

    override fun toString(): String {
        return cards.joinToString(separator = "")
    }

}

fun readHand(line: String): Hand {
    val cards = line.substring(0..4).map { readCard(it) }.toTypedArray()
    val bid = line.substring(6).toInt()
    return Hand(cards, bid)
}

val hands = generateSequence { readlnOrNull() }
        .map { readHand(it) }
val part1 = hands
        .sortedDescending()
        .withIndex()
        .onEach { println(it) }
        .map { hand -> (hand.index + 1) * hand.value.bid }
        .sum()
println("Part1: $part1")
