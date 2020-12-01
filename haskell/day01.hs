{-# LANGUAGE TupleSections #-}

import Data.Set (Set)
import qualified Data.Set as Set
import qualified System.IO as IO

type Product = Int

type Complements = Set Int

main :: IO ()
main = do
  input <- IO.readFile "_inputs/day01.txt"
  let numbers = [read word | word <- lines input]
      complements = Set.fromList . map (2020 -) $ numbers

  putStrLn $ "Part 1: " ++ show (part1 numbers complements)
  putStrLn $ "Part 2: " ++ show (part2 numbers complements)

part1 :: [Int] -> Complements -> Product
part1 numbers complements =
  head [x * (2020 - x) | x <- numbers, x `Set.member` complements]

part2 :: [Int] -> Complements -> Product
part2 numbers complements =
  head
    [ x * y * (2020 - x - y)
      | (x, y) <- combinations,
        (x + y) `Set.member` complements
    ]
  where
    combinations = generateCombinations numbers

generateCombinations :: [Int] -> [(Int, Int)]
generateCombinations (x : xs) = map (,x) xs ++ generateCombinations xs
