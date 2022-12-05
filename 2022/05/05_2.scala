import scala.io.Source

case class Instruction (
  amount: Int,
  from: Int,
  to: Int
)

case class Cargo(
  positions: Array[Array[String]],
  instructions: Array[Instruction]
)

def parsePosition(line: String): Array[String] = {
  val s = line.size
  var ar: Array[String] = Array()
  var i = 1
  while (i<=s-1) {
    ar = ar.appended(line(i).toString)
    i = i + 4
  }
  ar
}

def parseInstruction(line: String): Instruction = {
  val lineValues = line.replace("move ","").replace(" from ",",").replace(" to ",",").split(",")
  Instruction(
    lineValues(0).toInt,
    lineValues(1).toInt -1,
    lineValues(2).toInt -1
  )
}

def moveFromTo(i: Instruction, pos: Array[Array[String]]): Array[Array[String]] = {
  var newPos = pos
  newPos(i.to) = newPos(i.from).take(i.amount) ++ newPos(i.to)
  newPos(i.from) = newPos(i.from).drop(i.amount)
  newPos
}

def followInstructions(c: Cargo): Array[Array[String]] = {
  if (c.instructions.isEmpty) {
    c.positions
  } else {
    followInstructions(Cargo(moveFromTo(c.instructions.head, c.positions), c.instructions.tail))
  }
}

def readInput(path: String): Cargo = {
  var ar: Array[Array[String]] = Array()
  var instructions: Array[Instruction] = Array()
  for (line <- Source.fromFile(path).getLines) {
    if (line.contains("[")) {
      ar = ar.appended(parsePosition(line))
    } else if (line.contains("move")) {
      instructions = instructions.appended(parseInstruction(line))
    } else {
      // do nothing
    }
  }
  Cargo(ar.transpose.map(_.filter(_ != " ")),instructions)
}
println(
  followInstructions(
    readInput("input.txt") 
  ).map(_.head).reduce(_ + _)
)
