module Day01 where

import qualified Control.Monad.State as MS
import qualified Data.Maybe          as M
import qualified Data.Set            as S
import           Safe                      (headMay)

type Product     = Int
type Complements = S.Set Int
type Combination = (Int, Int)

run :: String -> IO ()
run input =
  do let numbers = read <$> lines input

     putStrLn $ "Part 1: " ++ show (part1 numbers)
     putStrLn $ "Part 2: " ++ show (part2 numbers)

part1 :: [Int] -> Product
part1 = M.fromJust . findPairSum 2020

-- | Find a pair of integers in `xs` that sum to `target`.
findPairSum :: Int -> [Int] -> Maybe Product
findPairSum target xs =
  do e <- headMay . MS.evalState (MS.filterM guard xs) $ S.empty
     pure $ e * (target - e)
  where
    guard :: Int -> MS.State Complements Bool
    guard x =
      do complements <- MS.get
         MS.put  $ (target - x) `S.insert` complements
         pure    $ x            `S.member` complements

-- TODO: Maybe one day in the future I will HOF this~
part2 :: [Int] -> Product
part2 []     = error "No triplet found :("
part2 (x:xs) = case findPairSum (2020 - x) xs of
                 Just yz -> x * yz
                 Nothing -> part2 xs
