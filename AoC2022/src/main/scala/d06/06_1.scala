package d06

import scala.io.Source

object Part1 extends App {
  def readInput(path: String): Int = {
    var ar: Int = 0
    var tmp: Array[String] = Array()
    val fs = Source.fromFile(path)

    while (fs.hasNext && tmp.toSet.size < 4) {
      if (ar < 4) {
        tmp = tmp.appended(fs.next.toString)
      } else {
        tmp = tmp.drop(1).appended(fs.next.toString)
      }
      ar += 1
    }
    ar
  }

  println(readInput("input.txt"))
}