import scala.io.Source

val input = Source.fromFile("input.txt").getLines.toList.map(x => {
  val l = x.split(" ")
  (l(0),l(1).toInt)
})

def tailMove(oldRope:List[(Int,Int)], newRope:List[(Int,Int)]):List[(Int,Int)] = {
  if (oldRope == List()) { newRope } else {
  val h = newRope.last
  val t = oldRope.head

  val horDiff = h._1 - t._1
  val verDiff = h._2 - t._2

  val newTail = (horDiff,verDiff) match {
    case (2,2) => (t._1+1,t._2+1)
    case (-2,-2) => (t._1-1,t._2-1)
    case (2,_) => (t._1+1,h._2)
    case (-2,_) => (t._1-1,h._2)
    case (_,2) => (h._1, t._2+1)
    case (_,-2) => (h._1, t._2-1)
    case _ => {
      println(h,t)
      t
    }
  }
  tailMove(oldRope.drop(1), newRope :+ newTail)
}}

def moveHead(input:List[(String,Int)], rope:List[(Int,Int)] = List.fill(10)(0,0), hist: List[(Int, Int)] = List()): Int = {
  input.headOption match {
    case None => {
      hist.toSet.size
    }
    case Some(i) => {
      val h = rope.head
      val newHead = i._1 match {
        case "R" => (h._1 + 1, h._2)
        case "L" => (h._1 - 1, h._2)
        case "U" => (h._1, h._2 + 1)
        case "D" => (h._1, h._2 - 1)
      }
      if (i._2 > 0) {
        val newRope = tailMove(rope.tail,List(newHead))
        moveHead(List((i._1,i._2-1)) ++ input.tail, newRope, hist :+ newRope.last) 
      } else {
        moveHead(input.tail, rope,  hist) 
      }
    }
  }
}

println(
  moveHead(input)
)
