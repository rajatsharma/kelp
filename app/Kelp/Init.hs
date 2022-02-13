{-# LANGUAGE OverloadedStrings #-}

module Kelp.Init where

import Data.String (IsString (fromString))
import System.Directory (getHomeDirectory)
import System.Process (callProcess)

run :: IO ()
run = do
  home <- getHomeDirectory
  callProcess "mkdir" ["-p", home ++ "/.kelp"]
  let shellScript =
        unlines
          ["for file in ~/.kelp/**/*.fish", "\t[ -r \"$file\" ] && [ -f \"$file\" ] && source \"$file\";", "end"]
  putStr shellScript
