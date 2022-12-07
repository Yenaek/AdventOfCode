-- find the first sequence of n characters with different characters and return the index of the last one
firstMarker :: Eq a => Int -> [a] -> Int
firstMarker n input = go 0 n input
    where
    different :: Eq a => [a] -> [a] -> Bool
    different [] _  = True
    different (x:xs) ys
        | elem x ys = False 
        | otherwise = different xs (x:ys)
    go :: Eq a => Int -> Int -> [a] -> Int
    go i n input
        | different (take n input) []  = i + n
        | otherwise         = go (i+1) n (tail input)
