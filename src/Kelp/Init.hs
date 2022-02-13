{-# LANGUAGE OverloadedStrings #-}

module Kelp.Init where

import Data.String (IsString (fromString))
import System.Directory (getHomeDirectory)
import System.Process (createProcess, proc)

run :: IO ()
run = do
  home <- getHomeDirectory
  let process = proc "mkdir" ["-p", home ++ "/.kelp"]
  _ <- createProcess process
  let shellScript =
        unlines
          ["for file in ~/.kelp/*.fish", "\t[ -r \"$file\" ] && [ -f \"$file\" ] && source \"$file\";", "end"]
  putStr shellScript
