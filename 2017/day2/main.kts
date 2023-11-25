#!/usr/bin/env -S kotlinc -script

fun minMaxDiff(line: String) : Int{
    var min = 10_000;
    var max = 0;

    line.split('\t')
        .map{it.toInt()}
        .forEach{
            if (it < min){
                min = it;
            }
            if (it > max){
                max = it;
            }
        };

    return max - min;
}

fun evenDivDiv(line: String) : Int{
    var found = false;
    var res: Int = 0;

    val list = line.split('\t')
        .map{it.toInt()};
    for ((idx, a) in list.withIndex()){
        for (b in list.subList(idx+1, list.size)){
            if (a > b){
                found = a % b == 0;
                res = a / b;
            } else {
                found = b % a == 0;
                res = b / a;
            }
            if (found){
                break;
            }
        }
        if (found){
            break;
        }
    }
    return res;
}

var res1 = 0;
var res2 = 0;

for (line in generateSequence{readLine()}){
    res1 += minMaxDiff(line);
    res2 += evenDivDiv(line);
}
println("Part1: $res1")
println("Part2: $res2")
