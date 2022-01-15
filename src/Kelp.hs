{-# LANGUAGE OverloadedStrings #-}

module Kelp (shell) where

import Data.Semigroup ((<>))
import qualified Kelp.Get as Get
import qualified Kelp.Init as Init
import Options.Applicative

data Kelp = Init | Get String

runGet :: Parser Kelp
runGet = Get <$> argument str idm

parser :: Parser Kelp
parser =
  subparser $
    command "init" (info (pure Init) (progDesc "Initialise Kelp"))
      <> command "get" (info runGet (progDesc "Get fish files from gist"))

runner :: Kelp -> IO ()
runner Init = Init.run
runner (Get name) = Get.run name

shell :: IO ()
shell = runner =<< execParser opts
  where
    opts =
      info (parser <**> helper) $
        fullDesc
          <> progDesc "Enter Command to run, see available commands for command descriptions."
          <> header "Kelp - Source fish scripts from Github gists"
