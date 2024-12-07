package d12

import scala.io.Source

//object Part1 {
//  val input: List[List[String]] = Source.fromFile("input.txt").getLines.toList.map(_.split("").toList)
//
//  val letterMap: Any = ('a' to 'z').zipWithIndex.map {
//    (s, n) => Map(s.toString -> (n + 1))
//  }.reduce(_ ++ _) ++ Map("S" -> 0, "E" -> 99)
//
//
//  val inputMap = input.map(_.map(letterMap(_)))
//
//  case class Vertice(i: Int, j: Int, value: Int, distance: Int = Int.MaxValue, s: Boolean = false, e: Boolean = false)
//
//  val imax = inputMap.length
//  val jmax = inputMap(0).length
//
//  val vertices: List[Vertice] = (0 until imax).map(
//    i => (0 until jmax).map(
//      j =>
//  val value = inputMap(i)(j)
//  value match {
//    case 0 => Vertice(i, j, 1, 0, true)
//    case 99 => Vertice(i, j, 26, Int.MaxValue, false, true)
//    case _ => Vertice(i, j, value)
//  }
//  ).toList
//  ).toList.reduce(_ ++ _)
//
//  def findVertice(x: Int, y: Int, vertices: List[Vertice]): Option[Vertice] = vertices.find(v => v.j == x & v.i == y)
//
//
//  def getNeighbours(x: Int, y: Int, vertices: List[Vertice]): List[Vertice] = {
//    List((x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1))
//      .flatMap(xy => findVertice(xy(0), xy(1), vertices))
//      .toList
//  }
//
//  @scala.annotation.tailrec
//  def dijkstra(unexplored: List[Vertice]): Int = {
//    if unexplored.isEmpty then
//    throw new Exception("Could not find path")
//    val currentVertice = unexplored.minBy(_.distance)
//    if currentVertice.e then currentVertice.distance else {
//      val viableNeighbours = getNeighbours(currentVertice.j, currentVertice.i, unexplored)
//        .filter(n => n.value - currentVertice.value <= 1)
//      val newNeighbours = viableNeighbours.map(n => n.copy(distance = currentVertice.distance + 1))
//      val newUnexplored = unexplored
//        .filter(n => n != currentVertice & !viableNeighbours.contains(n))
//        .union(newNeighbours)
//      dijkstra(newUnexplored)
//    }
//  }
//
//  @main def entryPoint() =
//    println(dijkstra(vertices))
//}