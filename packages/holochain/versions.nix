# TODO: automate updating these
# 0. bump ${attrName}.holochain.rev
# 1. set all sha256 and cargoSha256 to "0000000000000000000000000000000000000000000000000000"
# 2. build holochain: nix build -f default.nix packages.holochainAllBinariesWithDeps.${attrName}.holochain
# 3. replace ${attrName}.holochain.sha256 with output
# 4. build holochain: nix build -f default.nix packages.holochainAllBinariesWithDeps.${attrName}.holochain
# 5. replace ${attrName}.holochain.cargoSha256 with output
# 6. build lair-keystore: nix build -f default.nix packages.holochainAllBinariesWithDeps.${attrName}.lair-keystore
# 7. replace ${attrName}.lair-keystore.sha256 with output
# 8. build lair-keystore: nix build -f default.nix packages.holochainAllBinariesWithDeps.${attrName}.lair-keystore
# 10. replace ${attrName}.lair-keystore.cargoSha256 with output

{
  develop = {
    rev = "a317cbfbf1410548f8352ad70b2d615a659ce2b8";
    sha256 = "1rxqjfc78rjyi28icdiy3mc3722wy9wh5xdk94xij4lv9vxdxkds";
    cargoSha256 = "07bncnb45m82y37p2rdck5vd0hymd9a9vh3h6xmip1cf89fmsa3a";
    bins = {
      holochain = "holochain";
      hc = "hc";
      kitsune-p2p-proxy = "kitsune_p2p/proxy";
    };

    lairKeystoreHashes = {
      sha256 = "0khg5w5fgdp1sg22vqyzsb2ri7znbxiwl7vr2zx6bwn744wy2cyv";
      cargoSha256 = "1lm8vrxh7fw7gcir9lq85frfd0rdcca9p7883nikjfbn21ac4sn4";
    };
  };

  main = {
    rev = "holochain-0.0.108";
    sha256 = "1p9rqd2d2wlyzc214ia93b1f18fgqspmza863q4hrz9ba6xigzjs";
    cargoSha256 = "0p4m8ckbd7v411wgh14p0iz4dwi84i3cha5m1zgnqlln0wkqsb0f";
    bins = {
      holochain = "holochain";
      hc = "hc";
      kitsune-p2p-proxy = "kitsune_p2p/proxy";
    };

    lairKeystoreHashes = {
      sha256 = "0khg5w5fgdp1sg22vqyzsb2ri7znbxiwl7vr2zx6bwn744wy2cyv";
      cargoSha256 = "1lm8vrxh7fw7gcir9lq85frfd0rdcca9p7883nikjfbn21ac4sn4";
    };
  };
}
