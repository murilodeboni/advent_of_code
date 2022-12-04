import scala.io.Source

def readInput(path: String): Array[Array[Int]] = {
  var ar: Array[Array[Int]] = Array()
  var tmp: Array[Array[Int]] = Array()
  for (line <- Source.fromFile(path).getLines) {
    tmp = line.split(",").map(_.split("-").map(_.toInt))
    ar = ar ++ tmp
  }
  ar
}

val input = readInput("input.txt")

def checkPair(input: Array[Array[Int]], ans: Int = 0): Int = {
  input match {
    case Array() => ans
    case _ => {
      val elfOne = Range(input(0)(0),input(0)(1)+1)
      val elfTwo = Range(input(1)(0),input(1)(1)+1)
      val overlaps = elfOne.contains(elfTwo.min) || elfOne.contains(elfTwo.max) || elfTwo.contains(elfOne.min) || elfTwo.contains(elfOne.max)
      if (overlaps) {
        checkPair(input.drop(2), ans+1)
      } else {
        checkPair(input.drop(2),ans)
      }
    }
  }
}

println(checkPair(input))
