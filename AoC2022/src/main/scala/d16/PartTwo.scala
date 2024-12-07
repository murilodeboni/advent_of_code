package d16

import scala.annotation.tailrec
import scala.io.Source
import scala.util.matching.Regex

object PartTwo16 extends App {

  case class Valve(code: String, rate: Int, leadsTo: List[String])
  case class Cave(valves: List[Valve], currentValve: Valve) {
    def findValve(code: String): Valve = {
      valves.find(_.code == code).getOrElse(throw new Exception(f"Valve $code not found"))
    }

    def removeValve(code: String): Cave = {
      this.copy(valves = valves.filter(_.code != code))
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

  val listInput = loadInput("./test_input.txt")
  val valves = parseInput(listInput)
  val cave = Cave(valves, valves.head)
  val distances: Map[(String, String), Int] = cave.getInitialDistances.filter{case (k,_) => k._1 != k._2}

  val s: String = valves.find(_.code == "AA").get.code
  val flows: Map[String, Int] = valves.map(v => Map(v.code -> v.rate)).reduce(_ ++ _)
  val valvesString: List[String] = valves.map(_.code).filter(flows(_) > 0)

  def run(t: Int = 26, s: String = s, valves: List[String] = valvesString, elephantStart: Boolean = false): Int = {
    valves.filter(v=> distances((s,v))<t).map(
      v => {
        val d: Int = distances((s,v))
        val nt: Int = t - d - 1
        flows(v)*nt + run(nt, v, valves.filter(_!=v)) + (if (elephantStart) run() else 0)
      }
    ).reduceOption(_.max(_)).getOrElse(0) //+ (if (elephantStart) run() else 0)
  }

  print(run(elephantStart = true))
}