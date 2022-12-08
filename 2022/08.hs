import Data.List.Split

main :: IO ()
main = do 
    file <- readFile "08.input"
    let forest = zip [0..] $ map (zip [0..] . map (read :: String -> Int) . tail . splitOn "") $ lines file :: [(Int, [(Int, Int)])]
    let trees = [ (x, y, z) | (x, ys) <- forest, (y, z) <- ys ] :: [(Int, Int, Int)]
    let visible = [ (x, y, z) | (x, y, z) <- trees, checkVisible (y, z) (getRow x trees) || checkVisible (x, z)  (getColumn y trees) ]
    print $ length visible
    print $ maximum $ map (`scenicScore` trees) visible

getColumn :: Int -> [(Int, Int, a)] -> [(Int, a)]
getColumn i xs = [(x, z) | (x, y, z) <- xs, y == i]

getRow :: Int -> [(Int, Int, a)] -> [(Int, a)]
getRow i xs = [(y, z) | (x, y, z) <- xs, x == i]

checkVisible :: Ord a => (Int, a) -> [(Int, a)] -> Bool 
checkVisible (i, h) xs
    | null [ (x, y) | (x, y) <- xs, x < i, y >= h ] = True
    | null [ (x, y) | (x, y) <- xs, x > i, y >= h ] = True
    | otherwise                                     = False

scenicScore :: Ord a => (Int,Int,a) -> [(Int, Int, a)] -> Int
scenicScore (r, c, h) xs = let 
    column  = splitAt r $ getColumn c xs 
    row     = splitAt c $ getRow r xs
    left = reverse $ fst row 
    right = tail $ snd row 
    up = reverse $ fst column 
    down = tail $ snd column
    in
    calc left * calc right * calc up * calc down 
        where 
        calc [] = 0 
        calc ((x,y):xs) 
            | y < h    = 1 + calc xs 
            | otherwise = 1
