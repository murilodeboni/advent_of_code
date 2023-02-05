import PartOne.Map.findBoundariesMap

import scala.annotation.tailrec
import scala.io.Source

object PartOne extends App {

  case class Coordinate(x: Int, y: Int, isRock: Boolean = false, isSource: Boolean = false, hasSnow: Boolean = false)
  object Coordinate {
    @tailrec
    def getAllPaths(input: List[Coordinate], result: List[Coordinate] = List()): List[Coordinate] = {
      if (input.tail.nonEmpty) {
        getAllPaths(input.tail, getFullPath(input.head, input.tail.head))
      } else {
        result
      }
    }

    def getFullPath(from: Coordinate, to: Coordinate): List[Coordinate] = {
      val dx = to.x - from.x
      val dy = to.y - from.y

      (dx, dy) match {
        case (0, 0) => List(from)
        case (0, _) => (from.y to to.y by dy / math.abs(dy)).map(Coordinate(from.x, _, isRock = true)).toList
        case (_, 0) => (from.x to to.x by dx / math.abs(dx)).map(Coordinate(_, from.y, isRock = true)).toList
        case _ => throw new Exception(f"dx = $dx and dy = $dy")
      }
    }
  }


  case class Map(coordinates: List[Coordinate])
  object Map {
    def createMap(input: List[List[Coordinate]]): Map = {
      val rocks = input.map(i => Coordinate.getAllPaths(i)).reduce(_ ++ _)
      new Map(rocks)
    }

    def findBoundariesMap(rocks: List[Coordinate]): (Int,Int,Int,Int) = {
      val boundaries = rocks
        .map(a => (a.x,a.y) match { case (x, y) => (x, x, y, y) })
        .reduce{ (a,b) => (a,b) match { case ((xl, xr, xb, xt), (yl, yr, yb, yt)) =>
          (xl min yl, xr max yr, xb min yb, xt max yt)
        }}

      boundaries match {
        case (down, up, left, right) => (down -1, up, left - 1, right +1)
      }

    }
  }

  val inputText: List[List[Coordinate]] = Source.fromFile(name = "./2022/14/test_input.txt")
    .getLines
    .toList
    .map(line => line.split("->").map(coordinate => {
      val c = coordinate.split(",")
      Coordinate(c(0).trim.toInt, c(1).trim.toInt)
    }).toList)

  val map = Map.createMap(inputText)

  println(findBoundariesMap(map.coordinates))

}