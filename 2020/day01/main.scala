import scala.io.Source
import scala.collection.mutable.ArrayBuffer

object Main {
  def part1(): Unit = {
    val numbers = new ArrayBuffer[Int]()
    for(line <- Source.fromFile("input").getLines) {
      numbers += line.toInt
    }
    for (i <- 0 until numbers.size;
         j <- i + 1 until numbers.size;
         if numbers(i) + numbers(j) == 2020) {
        println(numbers(i), numbers(j), numbers(i) * numbers(j))
    }
  }

  def part2(): Unit = {
    val numbers = new ArrayBuffer[Int]()
    for(line <- Source.fromFile("input").getLines) {
      numbers += line.toInt
    }
    for (i <- 0 until numbers.size;
         j <- 0 until numbers.size;
         k <- 0 until numbers.size;
         if numbers(i) + numbers(j) + numbers(k) == 2020) {
        println(numbers(i), numbers(j), numbers(k), numbers(i) * numbers(j) * numbers(k))
    }
  }

  def main(args: Array[String]): Unit = {
    part1()
    part2()
  }
}
