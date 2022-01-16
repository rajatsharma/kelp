with (import <nixpkgs> { });

haskell.lib.buildStackProject {
  name = "kelp";
  src = ./.;
}
