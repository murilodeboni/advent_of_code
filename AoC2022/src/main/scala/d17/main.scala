import scala.io.Source

object PartOne17 extends App {
  def loadInput(input: String): List[String] = {
    Source.fromFile(name = input)
      .getLines
      .toList
  }

  val listInput: Array[String] = loadInput("./src/main/scala/d17/test_input.txt").toArray
  val listInputLen = listInput.length

  var t = Tunnel(count = 0).init()

  while (t.rockCount < 2023) {
    t = t.insert().move(listInput(t.count % listInputLen)).move("down")
//    t.print()
    println(t.rockCount)
  }

  println(t.height())
}
case class Rock(t: Int){
  def format: Array[String] = t match {
    case 0 => Array("@@@@")
    case 1 => Array(
      " @ ",
      "@@@",
      " @ "
    )
    case 2 => Array(
      "  @",
      "  @",
      "@@@"
    )
    case 3 => Array(
      "@",
      "@",
      "@",
      "@"
    )
    case 4 => Array(
      "@@",
      "@@"
    )
  }

  def width: Int = t match {
    case 0 => 4
    case 1 => 3
    case 2 => 3
    case 3 => 1
    case 4 => 2
  }

  def height: Int = t match {
    case 0 => 1
    case 1 => 3
    case 2 => 3
    case 3 => 4
    case 4 => 2
  }
}

case class Tunnel(t: Array[String] = Array(), count: Int) {

  val rockCount = count

  val tunnel: Array[String] = t
  def init(): Tunnel = Tunnel(
      Array("+-------+"),
      count = rockCount
  )

  def height(): Int = t.length

  def move(direction: String): Tunnel = {
    val width = 9
    val height = tunnel.length

    // Convert the matrix into a mutable array of arrays for easier manipulation
    val board: Array[Array[Char]] = tunnel.map(_.toCharArray)

    def detectGroundedPieces(board: Array[Array[Char]]): Set[(Int, Int)] = {
      // Set to track grounded coordinates
      var groundedPieces = Set[(Int, Int)]()

      // Helper function to perform flood-fill from a given position
      def floodFill(y: Int, x: Int): Unit = {
        if (
          y >= 0 && y < height && x >= 0 && x < width &&
            !groundedPieces.contains((y, x)) &&
            (board(y)(x) == '#' || board(y)(x) == '@' || board(y)(x) == '-')
        ) {
          groundedPieces += ((y, x))
          floodFill(y - 1, x) // Up
          floodFill(y, x + 1) // Right
          floodFill(y, x - 1) // Left
        }
      }

      // Start flood-fill from the ground level ('-' or '#')
      for {
        x <- 1 to 8
      } {
        val b = board(height-1)(x)
        floodFill(height-1, x)
      }
      groundedPieces
    }

    // Helper function to find the coordinates of the current shape
    def findShape() = {
      for {
        y <- board.indices
        x <- board(y).indices
        if board(y)(x) == '@'
      } yield (y, x)
    }

    val shapeCoords = findShape()

    // Function to check if moving is possible in the specified direction
    def canMove(dx: Int, dy: Int): Boolean = {
      shapeCoords.forall { case (y, x) =>
        val newY = y + dy
        val newX = x + dx

        newY >= 0 && newY < height &&
          newX >= 0 && newX < width &&
          (board(newY)(newX) == ' ' || shapeCoords.contains((newY, newX)))
      }
    }

    // Calculate the movement offsets based on the direction
    val (dx, dy) = direction match {
      case "left" => (-1, 0)
      case "right" => (1, 0)
      case "down" => (0, 1)
      case _ => (0, 0) // No movement for invalid direction
    }

    val canMoveAns: Boolean = canMove(dx, dy)

    // Move the shape if possible
    if (canMoveAns) {
      // Clear the current shape
      shapeCoords.foreach { case (y, x) =>
        board(y)(x) = ' '
      }
      // Place the shape at the new position
      shapeCoords.foreach { case (y, x) =>
        board(y + dy)(x + dx) = '@'
      }
    } else if (!canMoveAns && direction == "down") {

      val coordsToFreeze = detectGroundedPieces(board)

      // Freeze only the pieces touching ground into '#'
      coordsToFreeze.foreach { case (y, x) =>
        if (y > 0) {
          board(y)(x) = '#'
        }
      }
    }

    // Convert the board back to the Tunnel
    Tunnel(board.map(_.mkString), rockCount)
  }


  def emptyTunnelSection(s: Int): Tunnel = {
    if (s > 1) {
      Tunnel(tunnel.prepended("|       |"), rockCount).emptyTunnelSection(s-1)
    } else {
      Tunnel(tunnel.prepended("|       |"), rockCount)
    }

  }

  def insert(t: Int = rockCount % 5): Tunnel = {
    val rock = Rock(t)
    var new_tunnel = Tunnel(tunnel, rockCount).emptyTunnelSection(3).tunnel
    for (i <- 0 to rock.height-1) yield {
      new_tunnel = new_tunnel.prepended("|  " + rock.format(i) + " "*(5-rock.width) + "|")
    }
    Tunnel(new_tunnel, rockCount+1)
  }

  def print(): Unit = {
    tunnel.map(println)
  }

}