import scala.io.Source

def readInput(path: String): Int = {
  var ar: Int = 0
  var tmp: Array[String] = Array()
  val fs = Source.fromFile(path)

  val markerCount = 14  

  while (fs.hasNext && tmp.toSet.size < markerCount) {
    if (ar < markerCount) {
      tmp = tmp.appended(fs.next.toString)
    } else {
      tmp = tmp.drop(1).appended(fs.next.toString)
    }
    ar += 1
  }
  ar
}

println(readInput("input.txt"))

