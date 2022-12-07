import Data.Char

main :: IO()
main = do
    file <- readFile "03.input"
    print $ sum $ map prio $ lines file 
    print $ sum $ map findCommon $ group3 $ lines file

prio :: String -> Int 
prio s = convert $ check $ splitAt (div (length s) 2) s
    where
    check :: (String, String) -> Char
    check (x:xs, ys) 
        | elem x ys = x
        | otherwise = check (xs, ys)

convert :: Char -> Int
convert c 
    | isLower c = ord c - ord 'a' + 1
    | isUpper c = ord c - ord 'A' + 27 
    | otherwise = error "invalid input"


findCommon :: (String, String, String) -> Int 
findCommon (s:ss, r, v) 
    | elem s r && elem s v  = convert s 
    | otherwise             = findCommon (ss, r, v)

group3 :: [a] -> [(a, a, a)]
group3 [] = []
group3 (x:y:z:rest) = (x, y, z) : group3 rest
group3 _ = error "invalid input"
