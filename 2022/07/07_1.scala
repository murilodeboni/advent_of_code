import scala.io.Source

val input = Source.fromFile("input.txt").getLines.toList

val inputSplit: List[List[String]]= input.tail.map(_.split(" ").toList)

var paths: List[List[String]] = List()

var currentDir: List[String] = List("/")

inputSplit.map(a => a  match {
  case List("$", "cd", "..") => currentDir = currentDir.dropRight(1)
  case List("$", "cd", a) =>  currentDir = currentDir :+ (a + "/")
  case List("$", "ls") => println("reading dir", currentDir.mkString(""))
  case List("dir", d) => paths = paths :+ (currentDir ++ List(d + "/", "", "0"))
  case List(s, f) => paths = paths :+ (currentDir ++ List(f,s))
  case _ => println("erro")
})

val dirList = paths.map(p => p.dropRight(2).mkString("")).toSet.toList

val fileList = paths.map(p => (p.dropRight(1).mkString(""),p.last.toInt))

def getDirSize(dirList: List[String], fileList: List[(String, Int)], ans: Int = 0): Int = {
  dirList.headOption match {
    case None => ans
    case Some(dir) => {
      val totalSize = fileList.filter(_._1.startsWith(dir)).map(_._2).sum
      if (totalSize <= 100000) {
        getDirSize(dirList.tail, fileList, ans + totalSize)
      } else {
        getDirSize(dirList.tail, fileList, ans)
      }
    }
  }
}

println(getDirSize(dirList, fileList))
