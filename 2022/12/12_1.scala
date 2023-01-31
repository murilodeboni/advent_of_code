import scala.io.Source

val input: List[List[String]] = Source.fromFile("test_input.txt").getLines.toList.map(_.split("").toList)

val letterMap = ('a' to 'z').zipWithIndex.map{
  (s, n) => Map(s.toString -> (n+1))
}.reduce(_ ++ _) ++ Map("S" -> 0, "E" -> 99)

val inputMap = input.map(_.map(letterMap(_)))

@main def entryPoint() =
  inputMap.map(println)
