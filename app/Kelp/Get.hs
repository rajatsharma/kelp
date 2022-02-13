{-# LANGUAGE OverloadedStrings #-}

module Kelp.Get where

import Data.String (IsString (fromString))
import Data.UUID.V4 (nextRandom)
import Soothsayer ((***))
import System.Directory (getHomeDirectory)
import System.Process (callProcess)

run :: String -> IO ()
run name = do
  home <- getHomeDirectory
  uuid <- nextRandom
  let destination = "{0}/.kelp/{1}" *** [home, show uuid]
  callProcess "mkdir" ["-p", destination]
  callProcess "curl" [name, "-o", "{0}/private.fish" *** [destination]]
