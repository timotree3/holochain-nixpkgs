# This file was generated by nvfetcher, please do not modify it manually.
{ fetchgit, fetchurl }:
{
  holochain_develop = {
    pname = "holochain_develop";
    version = "05c939784ac8aed6a8e2db416f8bed0b73980c32";
    src = fetchgit {
      url = "https://github.com/holochain/holochain";
      rev = "05c939784ac8aed6a8e2db416f8bed0b73980c32";
      fetchSubmodules = false;
      deepClone = false;
      leaveDotGit = false;
      sha256 = "11g1b6219ikcirk0zva1c1laiav4xhl7cwy10jq6nxgpb77gb7zn";
    };
    cargoLock = {
      lockFile = ./holochain_develop-05c939784ac8aed6a8e2db416f8bed0b73980c32/Cargo.lock;
      outputHashes = {
        "cargo-test-macro-0.1.0" = "1yy1y1d523xdzwg1gc77pigbcwsbawmy4b7vw8v21m7q957sk0c4";
      };
    };
  };
  holochain_main = {
    pname = "holochain_main";
    version = "05c939784ac8aed6a8e2db416f8bed0b73980c32";
    src = fetchgit {
      url = "https://github.com/holochain/holochain";
      rev = "05c939784ac8aed6a8e2db416f8bed0b73980c32";
      fetchSubmodules = false;
      deepClone = false;
      leaveDotGit = false;
      sha256 = "11g1b6219ikcirk0zva1c1laiav4xhl7cwy10jq6nxgpb77gb7zn";
    };
    cargoLock = {
      lockFile = ./holochain_main-05c939784ac8aed6a8e2db416f8bed0b73980c32/Cargo.lock;
      outputHashes = {
        "cargo-test-macro-0.1.0" = "1yy1y1d523xdzwg1gc77pigbcwsbawmy4b7vw8v21m7q957sk0c4";
      };
    };
  };
  nixpkgs = {
    pname = "nixpkgs";
    version = "9dc4a6a3c1d68c5ad333dc7b16580229ec99e8da";
    src = fetchgit {
      url = "https://github.com/nixos/nixpkgs";
      rev = "9dc4a6a3c1d68c5ad333dc7b16580229ec99e8da";
      fetchSubmodules = false;
      deepClone = false;
      leaveDotGit = false;
      sha256 = "122p64ccs4spimx1fa18vvbpn7vb6y84d8bbqizbpxv67fg1qa6j";
    };
  };
  nixpkgs-unstable = {
    pname = "nixpkgs-unstable";
    version = "9dc4a6a3c1d68c5ad333dc7b16580229ec99e8da";
    src = fetchgit {
      url = "https://github.com/nixos/nixpkgs";
      rev = "9dc4a6a3c1d68c5ad333dc7b16580229ec99e8da";
      fetchSubmodules = false;
      deepClone = false;
      leaveDotGit = false;
      sha256 = "122p64ccs4spimx1fa18vvbpn7vb6y84d8bbqizbpxv67fg1qa6j";
    };
  };
}
