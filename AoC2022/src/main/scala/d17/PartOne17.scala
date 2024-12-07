import scala.io.Source

object PartOne17 extends App {
  def loadInput(input: String): List[String] = {
    Source.fromFile(name = input)
      .getLines
      .toList
  }

  val listInput = loadInput("./src/main/scala/d17/test_input.txt")

  println("hello")
}