type Fallible<T> = anyhow::Result<T>;
use rnix::types::*;

pub(crate) struct VersionUpdater {
    ast: rnix::AST,
    // attrs: AttrSet,
}

impl VersionUpdater {
    pub(crate) fn from_reader<R: std::io::Read>(mut input: R) -> Fallible<Self> {
        let mut input_buf = vec![];
        input.read_to_end(&mut input_buf)?;
        let input_str = String::from_utf8(input_buf)?;

        println!("TRACE: parsed input from: \n\n```\n{}```", input_str,);

        let ast = rnix::parse(&input_str).as_result()?;

        // let attrs = ast
        //     .root()
        //     .inner()
        //     .and_then(AttrSet::cast)
        //     .ok_or("expected a set at root")?;

        Ok(Self { ast
          // , attrs
        })
    }

    pub(crate) fn get_string_at_path(&self, path: &str) -> Fallible<Option<String>> {
        let searched_path = path.split('.').collect::<Vec<_>>();

        let mut level = 0;
        let mut next_level = 0;
        let mut matched_nodes_per_level = vec![];

        let mut children = self.ast.node().children();

        while let Some(node) = children.next() {
            if AttrSet::cast(node.clone()).is_some() {
                level = next_level;
                next_level += 1;
                println!("[{}] found AttrSet", level);
            }

            let searched_path_at_level =
                if let Some(searched_path_at_level) = searched_path.get(level) {
                    searched_path_at_level
                } else {
                    println!("search path has no entry at level {}", level);
                    continue;
                };

            if let Some(key_value) = KeyValue::cast(node.clone()) {
                println!("[{}] found KeyValue", level);
                if let Some(key) = key_value.key() {
                    let path = key
                        .path()
                        .map(Ident::cast)
                        .flatten()
                        .map(|i| i.as_str().to_string())
                        .collect::<String>();

                    println!("[{}] {:?} == {:?} ?", level, path, searched_path_at_level);

                    if path == *searched_path_at_level {
                        println!("[{}] found path for this level", level);

                        match (level > 0, node.parent()) {
                            (false, _) => {
                                println!("[{}] storing value for key {:#?}", level, key);
                                matched_nodes_per_level.push(key_value.value().unwrap());
                            }
                            (true, Some(parent)) => {
                                if let Some(maybe_matching_parent) =
                                    matched_nodes_per_level.get(level - 1)
                                {
                                    assert_eq!(parent.text(), maybe_matching_parent.text());

                                    if let Some(value) = key_value.value() {
                                        return Ok(Some(
                                            value
                                                .text()
                                                .to_string()
                                                .trim_matches(|c| c == '\\' || c == '"')
                                                .to_string(),
                                        ));
                                    }

                                    return Ok(None);
                                }
                            }
                            (true, None) => {
                                panic!("[{}] {:#?} doesn't have a parent", level, node);
                            }
                        }
                    }
                }
            } else {
                println!("[{}] unhandled node {:#?}", level, node.kind());
            }
        }

        // if a level doesn't have a matching path then we can return None
        // only attrsets should increase levels
        // go level by level and look at all children to determine whether the key matches

        Ok(None)
    }

    #[cfg(feature = "broken")]
    pub(crate) fn get_string_at_path(&self, path: &str) -> Fallible<Option<String>> {
        let searched_path = path.split('.').collect::<Vec<_>>();

        {
            let mut level = 0;
            let mut next_level = 0;
            let mut matched_nodes_per_level = vec![];
            for event in self
                .ast
                .root()
                .inner()
                .ok_or_else(|| anyhow::anyhow!("no inner node"))?
                .preorder()
            {
                match event {
                    WalkEvent::Enter(node) => {
                        if AttrSet::cast(node.clone()).is_some() {
                            level = next_level;
                            next_level += 1;
                            println!("[{}] found AttrSet", level);
                        }

                        let searched_path_at_level =
                            if let Some(searched_path_at_level) = searched_path.get(level) {
                                searched_path_at_level
                            } else {
                                println!("search path has no entry at level {}", level);
                                continue;
                            };

                        if let Some(key_value) = KeyValue::cast(node.clone()) {
                            println!("[{}] found KeyValue", level);
                            if let Some(key) = key_value.key() {
                                let path = key
                                    .path()
                                    .map(Ident::cast)
                                    .flatten()
                                    .map(|i| i.as_str().to_string())
                                    .collect::<String>();

                                println!(
                                    "[{}] {:?} == {:?} ?",
                                    level, path, searched_path_at_level
                                );

                                if path == *searched_path_at_level {
                                    println!("[{}] found path for this level", level);

                                    match (level > 0, node.parent()) {
                                        (false, _) => {
                                            println!(
                                                "[{}] storing value for key {:#?}",
                                                level, key
                                            );
                                            matched_nodes_per_level
                                                .push(key_value.value().unwrap());
                                        }
                                        (true, Some(parent)) => {
                                            if let Some(maybe_matching_parent) =
                                                matched_nodes_per_level.get(level - 1)
                                            {
                                                assert_eq!(
                                                    parent.text(),
                                                    maybe_matching_parent.text()
                                                );

                                                if let Some(value) = key_value.value() {
                                                    return Ok(Some(
                                                        value
                                                            .text()
                                                            .to_string()
                                                            .trim_matches(|c| c == '\\' || c == '"')
                                                            .to_string(),
                                                    ));
                                                }

                                                return Ok(None);
                                            }
                                        }
                                        (true, None) => {
                                            panic!("[{}] {:#?} doesn't have a parent", level, node);
                                        }
                                    }
                                }
                            }
                        } else {
                            println!("[{}] unhandled node {:#?}", level, node.kind());
                        }
                    }
                    WalkEvent::Leave(node) => {
                        if AttrSet::cast(node.clone()).is_some() {
                            println!("[{}] decreasing level", level);
                            next_level -= 1;
                        }
                    }
                }
            }

            assert_eq!(next_level, 0);
        }

        Ok(None)
    }

    #[cfg(features = "broken")]
    fn set_string_at_path(&self, path: &str, value: &str) -> Fallible<Option<String>> {
        let searched_path = path.split('.').collect::<Vec<_>>();

        {
            let mut level = 0;
            let mut next_level = 0;
            let mut matched_nodes_per_level = vec![];
            for event in self
                .ast
                .root()
                .inner()
                .ok_or_else(|| anyhow::anyhow!("no inner node"))?
                .preorder()
            {
                match event {
                    WalkEvent::Enter(node) => {
                        if AttrSet::cast(node.clone()).is_some() {
                            level = next_level;
                            next_level += 1;
                            println!("[{}] found AttrSet", level);
                        }

                        let searched_path_at_level =
                            if let Some(searched_path_at_level) = searched_path.get(level) {
                                searched_path_at_level
                            } else {
                                println!("search path has no entry at level {}", level);
                                continue;
                            };

                        if let Some(key_value) = KeyValue::cast(node.clone()) {
                            println!("[{}] found KeyValue", level);
                            if let Some(key) = key_value.key() {
                                let path = key
                                    .path()
                                    .map(Ident::cast)
                                    .flatten()
                                    .map(|i| i.as_str().to_string())
                                    .collect::<String>();

                                println!(
                                    "[{}] {:?} == {:?} ?",
                                    level, path, searched_path_at_level
                                );

                                if path == *searched_path_at_level {
                                    println!("[{}] found path for this level", level);

                                    match (level > 0, node.parent()) {
                                        (false, _) => {
                                            println!(
                                                "[{}] storing value for key {:#?}",
                                                level, key
                                            );
                                            matched_nodes_per_level
                                                .push(key_value.value().unwrap());
                                        }
                                        (true, Some(parent)) => {
                                            if let Some(maybe_matching_parent) =
                                                matched_nodes_per_level.get(level - 1)
                                            {
                                                assert_eq!(
                                                    parent.text(),
                                                    maybe_matching_parent.text()
                                                );

                                                let previous_value =
                                                    key_value.value().map(|value| {
                                                        value
                                                            .text()
                                                            .to_string()
                                                            .trim_matches(|c| c == '\\' || c == '"')
                                                            .to_string()
                                                    });

                                                // todo: set value
                                                node.text_range();
                                            }
                                        }
                                        (true, None) => {
                                            panic!("[{}] {:#?} doesn't have a parent", level, node);
                                        }
                                    }
                                }
                            }
                        } else {
                            println!("[{}] unhandled node {:#?}", level, node.kind());
                        }
                    }
                    WalkEvent::Leave(node) => {
                        if AttrSet::cast(node.clone()).is_some() {
                            next_level -= 1;
                        }
                    }
                }
            }

            assert_eq!(next_level, 0);
        }

        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    const FIXTURE1: &str = r#"
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
                rev = "cacb6af9d733bcd782a04a9f4b0a72e520433a6e";
                sha256 = "18lc87z6pmbyzffgpi6b6jcikb44a0c4bmjzvvf7l4dgqmm2xbm6";
                cargoSha256 = "19z2qakhhvwrva16ycq4zpnhl0xhksli8jknfpr1l2sxfbm2zjiw";
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
                rev = "holochain-0.0.103";
                sha256 = "1z0y1bl1j2cfv4cgr4k7y0pxnkbiv5c0xv89y8dqnr32vli3bld7";
                cargoSha256 = "1rf8vg832qyymw0a4x247g0iikk6kswkllfrd5fqdr0qgf9prc31";
                bins = {
                    holochain = "holochain";
                    hc = "hc";
                    kitsune-p2p-proxy = "kitsune_p2p/proxy";
                };

                lairKeystoreHashes = {
                    sha256 = "1jiz9y1d4ybh33h1ly24s7knsqyqjagsn1gzqbj1ngl22y5v3aqh";
                    cargoSha256 = "0agykcl7ysikssfwkjgb3hfw6xl0slzy38prc4rnzvagm5wd1jjv";
                };
            };
        }
    "#;

    use super::*;

    #[test]
    fn get_string_at_path() {
        let version_updater = VersionUpdater::from_reader(FIXTURE1.as_bytes()).unwrap();

        let result = version_updater
            .get_string_at_path("develop.rev")
            .unwrap()
            .unwrap();

        assert_eq!(
            "cacb6af9d733bcd782a04a9f4b0a72e520433a6e",
            result.to_string()
        );
    }

    #[test]
    fn get_string_at_path_2() {
        let content = r#"
# This file was generated by nvfetcher, please do not modify it manually.
{ fetchgit, fetchurl }:
{
  holochain = {
    pname = "holochain";
    version = "3c1d86a9aa921e96f68d762557a016bd6bbe431b";
    src = fetchgit {
      url = "https://github.com/holochain/holochain";
      rev = "3c1d86a9aa921e96f68d762557a016bd6bbe431b";
      fetchSubmodules = false;
      deepClone = false;
      leaveDotGit = false;
      sha256 = "1xjhjggzvw0vysv83fl5pla16ami5yp1ac4y15233g7w1s2g4l3k";
    };
    cargoLock = {
      lockFile = ./holochain-3c1d86a9aa921e96f68d762557a016bd6bbe431b/Cargo.lock;
      cargoOutputHashes = {
        "cargo-test-macro-0.1.0" = "1yy1y1d523xdzwg1gc77pigbcwsbawmy4b7vw8v21m7q957sk0c4";
      };
    };
  };
}"#;

        let version_updater = VersionUpdater::from_reader(content.as_bytes()).unwrap();

        let result = version_updater
            .get_string_at_path("holochain.cargoLock.cargoOutputHashes")
            .unwrap()
            .unwrap();

        assert_eq!(
            r#"{
        "cargo-test-macro-0.1.0" = "1yy1y1d523xdzwg1gc77pigbcwsbawmy4b7vw8v21m7q957sk0c4";
      }"#,
            result.to_string()
        );
    }

    #[cfg(features = "broken")]
    #[test]
    fn set_string_at_path() {
        let version_updater = VersionUpdater::from_reader(FIXTURE1.as_bytes()).unwrap();

        let result = version_updater
            .set_string_at_path("develop.rev", "0000000000000000000000000000000000000000")
            .unwrap();

        assert_eq!(
            result,
            Some("cacb6af9d733bcd782a04a9f4b0a72e520433a6e".to_string())
        );

        // check that the value is set
        let result = version_updater
            .get_string_at_path("develop.rev")
            .unwrap()
            .unwrap();
        assert_eq!(
            "0000000000000000000000000000000000000000",
            result.to_string()
        );

        // todo: check that it writes to a file successfully
    }
}
