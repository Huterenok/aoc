import Data.List (tails)
import Text.Printf (printf)

type Point = (Int, Int)

type Galaxy = [Point]

calculateDistance :: Point -> Point -> [Bool] -> [Bool] -> Int -> Int
calculateDistance (y0, x0) (y1, x1) emptyRows emptyCols expansionSpeed =
  let expansion =
        length . filter id $
          take (abs $ y1 - y0) (drop (min y0 y1) emptyRows)
            ++ take (abs $ x1 - x0) (drop (min x0 x1) emptyCols)
   in abs (y1 - y0) + abs (x1 - x0) + expansion * expansionSpeed

findGalaxyDistances :: String -> Int -> Int
findGalaxyDistances input expansionSpeed =
  sum [calculateDistance g1 g2 emptyRows emptyCols expansionSpeed | (g1, g2) <- tupleCombinations galaxies]
  where
    (galaxies, emptyRows, emptyCols) = retrieveGrid input

retrieveGrid :: String -> (Galaxy, [Bool], [Bool])
retrieveGrid input =
  let galaxies = [(y, x) | (y, line) <- zip [0 ..] (lines input), (x, c) <- zip [0 ..] line, c == '#']
      (maxRow, maxCol) = foldl (\(y0, x0) (y, x) -> (max y0 y, max x0 x)) (0, 0) galaxies
      emptyRows = [not $ any (\(y, _) -> y == i) galaxies | i <- [0 .. maxRow]]
      emptyCols = [not $ any (\(_, x) -> x == i) galaxies | i <- [0 .. maxCol]]
   in (galaxies, emptyRows, emptyCols)

tupleCombinations :: [a] -> [(a, a)]
tupleCombinations xs = [(x, y) | (x : ys) <- tails xs, y <- ys]

main :: IO ()
main = do
  input <- readFile "../input.txt"
  exampleInput <- readFile "../example_input.txt"

  let res1Example = findGalaxyDistances exampleInput 1
  let res1 = findGalaxyDistances input 1

  printf "Result 1: example - %d, real - %d\n" res1Example res1

  let res2Example = findGalaxyDistances exampleInput 999999
  let res2 = findGalaxyDistances input 999999

  printf "Result 2: example - %d, real - %d\n" res2Example res2
