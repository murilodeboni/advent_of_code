import scala.io.Source

def readInput(path: String): List[String] = {
  var ar:List[String] = List()
  for (line <- Source.fromFile(path).getLines) {
    ar = ar :+ line 
  }
  ar
}

val input = readInput("input.txt")

val letters = 'a' to 'z' 

val priorities = (letters.zipWithIndex.map(e => (e._1, e._2+1)) ++ letters.zipWithIndex.map(e => (e._1.toUpper, e._2+27))).toMap

def identifyGroup(groups: List[String], ans: Int = 0): Int = {
  if (groups == List()){
    ans
  } else {
    val group = groups.take(3)
    val badge = group(0) intersect group(1) intersect group(2)
    identifyGroup(groups.drop(3),ans + priorities(badge(0)))
  }
}

println(identifyGroup(input))
