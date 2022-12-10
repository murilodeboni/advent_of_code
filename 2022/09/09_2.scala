import scala.io.Source

val input = Source.fromFile("input.txt").getLines.toList.map(x => {
  val l = x.split(" ")
  (l(0),l(1).toInt)
})

val initialRope = List.fill(10)(0,0)

def moveTail(h:(Int,Int),t:(Int,Int)):(Int,Int) = {
  val hx = h._1
  val hy = h._2
  val tx = t._1
  val ty = t._2

  val dx = hx - tx
  val dy = hy - ty

  if (math.abs(dx) < 2 && math.abs(dy) < 2) {
    t
  } else if (math.abs(dx) > 1 && math.abs(dy) > 1) {
    (if (tx < hx) hx - 1 else hx + 1,if (ty < hy) hy - 1 else hy + 1)
  } else if (math.abs(dx) > 1) {
    (if (tx < hx) hx - 1 else hx + 1, hy)
  } else if (math.abs(dy) > 1) {
    (hx,if (ty < hy) hy - 1 else hy + 1)
  } else {
    t
  }
}

var hist: List[(Int,Int)] = List()

def bringTailToHead(rope: List[(Int,Int)],newRope: List[(Int,Int)]): List[(Int,Int)] = {
  rope match {
    case List() => {
      // println(newRope)
      hist = hist :+ newRope.last
      newRope
    }
    case _ => bringTailToHead(rope.drop(1), newRope :+ moveTail(newRope.last,rope.head))
  }
}

def moveHead(h:(Int,Int), rope: List[(Int,Int)], dir:String):List[(Int,Int)] = {
  val hx = h._1
  val hy = h._2
  
  val nh = dir match {
    case "R" => (hx + 1, hy)
    case "L" => (hx - 1, hy)
    case "U" => (hx, hy + 1)
    case "D" => (hx, hy - 1)
  }

  bringTailToHead(rope, List(nh))
}

def moveRope(input:List[(String,Int)], rope:List[(Int,Int)]): List[(Int,Int)] = {
  val movement = input.headOption

  movement match {
    case None => rope
    case Some((d, a))  => if (a>0) {
      moveRope(List((d,a-1)) ++ input.tail, moveHead(rope.head, rope.tail, d))
    } else {
      moveRope(input.tail, rope)
    }
  }
} 

moveRope(input, initialRope)

println(hist.toSet.size)
