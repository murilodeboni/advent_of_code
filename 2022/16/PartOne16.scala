import scala.annotation.tailrec
import scala.io.Source
import scala.math
import scala.util.matching.Regex

object PartOne16 extends App {

  val timeLimit: Int = 30

  case class Valve(code: String, rate: Int, leadsTo: List[String], timeOpened: Int = timeLimit, isOpened: Boolean = false)
  case class Cave(valves: List[Valve], currentValve: Valve, time: Int = 0, rate: Int = 0) {
    def findValve(code: String): Valve = {
      valves.find(_.code == code).getOrElse(throw new Exception(f"Valve $code not found"))
    }

    def goTo(to: Valve, debug: Boolean): Cave = {
      if (currentValve.leadsTo.contains(to.code)) {
        if (debug) println(f"goign from ${currentValve} to ${to}")
        Cave(valves, to, time + 1, rate)
      } else {
        throw new Exception(f"Movement not possible (from ${currentValve.code} to ${to.code})")
      }
    }

    def getRate: Int = {
      valves.filter(_.timeOpened < time).map(v => v.rate * (time - v.timeOpened)).sum
    }

    def openValve(debug: Boolean): Cave = {
      if (currentValve.isOpened) {
        throw new Exception("Valve already opened")
      } else {
        val newValve = currentValve.copy(timeOpened = time, isOpened = true)
        val newValves = valves.filter(_.code != currentValve.code) :+ newValve

        if (debug) println(f"opening ${currentValve}")
        this.copy(valves = newValves, currentValve = newValve, time = time + 1, rate = getRate)
      }
    }

    // Floyd-Marshall
    def getInitialDistances:Map[(String, String), Int] = {
      val V = valves.map(_.code)
      val valveInit: Map[(String, String), Int] = valves.map(vi => vi.leadsTo.map(vf => Map((vi.code, vf) -> 1)).reduce(_ ++ _)).reduce(_ ++ _)
      var valveAll: Map[(String, String), Int] = valveInit
      for { x <- V; y <- V; z <- V} yield {
        valveAll = valveAll ++ Map((y,z) -> math.min(valveAll.getOrElse((y, z), 1000), valveAll.getOrElse((y, x), 1000) + valveAll.getOrElse((x,z), 1000)))
      }
      valveAll.filter(_._2 < 1000)
    }
  }



  def loadInput(input: String): List[String] = {
    Source.fromFile(name = input)
      .getLines
      .toList
  }

  @tailrec
  def parseInput(input: List[String], result: List[Valve] = List()): List[Valve] = {
    val regexAll: Regex = "Valve ([A-Z]*) has flow rate=(\\d*); tunnels? leads? to valves? (.*)".r
    input.headOption match {
      case None => result
      case Some(h) =>
        val regexAll(valve, rate, leadsTo) = h
        val newValves = Valve(valve, rate.toInt,leadsTo.split(",").map(_.trim).toList) +: result
        parseInput(input.tail, newValves)
    }
  }

  def algo(cave: Cave, debug: Boolean = false): Cave = {
    if (debug) println(f"time: ${cave.time}, rate: ${cave.getRate}")
    cave.time match {
      case PartOne16.timeLimit => cave
      case _ => if (cave.currentValve.isOpened) {
        algo(cave.goTo(cave.findValve(cave.currentValve.leadsTo.head),  debug), debug)
      } else {
        algo(cave.openValve(debug), debug)
      }
    }

  }

  def run(isTest: Boolean = false, input: String = "./2022/16/"): Unit = {
    val listInput = loadInput(input + (if (isTest) "test_input.txt" else "input.txt"))

    val valves = parseInput(listInput)

    val cave = Cave(valves, valves.head)

//    algo(cave, true)

    println(cave.getInitialDistances)
  }

  run(true)
}