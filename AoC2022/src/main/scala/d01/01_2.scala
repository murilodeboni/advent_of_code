package d01

import scala.io.Source

object Part2 {
  def readInput(path: String): List[Int] = {
    var ar: List[Int] = List()
    var tmp = 0
    for (line <- Source.fromFile(path).getLines) {
      line match {
        case "" => {
          ar = ar :+ tmp
          tmp = 0
        }
        case _ => tmp = tmp + line.toInt
      }
    }
    ar :+ tmp
  }

  val input = readInput("input.txt").sortWith(_ > _)

  println(input.take(3).reduce(_ + _))
}
