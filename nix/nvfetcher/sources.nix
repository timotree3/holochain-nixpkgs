let
  _nixpkgs = (import ./_sources/generated.nix (with builtins; {
      inherit fetchurl;
      fetchgit = { url, rev, fetchSubmodules, deepClone, leaveDotGit, sha256 }: fetchGit { inherit url rev; };
    })).nixpkgs.src;
  nixpkgs = import _nixpkgs {};
in nixpkgs.callPackage ./_sources/generated.nix {}
