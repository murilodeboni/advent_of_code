import scala.io.Source

def getMonkeyNumber(s: String): String = {
  s.replace("Monkey ","").replace(":","")//.toInt
}

def getStartingItems(s: String): String = {
  s.replace("  Starting items: ","").replace(" ","")//.split(",").map(_.toInt).toList
}

def getOperation(s: String): String = {
  s.replace("  Operation: new = ","")//.toList
}

def getTest(s: String): String = {
  s.replace("  Test: ","").replace("divisible by ","%") + "==0"
}

def getTrueCondition(s: String): String = {
  s.replace("    If true: ","if (test) throwToMonkey(").replace("throw to monkey ","") + ")"
}

def getFalseCondition(s: String): String = {
  s.replace("    If false: "," else throwToMonkey(").replace("throw to monkey ","") + ")"
}

val input = Source.fromFile("input.txt").getLines.toList.map(x => {
  if (x.startsWith("M")) {
    getMonkeyNumber(x)
  } else if (x.startsWith("  S")) {
    getStartingItems(x)
  } else if (x.startsWith("  O")) {
    getOperation(x)
  } else if (x.startsWith("  T")) {
    getTest(x)
  } else if (x.startsWith("    If true")) {
    getTrueCondition(x)
  } else if (x.startsWith("    If false")) {
    getFalseCondition(x)
  } else {
    ""
  }
}).filter(_ != "").toList

case class Monkey(
  number: Int,
  items: List[Int],
  operation: String,
  test: String,
  condition: String,
  hist: List[Int] = List()
)

val monkeys = input.grouped(6).toList.map(l => {
  Monkey(
    number = l(0).toInt,
    items = l(1).split(",").map(_.toInt).toList,
    operation = l(2),
    test = l(3),
    condition = l(4) + l(5)
  )
})

def playMonkeyBusiness(round: Int = 20, monkeys: List[Monkey]): List[Monkey] = {
  var newMonkeys:List[Monkey] = List()
  for (m <- monkeys) {
    newMonkeys = newMonkeys :+ m.copy(number = m.number+ 1)
  }
  newMonkeys
}

playMonkeyBusiness(20, monkeys).map(println)
