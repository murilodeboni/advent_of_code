import scala.io.Source

val input = Source.fromFile("test_input.txt").getLines.toList

case class File(
  name: String,
  size: Int
)

case class Directory(
  name: String,
  parentFolder: Option[Directory] = None,
  childrenFolders: Set[Directory] = Set(),
  files: Set[File] = Set()
)

def ls(input: List[String], d: Directory): (Option[List[String]], Directory) = { 
  input.headOption match {
    case None => (Some(input), d)
    case Some(a) if a.startsWith("$") => (Some(input), d)
    case Some(a) if a.startsWith("dir ") => ls(input.tail, d.copy(childrenFolders = d.childrenFolders + Directory(a.replace("dir ",""),Some(d))))
    case _ => ls(input.tail, d.copy(files = d.files + File(input.head.split(" ")(1), input.head.split(" ")(0).toInt)))
  }
}

def cd(dirName: String, dir:Directory): Directory = {
  if (dirName == "..") {
    dir.parentFolder.get
  } else {
    dir.childrenFolders.filter(_.name == dirName).headOption.getOrElse(dir.copy(childrenFolders = dir.childrenFolders + Directory(dirName)))  
  }
} 

val root = Directory("/")

def parseInput(input: List[String], dir: Directory = root): Directory = {
  println(input.headOption)
  println(dir)
  input.headOption match {
    case None => dir
    case Some(s) => s match {
      case "$ ls" => {
        val res = ls(input.tail, dir)
        if (res._1.isDefined) parseInput(res._1.get, res._2) else dir
      }
      case c if c.startsWith("$ cd") => parseInput(input.tail, cd(c.replace("$ cd ",""), dir))
    }
  }
}

def goToRoot(d: Directory): Directory = {
  d.parentFolder.headOption match {
    case None => d
    case _ => goToRoot(d.parentFolder.get)
  }
}

println(
  goToRoot(parseInput(input.tail))
)
