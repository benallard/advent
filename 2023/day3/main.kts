#!/usr/bin/env -S kotlinc -J-ea -script
println("ready")

class Schematics(val spots: List<List<Char>>){

    fun part1(): Int{
        var res = 0;
        for (y in spots.indices){
            var prevDigit = false;
            var start = -1;
            for (x in spots[y].indices){
                val isDigit = spots[y][x].isDigit();
                if (prevDigit != isDigit){
                    if (isDigit){
                        start = x;
                    } else {
                        if (!isolated(y, start, x-1)){
                            val partNr = spots[y].subList(start, x).joinToString(separator = "").toInt()
                            res += partNr
                            println("$x, $y: $partNr")
                        }
                    }
                }
                prevDigit = isDigit
            }
            if (prevDigit){
                if (!isolated(y, start, spots[y].size)){
                    res += spots[y].subList(start, spots[y].size).joinToString(separator = "").toInt();
                }
            }
        }
        return res;
    }

    private fun isolated(line: Int, start: Int, end: Int): Boolean{
        for (y in line - 1..line + 1){
            if (y < 0 || y >= spots.size) {
                continue
            }
            for (x in start-1..end+1){
                if ( x < 0 || x >= spots[y].size){
                    continue
                }
                if ( y == line && x >= start && x <= end){
                    continue
                }
                if (spots[y][x] != '.'){
                    println("$x, $y, ${spots[y][x]}")
                    return false
                }
            }

        }
        return true;
    }

}

fun readSchematics() : Schematics{
    val spots = ArrayList<List<Char>>();
    for (line in generateSequence{readLine()}){
        spots.add(line.toList())
    }
    return Schematics(spots);
}

var schematic = readSchematics();

println("Part1: ${schematic.part1()}")