main :: IO()
main = do
    file <- readFile "02.input"
    print $ sum $ map roundPoints $ lines file
{--
roundPoints :: String -> Int 
roundPoints "A X" = 1 + 3
roundPoints "A Y" = 2 + 6
roundPoints "A Z" = 3 + 0
roundPoints "B X" = 1 + 0
roundPoints "B Y" = 2 + 3
roundPoints "B Z" = 3 + 6
roundPoints "C X" = 1 + 6
roundPoints "C Y" = 2 + 0
roundPoints "C Z" = 3 + 3
--}
roundPoints :: String -> Int 
roundPoints "A X" = 3 + 0
roundPoints "A Y" = 1 + 3
roundPoints "A Z" = 2 + 6
roundPoints "B X" = 1 + 0
roundPoints "B Y" = 2 + 3
roundPoints "B Z" = 3 + 6
roundPoints "C X" = 2 + 0
roundPoints "C Y" = 3 + 3
roundPoints "C Z" = 1 + 6
