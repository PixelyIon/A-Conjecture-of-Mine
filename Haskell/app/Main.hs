module Main where

import Numeric
import Numeric.Natural
import System.Clock

main :: IO ()
main = do
    putStrLn "\nThis program is a simple test for the following conjecture:\n"
    putStrLn "Let S: N -> N be the sum of the digits of a positive integer."
    putStrLn "For all A and B in N, S(A + B) = S(A) + S(B) - 9k, where k is an interger.\n"
    putStrLn "What value would you like to test the conjecture for?"

    maxStr <- getLine
    let max = readDec maxStr

    if (not $ null max) && ((snd $ head max ) == "")
        then let counter = exceptions' 0 (fst $ head max) in do
            start <- getTime Monotonic
            putStrLn "\nLOADING. . ."

            if null counter
                then do 
                    end <- getTime Monotonic
                    putStrLn $ "LOADED in " ++ formatTime (end - start) ++ "s [1 Thread]"
                    putStrLn $ "\nThe conjecture is proved for all natural numbers smaller or equals to " ++ show (fst $ head max) ++ "!"
                    exit
                else do
                    putStrLn $ "\nThe conjecture is disproved! Here are the counter examples:\n" ++ (init $ tail $ show counter)
                    exit
        else do
            putStrLn $ "\n'" ++ maxStr ++ "' is not a natural number!"
            exit

exit :: IO ()
exit = do
    putStrLn "Press any key to continue. . ."
    getChar
    return ()

digs :: Natural -> [Natural]
digs 0 = [0]
digs x = digs (x `div` 10) ++ [x `mod` 10]

sum' :: Natural -> Natural
sum' x = sum $ digs x

test' :: Natural -> Natural -> Bool
test' a b = ((fromEnum $ sum' (a + b)) - (fromEnum $ sum' a) - (fromEnum $ sum' b)) `mod` 9 == 0

exceptions' :: Natural -> Natural -> [(Natural, Natural)]
exceptions' min max = [(a, b) | a <- [min..max], b <- [a..max], not $ test' a b]

formatTime :: TimeSpec -> [Char]
formatTime t = show (sec t) ++ "." ++ show (nsec t `div` 10^8)