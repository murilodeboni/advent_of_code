import java.io.File
import scala.annotation.tailrec
import scala.io.Source

object PartTwo extends App {
  val additionalText = List("[[2]]","[[6]]")
  val inputText: List[String] = Source.fromFile(name = "./2022/13/input.txt")
    .getLines
    .toList
    .filter(_ != "")
    .map(s => s.replace("10", "A").replace(",", "")) ++ additionalText

  val numberPairs: Int = inputText.length / 2

  val pairs: List[(String, String)] = (0 until numberPairs).map(
    n => (inputText(2 * n), inputText(2 * n + 1))
  ).toList

  @tailrec
  def compare(left: String, right: String): Boolean = {
    println(left + " <> " + right)

    (left.head, right.head) match {
      case (a, b) if a == b => compare(left.tail, right.tail)
      case (']', _) => true
      case (_, ']') => false
      case ('[', b) => compare(left.tail, s"$b]" + right.tail)
      case (a, '[') => compare(s"$a]" + left.tail, right.tail)
      case (a, b) => a < b
    }
  }

  val ans = inputText.sortWith(compare).zipWithIndex.flatMap { case (s, i) => if (additionalText.contains(s)) Some(i+1) else None }.product

  println(ans)
}