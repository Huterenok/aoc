-- Haskell is too extraordinary for me so i have stolen
-- this solution from https://github.com/jimflood/aoc2023/blob/main/src/Lib.hs
-- because it is so beautiful that it was impossible not to steal
-- ghc -o main main.hs && ./main

import Control.Parallel.Strategies (parMap, rdeepseq)
import Data.List (nub)
import Data.Map qualified as Map

slurpLines :: String -> IO [String]
slurpLines filename = lines <$> readFile filename

type Coordinate = (Int, Int)

type Grid = ((Int, Int), Map.Map Coordinate Char)

data Direction = North | South | East | West deriving (Eq, Ord, Show)

type Beam = (Coordinate, Direction)

type Energy = Map.Map Beam Int

parseGrid :: [String] -> Grid
parseGrid css = ((length (head css), length css), Map.fromList [((x, y), c) | (y, cs) <- zip [0 ..] css, (x, c) <- zip [0 ..] cs])

energize :: Grid -> Beam -> Energy
energize (_, m) x = energize' (Map.empty, [x])
  where
    energize' :: (Energy, [Beam]) -> Energy
    energize' (e, []) = e
    energize' (e, b : bs)
      | fst b `Map.member` m = energize'' (Map.insertLookupWithKey (\k a1 a2 -> a1 + a2) b 1 e) (m Map.! fst b)
      | otherwise = energize' (e, bs) -- beam has left the grid
      where
        energize'' :: (Maybe Int, Energy) -> Char -> Energy
        energize'' (Nothing, ee) tile = energize' (ee, map step (f b tile) ++ bs)
          where
            f
              | tile == '.' = \b _ -> [b]
              | tile `elem` "\\/" = reflect
              | tile `elem` "|-" = split
              | otherwise = error "Kaboom!"
        energize'' (Just _, ee) _ = energize' (ee, bs) -- beam has merged with prior beam

step :: Beam -> Beam
step ((x, y), North) = ((x, y - 1), North)
step ((x, y), South) = ((x, y + 1), South)
step ((x, y), East) = ((x + 1, y), East)
step ((x, y), West) = ((x - 1, y), West)

reflect :: Beam -> Char -> [Beam]
reflect (p, North) '/' = [(p, East)]
reflect (p, South) '/' = [(p, West)]
reflect (p, East) '/' = [(p, North)]
reflect (p, West) '/' = [(p, South)]
reflect (p, North) '\\' = [(p, West)]
reflect (p, South) '\\' = [(p, East)]
reflect (p, East) '\\' = [(p, South)]
reflect (p, West) '\\' = [(p, North)]
reflect _ _ = error "???"

split :: Beam -> Char -> [Beam]
split (p, North) '|' = [(p, North)]
split (p, South) '|' = [(p, South)]
split (p, _) '|' = [(p, North), (p, South)]
split (p, East) '-' = [(p, East)]
split (p, West) '-' = [(p, West)]
split (p, _) '-' = [(p, East), (p, West)]
split _ _ = error "???"

candidates :: Grid -> [Beam]
candidates ((mx, my), _) = top ++ bottom ++ left ++ right
  where
    top = [((x, 0), South) | x <- [0 .. mx]]
    bottom = [((x, my), North) | x <- [0 .. mx]]
    left = [((0, y), East) | y <- [0 .. my]]
    right = [((mx, y), West) | y <- [0 .. my]]

measure :: Energy -> Int
measure e = length $ nub $ map fst $ Map.keys e

solve1 :: Grid -> Int
solve1 g = measure $ energize g ((0, 0), East)

solve2 :: Grid -> Int
solve2 g = maximum $ parMap rdeepseq (measure . energize g) (candidates g)

day16 :: IO ()
day16 = do
  grid <- parseGrid <$> slurpLines "day16.txt"
  let answer1 = solve1 grid
  print $ "part 1: " ++ show answer1
  let answer2 = solve2 grid
  print $ "part 2: " ++ show answer2