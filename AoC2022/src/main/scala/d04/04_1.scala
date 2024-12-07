package d04

import scala.io.Source

object Part1 {
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
        val elfOne = input(0)
        val elfTwo = input(1)
        val all = elfOne ++ elfTwo
        val coverage = Array(all.min, all.max)
        if (coverage.sameElements(elfOne) || coverage.sameElements(elfTwo)) {
          checkPair(input.drop(2), ans + 1)
        } else {
          checkPair(input.drop(2), ans)
        }
      }
    }
  }

  println(checkPair(input))
}