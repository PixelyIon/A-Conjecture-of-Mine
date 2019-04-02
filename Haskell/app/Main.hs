-- The following program is a simple test for the following conjecture:

-- Let S: N -> N be the sum of the digits of a positive integer.
-- For all A and B in N, S(A + B) = S(A) + S(B) - 9k, where k is an interger.

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

    case readDec maxStr :: [(Natural, String)] of
        [(max, "")] -> do
            putStrLn "\nLOADING. . ."
            start <- getTime Monotonic
            
            case counter' 0 max of
                Nothing -> do
                    end <- getTime Monotonic
                    putStrLn $ "LOADED. . . in " ++ formatMils (end - start) ++ "ms [1 Thread]"
                    putStrLn $ "\nThe conjecture is proved for all natural numbers smaller or equals to " ++ show max ++ "!"
                Just counter -> do
                    end <- getTime Monotonic
                    putStrLn $ "LOADED. . . in " ++ formatMils (end - start) ++ "ms [1 Thread]"
                    putStrLn $ "\nThe conjecture is disproved! Here's a counterexample: (" ++ (show $ fst $ counter) 
                        ++ ", " ++ (show $ snd $ counter) ++ ")"
                
        _ -> putStrLn $ "\n'" ++ maxStr ++ "' is not a natural number!"

sum' :: Natural -> Natural
sum' x = case x of
    0 -> 0
    x -> (x `mod` 10) + sum' (x `div` 10)

test' :: Natural -> Natural -> Bool
test' a b = ((fromEnum $ sum' (a + b)) - (fromEnum $ sum' a) - (fromEnum $ sum' b)) `mod` 9 == 0

counter' :: Natural -> Natural -> Maybe (Natural, Natural)
counter' min max = case [(a, b) | a <- [min..max], b <- [a..max], not $ test' a b] of
    [] -> Nothing
    fst : _ -> Just fst

formatMils :: TimeSpec -> [Char]
formatMils t = show (sec t * 10^3 + nsec t `div` 10^6)