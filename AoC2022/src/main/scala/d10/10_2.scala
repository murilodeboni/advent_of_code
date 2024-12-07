package d10

import scala.io.Source

object Part2 {
  val input = Source.fromFile("input.txt").getLines.toList.map(x => {
    x match {
      case "noop" => (1, 0)
      case _ => (2, x.split(" ")(1).toInt)
    }
  }).toList

  def registerPixel(cycle: Int, x: Int): String = {
    val drawHashtag = (x % 40 >= (cycle - 1) % 40 - 1 && x % 40 <= (cycle - 1) % 40 + 1)
    // println(cycle, x, drawHashtag)
    if (drawHashtag) "#" else "."
  }

  def parseInstructions(i: List[(Int, Int)], cycle: Int = 0, x: Int = 1, screen: List[String] = List()): List[String] = {
    val newCycle = cycle + 1
    val newScreen = screen :+ registerPixel(newCycle, x)

    i.headOption match {
      case None => screen
      case Some((1, a)) => {
        val newX = x + a
        parseInstructions(i.tail, newCycle, newX, newScreen)
      }
      case Some((c, s)) => {
        parseInstructions((c - 1, s) +: i.tail, newCycle, x, newScreen)
      }
    }
  }

  parseInstructions(input).grouped(40).toList.map(_.mkString).map(println)
}