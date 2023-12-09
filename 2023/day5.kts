#!/usr/bin/env -S kotlinc -J-ea -script
println("ready")

val seeds = readln().substring("seeds: ".length)
        .split(" ")
        .map { it.toLong() }
        .toSet()
readln()

class CategoryMap(val source: String, val destination: String){
    private var mappings = HashSet<Range>()

    fun fillMap(dest: Long, src: Long, len: Int){
        mappings.add(Range(src, dest, len))
    }

    fun get(value: Long): Long{
        for (mapping in mappings){
            if (mapping.matches(value)){
                return mapping.get(value)
            }
        }
        return value
    }

    class Range(val source: Long, val destination: Long, val length: Int){
        fun matches(value: Long): Boolean{
            return value > source && value <= source + length
        }
        fun get(value: Long): Long{
            return destination + (value - source)
        }
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
        catMap.fillMap(nrs[0].toLong(), nrs[1].toLong(), nrs[2].toInt())
    }
    return catMap
}

val maps = HashMap<String, CategoryMap>()

for (map in generateSequence { readMap() }){
    maps[map.source] = map
    if (map.destination == "location"){
        break
    }
}

println("Seeds: $seeds")

var lowestLoc = Long.MAX_VALUE
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
    println("Processed seed $seed: location=$value")
}
println("Part1: $lowestLoc")


