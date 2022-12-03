import scala.io.Source

def readInput(path: String): List[(String, String)] = {
  var ar:List[(String, String)] = List()
  var tmp: (String, String) = ("", "")
  for (line <- Source.fromFile(path).getLines) {
    tmp = line.splitAt(line.size/2)
    ar = ar :+ tmp
  }
  ar
}
val input = readInput("input.txt")

val letters = 'a' to 'z' 

val priorities = (letters.zipWithIndex.map(e => (e._1, e._2+1)) ++ letters.zipWithIndex.map(e => (e._1.toUpper, e._2+27))).toMap

val result = input.map(
  rs => (rs._1 intersect rs._2).toSet.map(s => priorities(s)).reduce(_ + _)
).reduce(_ + _)

println(result)
