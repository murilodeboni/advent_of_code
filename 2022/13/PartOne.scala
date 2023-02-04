import java.io.File
import scala.annotation.tailrec
import scala.io.Source

object PartOne extends App {
  val inputText: List[String] = Source.fromFile(name = "./2022/13/input.txt")
    .getLines
    .toList
    .filter(_ != "")
    .map(s => s.replace("10","A").replace(",",""))

  val numberPairs: Int = inputText.length / 2

  val pairs: List[(String, String)] = (0 until numberPairs).map(
    n => (inputText(2 * n), inputText(2 * n + 1))
  ).toList

  @tailrec
  def compare(left: String, right: String): Boolean = {
    /*
    https://github.com/maneatingape/advent-of-code-scala/blob/main/src/main/scala/AdventOfCode2022/Day13.scala
     */
    (left.head, right.head) match {
      case (a, b) if a == b => compare(left.tail, right.tail)
      case (']', _) => true
      case (_, ']') => false
      case ('[', b) => compare(left.tail, s"$b]" + right.tail)
      case (a, '[') => compare(s"$a]" + left.tail, right.tail)
      case (a, b) => a < b
    }
  }

  val ans = pairs.zipWithIndex.map { case (s, i) => if (compare(s._1, s._2)) i+1 else 0 }

  println(ans.sum)
}