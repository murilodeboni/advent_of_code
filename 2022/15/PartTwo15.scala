import PartOne15.Area.{Area, Beacon}

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
    val Sensor, Beacon = Value
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

  def clearSurroundings(read: Read, y: Int): List[Range] = {
    val (sx, sy) = (read.sensor.x, read.sensor.y)
    val radius = abs(sx - read.closestBeacon.x) + abs(sy - read.closestBeacon.y)
    val yRangeContainsGoal = (sy - radius) <= y && y <= (sy + radius)
    if(yRangeContainsGoal) {
      val dy = abs(sy - y)
      val range = ((sx - radius + dy) to (sx + radius - dy))

      //      val hasBeaconY = read.closestBeacon.y==y && range.contains(read.closestBeacon.x)
      //      if (hasBeaconY) {
      //        val ranges = range.splitAt(range.indexOf(read.closestBeacon.x))
      //        List(ranges._1, ranges._2.tail).filter(_.nonEmpty)
      //      } else {
      List(range).filter(_.nonEmpty)
      //      }
    } else {
      List()
    }
  }

  @tailrec
  def mapClearedAreas(reads: List[Read], y: Int, result: List[Range] = List()): List[Range] = {
    reads.headOption match {
      case None => result
      case Some(r) => mapClearedAreas(reads.tail, y, Util.merge(result ++ clearSurroundings(r, y)))
    }
  }

  @tailrec
  def findHiddenBeacon(reads: List[Read], y: Int = goal): Coordinate = {
    val ans = mapClearedAreas(reads, y)
    if (ans.length > 1) {
      Coordinate(ans(0).max+1, y, Beacon)
    } else {
      findHiddenBeacon(reads,y-1)
    }
  }

  def calculateFrequency(coordinate: Coordinate): BigInt = {
    BigInt(coordinate.x)*goal + BigInt(coordinate.y)
  }

  //  val goal: Int = 20
  val goal: Int = 4000000

  val initialReads: List[Read] = parseInputToTuples(inputText)
  println(calculateFrequency(findHiddenBeacon(initialReads)))

}

// Copied from https://gist.github.com/dougdonohoe/9da37863439c9e12ba36cc1deacf6184
object Util {
  @tailrec
  def collapse(rs: List[Range], sep: List[Range] = Nil): List[Range] = rs match {
    case x :: y :: rest =>
      if (y.start - 1 > x.end) collapse(y :: rest, x :: sep)
      else collapse(Range.inclusive(x.start, x.end max y.end) :: rest, sep)
    case _ =>
      (rs ::: sep).reverse
  }

  def merge(rs: List[Range]): List[Range] = collapse(rs.sortBy(_.start))
}