import Data.List.Split

main :: IO()
main = do
    file <- readFile "05.input"
    let stacks = foldr1 putOn $ map (map (removeBraces . filter (/=' ')) . chunksOf 4) $ init $ head $ splitOn [""] $ lines file
    let commands = reverse $ map readCommand $ last $ splitOn [""] $ lines file
    print $ map head $ foldr applyCommand stacks commands

readCommand :: String -> (Int, Int, Int)
readCommand s = convert $ splitOn " " s
    where convert [_, x, _, y, _, z] = (read x :: Int, read y :: Int , read z :: Int)

putOn :: [[a]] -> [[a]] -> [[a]]
putOn [] [] = []
putOn (x:xs) (y:ys) = (x ++ y) : putOn xs ys 

removeBraces :: [a] -> [a] 
removeBraces [] = []
removeBraces [_,s,_] = [s]

applyCommand :: (Int, Int, Int) -> [[a]] -> [[a]]
applyCommand (x, y, z) stacks = apply (zip [1..] stacks)
    where 
    apply [] = []
    apply ((i,s):ss) 
        | i == y    = drop x s : apply ss
        | i == z    = (reverse (take x (stacks !! (y-1))) ++ s) : apply ss -- remove reverse for part 2
        | otherwise = s : apply ss
