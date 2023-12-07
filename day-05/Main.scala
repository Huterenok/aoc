import scala.io.Source
import scala.collection.mutable.ArrayBuilder.ofLong

@main def Main = {
  val input = Source.fromFile("input.txt").mkString

  println(findLowestLocation(input))
  println(findLowestInRangeLocation(input))
}

def findLowestLocation(input: String): Long = {
  val SeedsAndMaps(seeds, maps) = retrieveSeedsAndMaps(input)

  seeds.map { seed =>
    maps.foldLeft(seed) { (currentStep, map) =>
      map
        .sliding(3, 3)
        .find { triplet =>
          (triplet(1) until triplet(1) + triplet(2))
            .contains(currentStep)
        }
        .map(triplet => triplet.head + currentStep - triplet(1))
        .getOrElse(currentStep)
    }
  }.min
}

def findLowestInRangeLocation(input: String): Long = {
  val SeedsAndMaps(seeds, maps) = retrieveSeedsAndMaps(input)

  (for {
    seed <- 0 until seeds.size by 2
    j <- seeds(seed) until seeds(seed) + seeds(seed + 1)
    step = maps.foldLeft(j)((currentStep, map) =>
      map
        .sliding(3, 3)
        .find(triplet =>
          (triplet(1) until triplet(1) + triplet(2)).contains(currentStep)
        )
        .map(triplet => triplet.head + currentStep - triplet(1))
        .getOrElse(currentStep)
    )
  } yield step).min
}

case class SeedsAndMaps(val seeds: Vector[Long], val maps: Vector[Vector[Long]])

def retrieveSeedsAndMaps(
    input: String
): SeedsAndMaps = {
  val mapsMatcher = """(?<=:\n)([\d\s]+)""".r
  val seedsMatcher = """([\d ]+)""".r

  val seeds =
    seedsMatcher
      .findAllIn(input)
      .next()
      .trim()
      .split(" ")
      .map(_.toLong)
      .toVector
  val maps =
    mapsMatcher
      .findAllIn(input)
      .filter(_ != "\n")
      .map(_.replace("\n", " ").split(" ").map(_.toLong).toVector)
      .toVector

  return SeedsAndMaps(seeds, maps);
}
