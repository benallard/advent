#!/usr/bin/env -S kotlinc -J-ea -script
println("ready")

val seeds = readln().substring("seeds: ".length)
        .split(" ")
        .map { it.toInt() }
        .toSet()
readln()

class CategoryMap(val source: String, val destination: String){
    private var mapping: MutableMap<Int, Int> = HashMap()

    fun fillMap(dest: Int, src: Int, len: Int){
        for (value in 0..<len){
            mapping[src + value] = dest+value
        }
    }

    fun get(value: Int): Int{
        return mapping.getOrDefault(value, value)
    }

}

fun readMap(): CategoryMap{
    val categories = readln().split(" ")[0].split("-to-")
    val catMap = CategoryMap(categories[0], categories[1])
    for (line in generateSequence { readlnOrNull()}){
        if (line.isEmpty()){
            break
        }
        val nrs = line.split(" ")
        catMap.fillMap(nrs[0].toInt(), nrs[1].toInt(), nrs[2].toInt())
    }
    return catMap
}

val maps = HashMap<String, CategoryMap>()

for (map in generateSequence { readMap() }){
    maps[map.source] = map
    if (map.destination == "location"){
        break;
    }
}

println("Seeds: $seeds")

var lowestLoc = Int.MAX_VALUE
for (seed in seeds){
    var value = seed
    var cat = "seed"
    while (cat != "location"){
        val map = maps[cat]!!
        value = map.get(value)
        cat = map.destination
    }
    if (value < lowestLoc){
        lowestLoc = value
    }
}
println("Part1: $lowestLoc")


