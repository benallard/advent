#!/usr/bin/env -S kotlinc -J-ea -script
println("ready")

enum class Pixel(val display : Char){
    EMPTY('.'),
    GALAXY('#'),
    ;
}

fun toPixel(char :Char): Pixel{
    for (p in Pixel.entries){
        if (char == p.display){
            return p
        }
    }
    throw AssertionError(char)
}

class Image(private val content : List<List<Pixel>>){

    val galaxies = content.withIndex()
            .flatMap {(y, line) -> line.withIndex().filter { it.value == Pixel.GALAXY }.map { Coordinate(it.index, y) } }
            .toList()

    init{
        println("${galaxies.size} Galaxies")
    }


    fun emptyRow(y: Int): Boolean = content[y].filterNot { it == Pixel.EMPTY }.isEmpty()
    fun emptyCol(x: Int): Boolean = content.indices.map { content[it][x] }.filterNot { it == Pixel.EMPTY }.isEmpty()


    data class Coordinate(val x: Int, val y: Int)

}

val image = Image(generateSequence { readlnOrNull() }
        .map {s ->  s.map{toPixel(it)}.toList() }
        .toList())
