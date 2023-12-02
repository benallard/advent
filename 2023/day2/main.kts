#!/usr/bin/env -S kotlinc -J-ea -script

class Game(val id: Int, val sets: Set<Map<String, Int>>){

    fun possible(): Boolean {
        // only 12 red cubes, 13 green cubes, and 14 blue cubes
        return sets.all{ it.get("red") ?: 0 <= 12
         && it.get("green")?: 0 <= 13
         && it.get("blue")?: 0 <= 14}
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
            s.put(color, amount);
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


var part1 = 0

for (line in generateSequence{readLine()}){
    val game = readGame(line)
    if (game.possible()){
        part1 += game.id;
    }
}
println("Part1: $part1")