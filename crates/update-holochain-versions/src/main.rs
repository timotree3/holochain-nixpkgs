#![allow(
    dead_code,
    unused_imports,
    unused_variables,
    unused_mut,
    non_snake_case
)]

use anyhow::bail;
use handlebars::JsonRender;
use once_cell::sync::Lazy;
use std::{
    collections::{BTreeMap, HashMap},
    io::Write,
    path::PathBuf,
    process::Command,
    str::FromStr,
};
use structopt::StructOpt;
use tempfile::{tempdir, TempPath};

mod nix_parser;

type Fallible<T> = anyhow::Result<T>;

/// This utility will write Nix code to `output_file`, that is tailored to be used as a specifier for which holochain repository to use, and which binaries to install from it.
#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(long, default_value = "versions.json")]
    output_file: PathBuf,

    /// Specifier for the holochain git repository
    #[structopt(long, default_value = "https://github.com/holochain/holochain")]
    git_repo: String,

    /// Git revisin specifier for fetching the holochain sources.
    /// Either: branch:<branch_name> or commit:<commit_id>
    #[structopt(long)]
    git_rev: GitRev,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct BinCrateSource<'a> {
    name: &'a str,
    git_repo: &'a str,
    git_rev: GitRev,
    bins: HashMap<&'a str, &'a str>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
enum GitRev {
    Branch(String),
    Commit(String),
}

impl FromStr for GitRev {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.to_string();
        let split = s
            .splitn::<_>(2, ':')
            .map(|s| s.to_lowercase())
            .map(|s| s.trim().to_owned())
            .map(|s| s.replace('"', ""))
            .collect::<Vec<_>>();
        match (split.get(0), split.get(1)) {
            (Some(key), Some(branch)) if key == "branch" => Ok(GitRev::Branch(branch.clone())),
            (Some(key), Some(commit)) if key == "commit" => Ok(GitRev::Commit(commit.clone())),
            (_, _) => bail!("invalid git-rev provided: {}", s),
        }
    }
}

// struct HolochainSource(BinCrateSource);
// impl Default for HolochainSource {
//     fn default() -> Self {
//         Self(BinCrateSource {
//             name: "holochain".to_string(),
//             repo: "https://github.com/holochain/holochain".to_string(),
//             rev: "main".to_string(),
//             bins: r#"{
//                 holochain = "holochain";
//                 hc = "hc";
//                 kitsune-p2p-proxy = "kitsune_p2p/proxy";
//             }"#
//             .to_string(),
//         })
//     }
// }

// struct LairSource(BinCrateSource);
// impl Default for LairSource {
//     fn default() -> Self {
//         Self(BinCrateSource {
//             name: "lair".to_string(),
//             repo: "https://github.com/holochain/lair".to_string(),
//             rev: "main".to_string(),
//             bins: r#"{
//                 "lair_keystore"
//             }"#
//             .to_string(),
//         })
//     }
// }

#[derive(serde::Serialize, serde::Deserialize)]
struct HolochainVersion<'a> {
    rev: &'a str,

    bins: HashMap<&'a str, &'a str>,

    hashes: CrateHashes<'a>,
    lair_hashes: CrateHashes<'a>,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct CrateHashes<'a> {
    sha256: &'a str,
    cargoLock: HashMap<String, String>,
}

static NVFETCHER_CARGO_TEMPLATE: &str = "nvfetcher_cargo_template";
static HOLOCHAIN_VERSION_TEMPLATE: &str = "holochain_version_template";
static HOLOCHAIN_VERSIONS_TEMPLATE: &str = "holochain_versions_template";

fn git_rev_helper(
    h: &handlebars::Helper,
    _: &handlebars::Handlebars,
    _: &handlebars::Context,
    rc: &mut handlebars::RenderContext,
    out: &mut dyn handlebars::Output,
) -> handlebars::HelperResult {
    let param = h.param(0).unwrap();

    let obj = param
        .value()
        .as_object()
        .unwrap()
        .iter()
        .next()
        .ok_or_else(|| handlebars::RenderError::new("invalid GitRev"))?;

    let git_rev = GitRev::from_str(&format!("{}:{}", obj.0, obj.1,)).unwrap();

    let (k, v) = match git_rev {
        GitRev::Branch(branch) => ("src.branch", branch),
        GitRev::Commit(commit) => ("src.manual", commit),
    };

    out.write(&format!(r#"{} = "{}""#, k, v))?;
    Ok(())
}

static HANDLEBARS: Lazy<handlebars::Handlebars> = Lazy::new(|| {
    let mut handlebars = handlebars::Handlebars::new();

    handlebars.register_helper("git-rev-helper", Box::new(git_rev_helper));
    handlebars
        .register_template_string(
            NVFETCHER_CARGO_TEMPLATE,
            r#"
[{{name}}]
src.git = "{{git_repo}}"
fetch.git = "{{git_repo}}"
{{git-rev-helper git_rev}}
cargo_lock = "Cargo.lock"
"#,
        )
        .unwrap();

    handlebars
        .register_template_string(
            HOLOCHAIN_VERSION_TEMPLATE,
            r#"
{
    {{#each holochain_version as |value key|}}
        {{@key}} = {
            rev = {{value.main}};
            sha256 = {{value.sha256}};
            cargoLock = {{value.cargoLock}};

            bins = {{value.bins}};

            lair = {
                sha256 = {{value.lair.sha256}};
                cargoLock = {{value.lair.cargoLock}};
            };
        }
    {{/each}}
}"#,
        )
        .unwrap();

    handlebars
        .register_template_string(
            HOLOCHAIN_VERSION_TEMPLATE,
            r#"
{
    {{#each holochain_version as |value key|}}
        {{@key}} = {{>holochain_version_template}};
    {{/each}}
}"#,
        )
        .unwrap();

    handlebars
});

fn main() -> Fallible<()> {
    let opt = Opt::from_args();

    // let holochain_git_rev = match opt.git_rev {
    //     Git => GitRev::Commit(commit),
    //     (Some(branch), _) if !branch.is_empty() => GitRev::Branch(branch),
    //     (branch, commit) => unreachable!(
    //         "neither git_branch or git_commit were provided. please report the occurrence of this message, as this case should have been covered by command line parsing. branch: {:?}, commit: {}",
    //         branch, commit
    //     ),
    // };

    let (holochain_sha256, holochain_cargoLock, lair_rev) = {
        let tempdir = tempdir()?;
        let nvfetcher_toml_path = tempdir.path().join("nvfetcher.toml");
        let mut nvfetcher_toml = std::fs::File::create(&nvfetcher_toml_path)?;

        let data = BinCrateSource {
            name: "holochain",
            git_repo: &opt.git_repo,
            git_rev: opt.git_rev,
            bins: maplit::hashmap! {
                "holochain" => "holochain",
                "hc" => "hc",
                "kitsune-p2p-proxy" => "kitsune_p2p/proxy",
            },
        };

        HANDLEBARS.render_to_write(NVFETCHER_CARGO_TEMPLATE, &data, nvfetcher_toml)?;

        println!(
            "wrote nvfetcher config at {}:\n{}",
            nvfetcher_toml_path.display(),
            std::fs::read_to_string(&nvfetcher_toml_path)?
        );

        let mut cmd = Command::new("nvfetcher");
        cmd.current_dir(tempdir.path())
            .args(&["build"])
            .stdout(std::process::Stdio::inherit())
            .stderr(std::process::Stdio::inherit());
        println!(
            "running the following cmd: {:#?} in {}",
            cmd,
            tempdir.path().display()
        );

        let output = cmd.output()?;
        // TODO: check nvfetcher for success

        let generated = std::fs::read_to_string(tempdir.path().join("_sources/generated.nix"))?;

        //         let re = regex::Regex::new(
        //             r#"
        // (.|\n)*
        // .*rev = "(?P<rev>.+)";
        // (.|\n)*
        // .*sha256 = "(?P<sha256>.+)";
        // "#,
        //         )
        //         .unwrap();
        // let captures = re.captures(&generated).unwrap();

        let vu = nix_parser::VersionUpdater::from_reader(generated.as_bytes())?;

        let sha256 = vu.get_string_at_path("holochain.src.fetchgit.sha256")?;
        println!("found sha256: {:?}", sha256);

        let cargoLock = vu.get_string_at_path("holochain.cargoLock")?.map(|s| {
            let s = s.trim_matches(|c| ['{', '}', ' ', '\n', '\r'].iter().any(|m| c == *m));
            s.split(';')
                .map(|p| p.trim())
                .filter_map(|p| {
                    p.split_once("=")
                        .map(|(k, v)| (k.trim().to_string(), v.trim().to_string()))
                })
                .collect::<HashMap<String, String>>()
        });
        println!("found cargoOutputHashes: {:?}", cargoLock);

        let _ = tempdir.into_path();

        // TODO: get lair rev

        (
            sha256.unwrap_or_default(),
            cargoLock.unwrap_or_default(),
            "TODO",
        )
    };

    // TODO: create a directory for nvfetcher
    // TODO: write nvfetcher.toml
    // TODO: run nvfetcher
    // TODO: get their sha256 and cargoLock

    let lair_sha256 = "TODO";
    let lair_cargoLock = maplit::hashmap! {};

    let mut rendered_holochain_source = vec![];

    let holochain_version = HolochainVersion {
        rev: "main",
        bins: maplit::hashmap! {},
        hashes: CrateHashes {
            sha256: &holochain_sha256,
            cargoLock: holochain_cargoLock,
        },
        lair_hashes: CrateHashes {
            sha256: lair_sha256,
            cargoLock: lair_cargoLock,
        },
    };

    HANDLEBARS
        .render_to_write(
            HOLOCHAIN_VERSION_TEMPLATE,
            &holochain_version,
            &mut rendered_holochain_source,
        )
        .unwrap();

    // let rendered_holochain_source = format!(
    //     VERSION_TEMPLATE!(),
    // );

    println!(
        "rendered source: {}",
        String::from_utf8_lossy(&rendered_holochain_source)
    );

    std::fs::write(opt.output_file, rendered_holochain_source)?;

    Ok(())
}
