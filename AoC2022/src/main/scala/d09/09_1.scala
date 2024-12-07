package d09

import scala.io.Source

object Part1 {
  val input = Source.fromFile("input.txt").getLines.toList.map(x => {
    val l = x.split(" ")
    (l(0), l(1).toInt)
  })

  def tailMove(h: (Int, Int), t: (Int, Int)): (Int, Int) = {
    val horDiff = h._1 - t._1
    val verDiff = h._2 - t._2

    (horDiff, verDiff) match {
      case (2, _) => (t._1 + 1, h._2)
      case (-2, _) => (t._1 - 1, h._2)
      case (_, 2) => (h._1, t._2 + 1)
      case (_, -2) => (h._1, t._2 - 1)
      case _ => t
    }
  }

  def moveHead(input: List[(String, Int)], h: (Int, Int) = (0, 0), t: (Int, Int) = (0, 0), hist: List[(Int, Int)] = List()): Int = {
    input.headOption match {
      case None => hist.toSet.size
      case Some(i) => {
        val newHead = i._1 match {
          case "R" => (h._1 + 1, h._2)
          case "L" => (h._1 - 1, h._2)
          case "U" => (h._1, h._2 + 1)
          case "D" => (h._1, h._2 - 1)
        }

        if (i._2 > 0) {
          val newTail = tailMove(newHead, t)
          moveHead(List((i._1, i._2 - 1)) ++ input.tail, newHead, newTail, hist :+ newTail)
        } else {
          moveHead(input.tail, h, t, hist)
        }
      }
    }
  }

  println(
    moveHead(input)
  )
}