import PartOne15.Area.Area

import scala.annotation.tailrec
import scala.io.Source
import scala.math.abs
import scala.util.matching.Regex

object PartOne15 extends App {
  val inputText= Source.fromFile(name = "./2022/15/input.txt")
    .getLines
    .toList

  object Area extends Enumeration {
    type Area = Value
    val Sensor, Beacon, Unknown = Value
  }

  case class Coordinate(x: Int, y: Int, areaType: Area)
  case class Read(sensor: Coordinate, closestBeacon: Coordinate)

  val regexAll: Regex = "Sensor at x=(-?\\d*), y=(-?\\d*): closest beacon is at x=(-?\\d*), y=(-?\\d*)".r
  @tailrec
  def parseInputToTuples(input: List[String], result: List[Read] = List()): List[Read] = {
    input.headOption match {
      case None => result
      case Some(h) =>
        val regexAll(sx, sy, bx, by) = h
        val (sxi, syi, bxi, byi) = (sx.toInt, sy.toInt, bx.toInt, by.toInt)
        val newReads = List(Read(
          Coordinate(sxi, syi, Area.Sensor),
          Coordinate(bxi, byi, Area.Beacon)
        )) ++ result

        parseInputToTuples(input.tail, newReads)
    }
  }

  def getDistance(c1x: Int, c1y: Int, c2: Coordinate): Int = {
    abs(c1x - c2.x) + abs(c1y - c2.y)
  }

  def clearSurroundings(read: Read, map: List[Int], goal: Int): List[Int] = {
    val radius = abs(read.sensor.x - read.closestBeacon.x) + abs(read.sensor.y - read.closestBeacon.y)
    val xClearRange = (read.sensor.x - radius) to (read.sensor.x + radius)
    val yRangeContainsGoal = (read.sensor.y - radius) <= goal && goal <= (read.sensor.y + radius)

    var tmp: List[Int] = List()

    if(yRangeContainsGoal) {
      xClearRange.foreach(
        i => {
          println(i/xClearRange.size)
          if (!map.contains(i) && getDistance(i, goal, read.sensor) <= radius) {
            tmp = tmp :+ i
          }
        }
      )
    }

    tmp
  }

  @tailrec
  def mapClearedAreas(reads: List[Read], map: List[Int], result: List[Int] = List(), goal: Int = goal): List[Int] = {
    println(reads.size)
    reads.headOption match {
      case None => result
      case Some(r) => mapClearedAreas(reads.tail, map, result ++ clearSurroundings(r,map, goal))
    }
  }

//  val goal: Int = 10
  val goal: Int = 2000000

  val initialReads: List[Read] = parseInputToTuples(inputText)
  val initialMap: List[Int] = initialReads.map(r => List(r.sensor,r.closestBeacon)).reduce(_ ++ _).filter(_.y == goal).map(_.x)
  val finalMap: List[Int] = mapClearedAreas(initialReads, initialMap)

  println(finalMap.toSet.size)
}