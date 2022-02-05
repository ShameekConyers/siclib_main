module Main where

import qualified SicLibHs.Lib (someFunc)

main :: IO ()
main = do
  putStrLn "Hello, Haskell!"
  SicLibHs.Lib.someFunc
