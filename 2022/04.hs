import Data.List.Split (splitOn)
main :: IO()
main = do
    file <- readFile "04.input"
    print $ length $ filter isSubrange $ map toRanges $ lines file
    print $ length $ filter overlap $ map toRanges $ lines file

isSubrange :: ((Int, Int),(Int, Int)) -> Bool 
isSubrange ((a,b),(c,d)) 
    | (a >= c) && (b <= d)  = True 
    | (c >= a) && (d <= b)  = True 
    | otherwise             = False


overlap :: ((Int, Int),(Int, Int)) -> Bool 
overlap ((a,b),(c,d)) 
    | (b >= c) && (a <= c)  = True 
    | (d >= a) && (c <= a)  = True 
    | otherwise = False

-- toRanges :: String -> ((Int, Int), (Int, Int))
toRanges s = toRange $ map (splitOn "-") $ splitOn "," s
    where toRange [[r,s], [u,v]] = ( ( read r :: Int, read s :: Int ), ( read u :: Int, read v :: Int ) )
