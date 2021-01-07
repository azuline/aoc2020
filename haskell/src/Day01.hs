module Day01 where

import Control.Monad.State
import qualified Data.Set as S
import qualified System.IO as IO

type Product     = Int
type Complements = S.Set Int
type Combination = (Int, Int)

main :: IO ()
main = do
  numbers <- map read . lines <$> IO.readFile "../inputs/day01.txt"

  putStrLn $ "Part 1: " ++ show (part1 numbers)
  putStrLn $ "Part 2: " ++ show (part2 numbers)

part1 :: [Int] -> Product
part1 xs = e * (2020 - e)
  where
    e = head $ evalState (filterM go xs) S.empty
    go :: Int -> State Complements Bool
    go x =
      do seen <- get
         put  $ x `S.insert` seen
         pure $ (2020 - x) `S.member` seen

part2 :: [Int] -> Product
part2 xs = e1 * e2 * (2020 - e1 - e2)
  where
    (e1, e2) = head $ evalState (filterM go $ pairs xs) S.empty
    go :: Combination -> State Complements Bool
    go (x, y) =
      do seen <- get
         put  $ (x + y) `S.insert` seen
         pure $ (2020 - x - y) `S.member` seen

pairs :: [Int] -> [Combination]
pairs xs = [ (x, y) | x <- xs, y <- xs ]
