package main

import java.io.File

// kotlinc main.kt -include-runtime -d main.jar && java -jar main.jar

fun findPoints(input: List<String>): Int {
  var res = 0
  val nums = retrieveNumMathes(input)

  for (num in nums) {
    res += (if (num > 0) 1 else 0) shl (num - 1)
  }

  return res
}

fun findTotalScratchcards(input: List<String>): Int {
  var res = 0
  val nums = retrieveNumMathes(input)

  var i = nums.size - 1
  while (i >= 0) {
    var temp = 1
    for (j in 1..nums[i]) {
      temp += nums.get(j + i).or(0)
    }

    res += temp
    nums[i] = temp
    i -= 1
  }

  return res
}

fun retrieveNumMathes(input: List<String>): MutableList<Int> {
  val res = mutableListOf<Int>()
  val matcher = "\\d+".toRegex()

  for (line in input) {
    val (winningPart, actualPart) = line.split("|").map { it.trim() }

    val winningNums = matcher.findAll(winningPart).drop(1).map { it.value.toInt() }.toHashSet()
    val actualNums = matcher.findAll(actualPart).map { it.value.toInt() }.toList()

    val matches = actualNums.filter { winningNums.contains(it) }.size
    res.add(matches)
  }

  return res
}

fun main() {
  val example_input = File("example_input.txt").readLines()
  val input = File("input.txt").readLines()

  val exampleRes1 = findPoints(example_input)
  if (!exampleRes1.equals(13)) {
    println("Retarted!!!")
  }
  val exampleRes2 = findTotalScratchcards(example_input)
  if (!exampleRes2.equals(30)) {
    println("Retarted!!!")
  }

  val res1 = findPoints(input)
  if (!res1.equals(33950)) {
    println("Retarted!!!")
  }
  val res2 = findTotalScratchcards(input)
  if (!res2.equals(14814534)) {
    println("Retarted!!!")
  }
}
