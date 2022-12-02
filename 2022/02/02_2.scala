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
  "X" -> 0,
  "Y" -> 3,
  "Z" -> 6 
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
  val matchPoint = translateElf(myPlay)
  val freePoint = if (matchPoint == 6) {
    freePointMap(winMap(opponentTranslate))
  } else if (matchPoint == 3) {
    freePointMap(opponentTranslate)
  } else {
    freePointMap(winMap.map(_.swap)(opponentTranslate))
  }

  matchPoint + freePoint
}

val input = readInput("input.txt")

val result = input.map{
  case(o,e) => play(o,e)
}.reduce(_ + _)

println(result)
