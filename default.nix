with (import <nixpkgs> { });

haskell.lib.buildStackProject {
  ghc = pkgs.ghc;
  name = "kelp";
  src = ./.;
}
