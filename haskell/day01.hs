{-# LANGUAGE TupleSections #-}

import Data.Set (Set)
import qualified Data.Set as Set
import qualified System.IO as IO

type Product = Int

type Complements = Set Int
type Combinations = [(Int, Int)]

main :: IO ()
main = do
  input <- IO.readFile "_inputs/day01.txt"
  let numbers = [read word | word <- lines input]
      complements = Set.fromList . map (2020 -) $ numbers

  putStrLn $ "Part 1: " ++ show (part1 numbers)
  putStrLn $ "Part 2: " ++ show (part2 numbers)

part1 :: [Int] -> Product
part1 numbers = findPair numbers Set.empty

findPair :: [Int] -> Complements -> Product
findPair (x:xs) complements
  | x `Set.member` complements = x * (2020 - x)
  | otherwise = findPair xs $ Set.insert (2020 - x) complements

part2 :: [Int] -> Product
part2 numbers = findTriplet combinations Set.empty
  where combinations = generateCombinations numbers

findTriplet :: Combinations -> Complements -> Product
findTriplet ((x, y):xs) complements
  | (x + y) `Set.member` complements = x * y * (2020 - x - y)
  | otherwise = findTriplet xs $ Set.insert (2020 - x - y) complements

generateCombinations :: [Int] -> Combinations
generateCombinations (x : xs) = map (,x) xs ++ generateCombinations xs
