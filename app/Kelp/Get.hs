{-# LANGUAGE OverloadedStrings #-}

module Kelp.Get where

import Data.String (IsString (fromString))
import System.Directory (getHomeDirectory)
import System.Process (createProcess, proc)

run :: String -> IO ()
run name = do
  home <- getHomeDirectory
  let process = proc "curl" [name, "-o", home ++ "/.kelp/private.fish"]
  _ <- createProcess process
  pure ()
