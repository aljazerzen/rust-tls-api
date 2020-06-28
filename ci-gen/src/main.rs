use crate::ghwf::Step;
use crate::yaml::{Yaml, YamlWriter};
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;

mod ghwf;
mod yaml;

fn crates_list() -> Vec<String> {
    assert!(Path::new("./ci-gen").exists());
    let mut r = Vec::new();
    for p in fs::read_dir(".").unwrap() {
        let p = p.unwrap();
        if Path::new(&format!("{}/Cargo.toml", p.path().display())).exists() {
            r.push(p.path().file_name().unwrap().to_str().unwrap().to_owned());
        }
    }
    r.sort();
    assert!(r.len() > 3);
    r
}

fn steps(rt: &str) -> Vec<Step> {
    let mut r = vec![
        Step::name_uses("Checkout sources", "actions/checkout@v2"),
        Step::name_uses_with(
            "Install toolchain",
            "actions-rs/toolchain@v1",
            Yaml::map(vec![
                ("profile", "minimal"),
                ("toolchain", "${{ matrix.channel }}"),
                ("override", "true"),
            ]),
        ),
    ];
    for c in crates_list() {
        let mut args = format!("--manifest-path={}/Cargo.toml", c);
        if c != "ci-gen" {
            args.push_str(&format!(" --no-default-features --features={}", rt));
        }
        r.push(Step::name_uses_with(
            &format!("cargo test {}", c),
            "actions-rs/cargo@v1",
            Yaml::map(vec![("command", "test"), ("args", &args)]),
        ));
    }
    r
}

fn runtimes() -> Vec<&'static str> {
    vec!["runtime-tokio", "runtime-async-std"]
}

fn jobs() -> Yaml {
    let mut r = Vec::new();
    for rt in runtimes() {
        r.push((
            format!("{}", rt),
            Yaml::map(vec![
                (
                    "name",
                    Yaml::string(format!("{} ${{{{ matrix.channel }}}}", rt)),
                ),
                ("runs-on", Yaml::string("ubuntu-latest")),
                (
                    "strategy",
                    Yaml::map(vec![(
                        "matrix",
                        Yaml::map(vec![(
                            "channel",
                            Yaml::list(&["stable", "beta", "nightly"]),
                        )]),
                    )]),
                ),
                ("steps", Yaml::list(steps(rt))),
            ]),
        ))
    }
    Yaml::map(r)
}

fn main() {
    let yaml = Yaml::map(vec![
        ("on", Yaml::list(vec!["push", "pull_request"])),
        ("name", Yaml::string("CI")),
        ("jobs", jobs()),
    ]);

    let mut writer = YamlWriter::default();
    writer.write_line(&format!(
        "# @generated by {}, do not edit",
        env!("CARGO_PKG_NAME")
    ));
    writer.write_line("");
    writer.write_yaml(&yaml);
    File::create(".github/workflows/ci.yml")
        .unwrap()
        .write_all(writer.buffer.as_bytes())
        .unwrap();
}
