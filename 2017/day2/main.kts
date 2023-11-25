
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

var res = 0;

for (line in generateSequence{readLine()}){
    res += minMaxDiff(line);
}
println("Part1: $res")
