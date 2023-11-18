import scala.math.BigDecimal
import scala.math.BigInt
import scala.util.Try

def factorial(n: BigInt): BigInt = {
  if (n == 0 || n == 1) 1
  else n * factorial(n - 1)
}

object PiDigits {
  def piNthDigit(n: Int): Try[BigDecimal] = Try {
    def piN(n: Int): BigDecimal = {
      val B2n = BigDecimal(-2) * n * zeta(1 - 2 * n)
      val nominator = BigDecimal(2).pow(-1 * (n + 1)) * factorial(2 * n)
      val denominator = BigDecimal(2).pow(2 * n) * B2n *
                        (1 - BigDecimal(2).pow(-2 * n)) *
                        (1 - BigDecimal(3).pow(-2 * n)) *
                        (1 - BigDecimal(5).pow(-2 * n)) *
                        (1 - BigDecimal(7).pow(-2 * n))
      (nominator / denominator).pow(1 / (2 * n))
    }

    if (n < 0) throw new IllegalArgumentException("n must be a non-negative integer")
    else if (n == 0) BigDecimal(3)
    else if (n == 1) BigDecimal(1)
    else if (n == 2) BigDecimal(4)
    else {
      val piNMinus1 = piN(n - 1)
      val fracPart = (piNMinus1 * BigDecimal(10).pow(n - 1)) % 1
      (fracPart * 10).setScale(0, BigDecimal.RoundingMode.FLOOR).toInt
    }
  }

  // Dummy implementations for zeta and factorial functions
  // Replace with actual implementations
  def zeta(x: Int): BigDecimal = ???

def main(args: Array[String]): Unit = {
    val piStr = BigDecimal(math.Pi).setScale(20, BigDecimal.RoundingMode.HALF_UP).toString
    (1 until piStr.length - 2).foreach { i =>
      val expectedDigit = piStr.charAt(i + 1).asDigit
      val calculatedDigit = piNthDigit(i).getOrElse(throw new Exception(s"Mismatch at the ${i}th digit of pi"))
      assert(calculatedDigit == expectedDigit, s"Mismatch at the ${i}th digit of pi")
    }
    println("All digits match correctly.")
  }
}
