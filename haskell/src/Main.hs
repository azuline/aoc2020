import qualified System.Environment as E
import qualified Text.Read          as R
import qualified Safe               as S

import qualified Day01
import qualified Day02

main :: IO ()
main =
  do args <- E.getArgs
     runDay $ S.headMay args >>= R.readMaybe

runDay :: Maybe Int -> IO ()
runDay day =
  do case day of
       Just 1 -> Day01.run =<< readFile "../inputs/day01.txt"
       Just 2 -> Day02.run =<< readFile "../inputs/day02.txt"
       _      -> putStrLn      "Invalid day!"
