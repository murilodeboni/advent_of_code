import scala.io.Source

def readInput(path: String): List[(String, String)] = {
  var ar:List[(String, String)] = List()
  var tmp = Array("")
  for (line <- Source.fromFile(path).getLines) {
    tmp = line.split(" ")
    ar = ar :+ ((tmp(0),tmp(1)))
  }
  ar
}

val translateOpponent = Map(
  "A" -> "Rock",
  "B" -> "Paper",
  "C" -> "Scisor"
)

val translateElf = Map(
  "Y" -> "Paper",
  "X" -> "Rock",
  "Z" -> "Scisor"
)

val winMap = Map(
  "Rock" -> "Paper",
  "Paper" -> "Scisor",
  "Scisor" -> "Rock" 
)

val freePointMap = Map(
  "Rock" -> 1,
  "Paper" -> 2,
  "Scisor" -> 3
)

def play(opponentPlay: String, myPlay: String): Int = {
  val opponentTranslate = translateOpponent(opponentPlay)
  val elfTranslate = translateElf(myPlay)

  val matchPoint = if (opponentTranslate == elfTranslate) {
    3
  } else if (winMap(opponentTranslate) == elfTranslate) {
    6
  } else {
    0
  }

  val freePoint = freePointMap(elfTranslate)

  matchPoint + freePoint
}

val input = readInput("test_input.txt")

val result = input.map{
  case(o,e) => play(o,e)
}.reduce(_ + _)

println(result)
