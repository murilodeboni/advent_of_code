import scala.io.Source

val input = Source.fromFile("input.txt").getLines.toList.map(_.split("").map(_.toInt).toList)

val ys = input.size
val xs = input.head.size

val y = 1 to (ys - 2)
val x = 1 to (xs - 2)

def viewingDistance(height: Int, trees: List[Int], distance: Int = 0): Int = {
  trees.headOption match {
    case None => distance
    case Some(t) => if (height > t) viewingDistance(height, trees.tail, distance+1) else distance +1
  }
}

def checkVisibility(i: Int, j: Int, treeMap: List[List[Int]] = input): Int = {
  val tree = treeMap(i)(j)
  
  val left = viewingDistance(tree, treeMap(i).take(j).reverse)
  val right = viewingDistance(tree, treeMap(i).takeRight(xs-j-1))
  val up = viewingDistance(tree, treeMap.map(_(j)).take(i).reverse)
  val down = viewingDistance(tree, treeMap.map(_(j)).takeRight(ys-i-1))

  left * right * up * down
}

var ans = 0

for {
  i <- x.toList
  j <- y.toList
} yield ans = scala.math.max(ans,checkVisibility(i,j))

println(ans)

