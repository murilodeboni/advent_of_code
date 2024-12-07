package d12

import scala.io.Source

//object Part2 {
//  val input: List[List[String]] = Source.fromFile("input.txt").getLines.toList.map(_.split("").toList)
//
//  val letterMap = ('a' to 'z').zipWithIndex.map {
//    case (s, n) => Map(s.toString -> (n + 1))
//  }.reduce(_ ++ _) ++ Map("S" -> 0, "E" -> 99)
//
//  val bigNumber = 999
//
//  val inputMap = input.map(_.map(letterMap(_)))
//
//  case class Vertice(i: Int, j: Int, value: Int, distance: Int = bigNumber, s: Boolean = false, e: Boolean = false)
//
//  val imax = inputMap.length
//  val jmax = inputMap(0).length
//
//  val vertices: List[Vertice] = (0 until imax).map(
//    i => (0 until jmax).map(
//      j => {
//  val value = inputMap(i)(j)
//  value match {
//    case 0 => Vertice(i, j, 1, bigNumber, false)
//    case 99 => Vertice(i, j, 26, bigNumber, false, true)
//    case _ => Vertice(i, j, value)
//  }}
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
//    if (unexplored.isEmpty) {
//      throw new Exception("Could not find path")
//      val currentVertice = unexplored.filter(v => v.s).headOption.getOrElse(unexplored.minBy(_.distance))
//      if (currentVertice.e) {
//        currentVertice.distance
//      } else {
//        val viableNeighbours = getNeighbours(currentVertice.j, currentVertice.i, unexplored)
//          .filter(n => n.value - currentVertice.value <= 1)
//        val newNeighbours = viableNeighbours.map(n => n.copy(distance = currentVertice.distance + 1))
//        val newUnexplored = unexplored
//          .filter(n => n != currentVertice & !viableNeighbours.contains(n))
//          .union(newNeighbours)
//        dijkstra(newUnexplored)
//      }
//    }
//  }
//
//  lazy val ans = vertices.filter(v => v.value == 1).map(
//    v =>
//  val newStart = v.copy(distance = 0, s = true)
//  val tmp = dijkstra(newStart +: vertices.filter(ov => ov.j != newStart.j || ov.i != newStart.i))
//  println(f"$newStart $tmp")
//  tmp
//  ).reduce((x, y) => x min y)
//
//  @main def entryPoint() =
//    println(ans)
//  //println(
//  //vertices.filter(v => v.value == 1).map(
//  //v => vertices.filter(ov => ov.i != v.i || ov.j != v.j) :+ v.copy(distance = 0, s = true)
//  //).map(_.filter(nv => nv.s))
//  //)
//}