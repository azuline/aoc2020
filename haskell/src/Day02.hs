module Day02 where

import qualified Data.Either                as E
import           Data.Void                        (Void)
import qualified Text.Megaparsec            as P
import qualified Text.Megaparsec.Char       as P
import qualified Text.Megaparsec.Char.Lexer as L

type Parser = P.Parsec Void String

data Policy = Policy Int Int Char String
              deriving (Show)

run :: String -> IO ()
run input =
  do let policies = E.rights $ P.parse parser "" <$> lines input

     putStrLn $ "Part 1: " ++ show (part1 policies)
     putStrLn $ "Part 2: " ++ show (part2 policies)

parser :: Parser Policy
parser = Policy
         <$> L.decimal
         <*> (P.char '-' *> L.decimal)
         <*> (P.space *> P.lowerChar)
         <*> (P.chunk ": " *> P.takeRest)

part1 :: [Policy] -> Int
part1 = length . filter guard
  where
    guard :: Policy -> Bool
    guard (Policy low high char password) =
      let n = count char password
       in low <= n && n <= high

count :: Char -> String -> Int
count c = length . filter (==c)

part2 :: [Policy] -> Int
part2 = length . filter guard
  where
    guard :: Policy -> Bool
    guard (Policy low high char pass) =
      (pass !! (low - 1) == char) /= (pass !! (high - 1) == char)
