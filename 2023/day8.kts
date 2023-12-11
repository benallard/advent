#!/usr/bin/env -S kotlinc -J-ea -script
println("ready")

class Element(val name: String, val left: String, val right: String){

    fun get(side: Char) : String{
        return if (side == 'L'){
            left
        } else {
            right
        }
    }

}

fun readElement(aLine: String): Element{
    val parsed = "(.{3}) = \\((.{3}), (.{3})\\)".toRegex().matchEntire(aLine)!!.groups
    return Element(parsed[1]!!.value, parsed[2]!!.value, parsed[3]!!.value)
}

val instructions = readln()
readln()

val network = HashMap<String, Element>()

for (line in generateSequence { readlnOrNull() }){
    val elem = readElement(line)
    network[elem.name] = elem
}
println("Network: ${network.keys}")

var instrIdx = 0
var steps = 0L
if (false) {
    var current = "AAA"
    instrIdx = 0
    steps = 0
    while (current != "ZZZ") {
        val instruction = instructions[instrIdx]
        instrIdx = (instrIdx + 1) % instructions.length
        println("Currently at $current")
        current = network[current]!!.get(instruction)
        steps++
    }
    println("Part1: $steps")
}

var currents = network.keys
        .filter{ it[2] == 'A'}
        .toMutableList()
var lengths = IntArray(currents.size){ 0 }
instrIdx = 0
steps = 0L
while(lengths.any { it == 0 }){
    val instruction = instructions[instrIdx]
    instrIdx = (instrIdx + 1) % instructions.length
    for ((idx, cur) in currents.withIndex()){
        currents[idx] = network[cur]!!.get(instruction)
    }
    steps++
    for ((idx, cur) in currents.withIndex()){
        if (cur[2] == 'Z' && lengths[idx] == 0){
            lengths[idx] = steps.toInt()
        }
    }
    //println("Currently at $currents")
}

fun gcd(a: Long, b: Long): Long{
    if (b == 0L) {

        return a
    }
    return gcd(b, a % b)
}

fun findlcm(arr: IntArray,): Long {
    var res: Long = arr[0].toLong()

    // ans contains LCM of arr[0], ..arr[i]
    // after i'th iteration,
    for (x in arr){
        res = (x * res) /
                gcd(x.toLong(), res)
    }

    return res
}


println("Part2: ${findlcm(lengths)}")