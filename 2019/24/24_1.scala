import scala.io.Source

def readInput(path: String): List[List[String]] = {
  var ar:List[List[String]] = List()
  for (line <- Source.fromFile(path).getLines) {
    ar = ar ::: List(line.split("").toList) 
  }
  ar
}

val input = readInput("input.txt")

println(input(2)(2))
