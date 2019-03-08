module Main where

import Numeric
import Numeric.Natural
import System.Clock
import GHC.Conc

main :: IO ()
main = do
    -- Configure the number of threads used by the program
    setNumCapabilities 1

    putStrLn "\nThis program is a simple test for the following conjecture:\n"
    putStrLn "Let S: N -> N be the sum of the digits of a positive integer."
    putStrLn "For all A and B in N, S(A + B) = S(A) + S(B) - 9k, where k is an interger.\n"
    putStrLn "What value would you like to test the conjecture for?"
    maxStr <- getLine

    case readDec maxStr :: [(Natural, String)] of
        [(max, "")] -> do
            start <- getTime Monotonic
            putStrLn "\nLOADING. . ."

            case exceptions' 0 max of
                [] -> do 
                    end <- getTime Monotonic
                    putStrLn $ "LOADED. . . in " ++ formatTime (end - start) ++ "s [1 Thread]"
                    putStrLn $ "\nThe conjecture is proved for all natural numbers smaller or equals to " ++ show max ++ "!"
                counter -> putStrLn $ "\nThe conjecture is disproved! Here are the counter examples:\n" ++ (init $ tail $ show counter)
                
        _ -> putStrLn $ "\n'" ++ maxStr ++ "' is not a natural number!"

digs :: Natural -> [Natural]
digs x = case x of
    0 -> [0]
    x -> digs (x `div` 10) ++ [x `mod` 10]

sum' :: Natural -> Natural
sum' x = sum $ digs x

test' :: Natural -> Natural -> Bool
test' a b = ((fromEnum $ sum' (a + b)) - (fromEnum $ sum' a) - (fromEnum $ sum' b)) `mod` 9 == 0

exceptions' :: Natural -> Natural -> [(Natural, Natural)]
exceptions' min max = [(a, b) | a <- [min..max], b <- [a..max], not $ test' a b]

formatTime :: TimeSpec -> [Char]
formatTime t = show (sec t) ++ "." ++ show (nsec t `div` 10^8)