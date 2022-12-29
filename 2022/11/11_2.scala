import scala.io.Source

def getMonkeyNumber(s: String): String = {
  s.replace("Monkey ","").replace(":","")//.toBigInt
}

def getStartingItems(s: String): String = {
  s.replace("  Starting items: ","").replace(" ","")//.split(",").map(_.toBigInt).toList
}

def getOperation(s: String): String = {
  s.replace("  Operation: new = ","")//.toList
}

def getTest(s: String): String = {
  s.replace("  Test: ","").replace("divisible by ","")
}

def getTrueCondition(s: String): String = {
  s.replace("    If true: ","").replace("throw to monkey ","")
}

def getFalseCondition(s: String): String = {
  s.replace("    If false: ","").replace("throw to monkey ","")
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
  items: List[BigInt],
  operation: String,
  test: Int,
  condition: List[Int],
  hist: Int = 0
)

val monkeys = input.grouped(6).toList.map(l => {
  Monkey(
    number = l(0).toInt,
    items = l(1).split(",").map(s => BigInt(s.toInt)).toList,
    operation = l(2),
    test = l(3).toInt,
    condition = List(l(4).toInt,l(5).toInt)
  )
})

def throwItemToMonkey(from: Int, to:Int, worry:BigInt, monkeys: List[Monkey]): List[Monkey] = {
  val fromMonkey = monkeys(from)
  val toMonkey = monkeys(to)

  val newToMonkey = toMonkey.copy(
    items = toMonkey.items :+ worry
  )

  val newFromMonkey = fromMonkey.copy(
    items = fromMonkey.items.drop(1),
    hist = fromMonkey.hist + 1
  )
  
  monkeys.patch(from,List(newFromMonkey),1).patch(to,List(newToMonkey),1)
}

val focus = List(2,3,5,7,11,13,17,19,23).reduce(_ * _)

val testFocus = List(23,19,13,17).reduce(_ * _)

def doOperation(worry: BigInt, operation: String, test: Int, condition: List[Int]): (Int,BigInt) = {
  val op = operation.split(" ")
  val op1 = op(0) match {
    case "old" => worry
    case _ => BigInt(op(0).toInt)
  }
  val op2 = op(2) match {
    case "old" => worry
    case _ => BigInt(op(2).toInt)
  }

  val result = op(1) match {
    case "*" => op1*op2
    case "+" => op1+op2
  }

  val newWorry = result%focus

  val testResult = newWorry%test==0
  val conditionResult = if (testResult) condition(0) else condition(1)

  (conditionResult, newWorry)
}

def lookAtItem(m: Monkey, monkeys: List[Monkey]): List[Monkey] = {
  val item = m.items.headOption

  item match {
    case None => monkeys
    case Some(i) => {
      // monkeys.map(println)
      val throwTo = doOperation(i, m.operation, m.test, m.condition) 
      val newMonkeys = throwItemToMonkey(m.number, throwTo._1, throwTo._2, monkeys)
      lookAtItem(m.copy(items = m.items.drop(1)),newMonkeys)
    }
  }
}

def playRound(monkeys: List[Monkey], n: Int = 0) : List[Monkey] = {
  if (n == monkeys.size) {
    monkeys
  } else {
    playRound(lookAtItem(monkeys(n.toInt), monkeys), n + 1)
  }
}

def playMonkeyBusiness(monkeys: List[Monkey], round: Int = 1): List[Monkey] = {
  if (round == 10001) {
    monkeys
  } else { 
    if(round%1000==0) {
      //println("=== ROUND ",round," ===")
      //println(monkeys.map(println))
    }
    playMonkeyBusiness(playRound(monkeys), round + 1)
  }
}

println(
  playMonkeyBusiness(monkeys).map(_.hist.toLong).sortWith(_ > _).take(2)
)

println(
  playMonkeyBusiness(monkeys).map(_.hist.toLong).sortWith(_ > _).take(2).reduce(_ * _)
)
