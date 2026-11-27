//! The public-repo files are not built by Cargo.
//! This check fails when a clone is missing the license, the contributor guide, or CI.

use std::path::PathBuf;

fn repo_file(name: &str) -> String {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(name);
    std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("read {}: {err}", path.display()))
}

#[test]
fn public_repo_has_contributor_surface() {
    let license_mit = repo_file("LICENSE-MIT");
    assert!(license_mit.contains("MIT License"));
    assert!(license_mit.contains("Copyright (c) 2026 Tai Alex"));

    let license_apache = repo_file("LICENSE-APACHE");
    assert!(license_apache.contains("Apache License"));
    assert!(license_apache.contains("Version 2.0"));

    let contributing = repo_file("CONTRIBUTING.md");
    assert!(contributing.contains("cargo test --workspace"));
    assert!(contributing.contains("editors/vscode"));
    assert!(contributing.contains("TRYBUILD"));

    let ci = repo_file(".github/workflows/ci.yml");
    assert!(ci.contains("cargo fmt --all -- --check"));
    assert!(ci.contains("cargo clippy --workspace --all-targets -- -D warnings"));
    assert!(ci.contains("cargo test --workspace"));
    assert!(ci.contains("1.97.1"));

    let readme = repo_file("README.md");
    assert!(readme.contains("CONTRIBUTING.md"));
    assert!(!readme.contains("C:/Users/ASUS"));
    assert!(readme.contains("https://github.com/taiocean1213/Alex-XP"));

    let manifest = repo_file("crates/alex-xp/Cargo.toml");
    assert!(manifest.contains("license = \"MIT OR Apache-2.0\""));
    assert!(manifest.contains("repository = \"https://github.com/taiocean1213/Alex-XP\""));

    assert!(repo_file("CODE_OF_CONDUCT.md").contains("Contributor Covenant"));
    assert!(repo_file("SECURITY.md").contains("taiocean1213@gmail.com"));
    assert!(repo_file("CHANGELOG.md").contains("## Unreleased"));
    assert!(repo_file("rust-toolchain.toml").contains("1.97.1"));
    assert!(repo_file(".github/PULL_REQUEST_TEMPLATE.md").contains("cargo test --workspace"));
    assert!(repo_file(".github/ISSUE_TEMPLATE/bug_report.yml").contains("rustc --version"));
    assert!(repo_file(".github/ISSUE_TEMPLATE/feature_request.yml")
        .contains("The call you want to write"));
}
