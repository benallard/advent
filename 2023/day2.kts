#!/usr/bin/env -S kotlinc -J-ea -script
println("ready")

class Game(val id: Int, val sets: Set<Map<String, Int>>){

    fun possible(): Boolean {
        // only 12 red cubes, 13 green cubes, and 14 blue cubes
        return sets.all{
            ((it["red"] ?: 0) <= 12
                    && (it["green"] ?: 0) <= 13
                    && (it["blue"] ?: 0) <= 14)
        }
    }

    fun power(): Int {
        var red = 0;
        var green = 0;
        var blue = 0;
        for (set_ in sets){
            if (red < (set_["red"] ?: 0)){
                red = set_["red"] ?: 0
            }
            if (green < (set_["green"] ?: 0)){
                green = set_["green"] ?: 0
            }
            if (blue < (set_["blue"] ?: 0)){
                blue = set_["blue"] ?: 0
            }
        }
        return red * green * blue;
    }

}

fun readGame(aLine: String): Game{
    val parts = aLine.split(": ")
    val id = parts[0].split(" ")[1].toInt()
    val sets = HashSet<Map<String, Int>>()
    for (set_ in parts[1].split("; ")){
        val s = HashMap<String, Int>();
        for (def in set_.split(", ")){
            val defs = def.split(" ")
            val color = defs[1]
            val amount = defs[0].toInt()
            s[color] = amount;
        }
        sets.add(s)
    }
    return Game(id, sets);
}

assert(readGame("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green").possible())
assert(readGame("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue").possible())
assert(!readGame("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red").possible())
assert(!readGame("Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red").possible())
assert(readGame("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green").possible())

assert(readGame("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green").power() == 48)
assert(readGame("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue").power() == 12)
assert(readGame("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red").power() == 1560)
assert(readGame("Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red").power() == 630)
assert(readGame("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green").power() == 36)



var part1 = 0
var part2 = 0

for (line in generateSequence{ readlnOrNull() }){
    val game = readGame(line)
    if (game.possible()){
        part1 += game.id;
    }
    part2 += game.power();
}
println("Part1: $part1")
println("Part2: $part2")