module Main where

import qualified Siclibhs.Lib (someFunc)

main :: IO ()
main = do
  putStrLn "Hello, Haskell!"
  Siclibhs.Lib.someFunc
