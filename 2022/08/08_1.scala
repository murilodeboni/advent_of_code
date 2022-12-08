import scala.io.Source

val input = Source.fromFile("input.txt").getLines.toList.map(_.split("").map(_.toInt).toList)

val ys = input.size
val xs = input.head.size

val y = 1 to (ys - 2)
val x = 1 to (xs - 2)

def checkVisibility(i: Int, j: Int, treeMap: List[List[Int]] = input): Int = {
  val tree = treeMap(i)(j)
  
  val left = treeMap(i).take(j).map(_ < tree).reduce(_ && _)
  val right = treeMap(i).takeRight(xs-j-1).map(_ < tree).reduce(_ && _)
  val up = treeMap.map(_(j)).take(i).map(_ < tree).reduce(_ && _)
  val down = treeMap.map(_(j)).takeRight(ys-i-1).map(_ < tree).reduce(_ && _)

  if (left || right || up || down) 1 else 0
}

var ans = 2*ys + 2*xs - 4

for {
  i <- x.toList
  j <- y.toList
} yield ans+=checkVisibility(i,j)

println(ans)

