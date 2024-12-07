package d10

import scala.io.Source

object Part1 {
  val input = Source.fromFile("input.txt").getLines.toList.map(x => {
    x match {
      case "noop" => (1, 0)
      case _ => (2, x.split(" ")(1).toInt)
    }
  }).toList

  def registerSignal(cycle: Int, signal: Int): Int = {
    //println(cycle, signal, cycle*signal)
    if (cycle == 20) {
      //println(cycle, signal, cycle*signal)
      cycle * signal
    } else if ((cycle - 20) % 40 == 0) {
      //println(cycle, signal, cycle*signal)
      cycle * signal
    } else {
      0
    }
  }

  def parseInstructions(i: List[(Int, Int)], cycle: Int = 0, x: Int = 1, signalStrength: Int = 0): Int = {
    i.headOption match {
      case None => signalStrength
      case Some((1, a)) => {
        val newStrength = signalStrength + registerSignal(cycle + 1, x)
        val newCycle = cycle + 1
        val newX = x + a
        parseInstructions(i.tail, newCycle, newX, newStrength)
      }
      case Some((c, s)) => {
        val newStrength = signalStrength + registerSignal(cycle + 1, x)
        val newCycle = cycle + 1
        parseInstructions((c - 1, s) +: i.tail, newCycle, x, newStrength)
      }
    }
  }

  println(
    parseInstructions(input)
  )
}