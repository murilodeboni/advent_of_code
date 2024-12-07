package d14

import scala.annotation.tailrec
import scala.io.Source

object PartOne14 extends App {

  case class Coordinate(x: Int, y: Int, isRock: Boolean = false, isSource: Boolean = false, hasSnow: Boolean = false)
  object Coordinate {
    @tailrec
    def getAllPaths(input: List[Coordinate], result: List[Coordinate] = List()): List[Coordinate] = {
      if (input.tail.nonEmpty) {
        getAllPaths(input.tail, result ++ getFullPath(input.head, input.tail.head))
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
      val source = Coordinate(500,0, isSource = true)
    def createMap(input: List[List[Coordinate]]): Map = {
      val rocks = input.map(i => Coordinate.getAllPaths(i)).reduce(_ ++ _).distinct
      fillMap(source +: rocks)
    }

    def findBoundariesMap(rocks: List[Coordinate]): (Int,Int,Int,Int) = {
      val boundaries = rocks
        .map(a => (a.x,a.y) match { case (x, y) => (x, x, y, y) })
        .reduce{ (a,b) => (a,b) match { case ((xl, xr, xb, xt), (yl, yr, yb, yt)) =>
          (xl min yl, xr max yr, xb min yb, xt max yt)
        }}
      boundaries match {
        case (left, right, up, down) => (left-1, right+1, up, down+1)
      }
    }

    def printMap(map: Map): Unit = {
      val sortedMap = map.coordinates.sortWith((c1,c2) => c1.x < c2.x).sortWith((c1,c2) => c1.y < c2.y)

      var line: String = ""
      var y = -1
      sortedMap.foreach(
        c => {
          if (c.y > y) {
            y = c.y
            line = line + "\n"
          }

          (c.hasSnow, c.isRock, c.isSource) match {
          case (true, _, _) => line = line + "o"
          case (_, true, _) => line = line + "#"
          case (_, _, true) => line = line + "+"
          case _ => line = line + "."
        }
        }
      )

      println(line)
    }

    def fillMap(initialCoordinates: List[Coordinate]): Map = {
      var otherCoordinates: List[Coordinate] = List()
      val (left, right, up, down) = Map.findBoundariesMap(initialCoordinates)
      (left to right).map(
        ix => (up to down).map(
          jy => initialCoordinates.find(c => c.x == ix && c.y == jy) match {
            case None => otherCoordinates = otherCoordinates :+ Coordinate(ix, jy)
            case Some(c) => // already exists
          }
        )
      )
      val fullMap = Map(initialCoordinates ++ otherCoordinates)
      fullMap
    }

    def countSnow(map: Map): Int = {
      map.coordinates.count(_.hasSnow)
    }
  }

  val inputText: List[List[Coordinate]] = Source.fromFile(name = "./input.txt")
    .getLines
    .toList
    .map(line => line.split("->").map(coordinate => {
      val c = coordinate.split(",")
      Coordinate(c(0).trim.toInt, c(1).trim.toInt)
    }).toList)

  def isFull(x: Int,y: Int, map: Map): Boolean = {
    map.coordinates.find(c => c.x == x && c.y == y && (c.isRock || c.isSource || c.hasSnow)) match {
      case None => false
      case Some(c) => true
    }
  }

  @tailrec
  def putSnow(x: Int, y:Int, map: Map, stop: Int): Map = {
    val downLeft = Coordinate(x-1,y+1)
    val downRight = Coordinate(x+1,y+1)


    if (!map.coordinates.exists(c=> c.x == x && c.y == y+1)) {
      println(s"This is the final map with snow, with ${Map.countSnow(map)} snowballs")
      Map.printMap(map)
      map
    } else if (!isFull(x,y+1,map) && y+1 <= stop) {
      putSnow(x,y+1,map,stop)
    } else if (!isFull(downLeft.x,downLeft.y,map)) {
      putSnow(downLeft.x, downLeft.y, map, stop)
    } else if (!isFull(downRight.x,downRight.y,map)) {
      putSnow(downRight.x, downRight.y, map, stop)
    } else {
      val newMap = Map(map.coordinates.filter(c => c.x != x || c.y != y) :+ Coordinate(x = x, y = y, hasSnow = true))
      putSnow(Map.source.x, Map.source.y, newMap, stop)
    }
  }

  val map = Map.createMap(inputText)
  val (_, _, _, down) = Map.findBoundariesMap(map.coordinates)
  putSnow(Map.source.x, Map.source.y, map, down)
}