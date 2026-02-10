// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use anyhow::Result;
use fs_extra::dir::CopyOptions;
use move_binary_format::{CompiledModule, file_format::Visibility};
use move_compiler::editions::{Edition, Flavor};
use move_package_alt_compilation::{
    build_config::BuildConfig as MoveBuildConfig, lint_flag::LintFlag,
};
use std::{
    collections::BTreeMap,
    env, fs,
    path::{Path, PathBuf},
};
use myso_move_build::BuildConfig;
use myso_package_alt::mainnet_environment;

const CRATE_ROOT: &str = env!("CARGO_MANIFEST_DIR");
const COMPILED_PACKAGES_DIR: &str = "packages_compiled";
const DOCS_DIR: &str = "docs";
const PUBLISHED_API_FILE: &str = "published_api.txt";

#[tokio::test]
async fn build_system_packages() {
    let tempdir = tempfile::tempdir().unwrap();
    let out_dir = if std::env::var_os("UPDATE").is_some() {
        let crate_root = Path::new(CRATE_ROOT);
        let _ = std::fs::remove_dir_all(crate_root.join(COMPILED_PACKAGES_DIR));
        let _ = std::fs::remove_dir_all(crate_root.join(DOCS_DIR));
        let _ = std::fs::remove_file(crate_root.join(PUBLISHED_API_FILE));
        crate_root
    } else {
        tempdir.path()
    };

    std::fs::create_dir_all(out_dir.join(COMPILED_PACKAGES_DIR)).unwrap();
    std::fs::create_dir_all(out_dir.join(DOCS_DIR)).unwrap();

    let packages_path = Path::new(CRATE_ROOT).join("packages");
    let indir = tempfile::tempdir().unwrap();
    fs_extra::dir::copy(
        packages_path,
        indir.path(),
        &CopyOptions::new().content_only(true),
    )
    .unwrap();
    let packages_path = indir.path();

    // Fix Move.toml dependency paths for nested package structure
    // myso-framework/myso-framework/Move.toml references ../move-stdlib, but needs ../../move-stdlib
    let framework_move_toml = packages_path.join("myso-framework").join("myso-framework").join("Move.toml");
    if framework_move_toml.exists() {
        let content = fs::read_to_string(&framework_move_toml).unwrap();
        let fixed = content.replace("../move-stdlib", "../../move-stdlib");
        fs::write(&framework_move_toml, fixed).unwrap();
    }
    
    // myso-system/myso-system/Move.toml references ../move-stdlib and ../myso-framework
    // From myso-system/myso-system, need ../../move-stdlib and ../../myso-framework/myso-framework
    let system_move_toml = packages_path.join("myso-system").join("myso-system").join("Move.toml");
    if system_move_toml.exists() {
        let content = fs::read_to_string(&system_move_toml).unwrap();
        let fixed = content
            .replace("../move-stdlib", "../../move-stdlib")
            .replace("../myso-framework", "../../myso-framework/myso-framework");
        fs::write(&system_move_toml, fixed).unwrap();
    }
    
    // Fix bridge and deepbook Move.toml files that reference ../myso-framework and ../myso-system
    // They need to point to the nested directories
    for pkg_name in ["bridge", "deepbook", "mydata", "myso-social"] {
        let move_toml = packages_path.join(pkg_name).join("Move.toml");
        if move_toml.exists() {
            let content = fs::read_to_string(&move_toml).unwrap();
            let fixed = content
                .replace("../myso-framework", "../myso-framework/myso-framework")
                .replace("../myso-system", "../myso-system/myso-system");
            if fixed != content {
                fs::write(&move_toml, fixed).unwrap();
            }
        }
    }

    let bridge_path = packages_path.join("bridge");
    let deepbook_path = packages_path.join("deepbook");
    let myso_system_path = packages_path.join("myso-system").join("myso-system");
    let myso_framework_path = packages_path.join("myso-framework").join("myso-framework");
    let move_stdlib_path = packages_path.join("move-stdlib");
    let mydata_path = packages_path.join("mydata");
    let myso_social_path = packages_path.join("myso-social");

    build_packages(
        &bridge_path,
        &deepbook_path,
        &myso_system_path,
        &myso_framework_path,
        &move_stdlib_path,
        &mydata_path,
        &myso_social_path,
        out_dir,
    )
    .await;
    check_diff(Path::new(CRATE_ROOT), out_dir)
}

// Verify that checked-in values are the same as the generated ones
fn check_diff(checked_in: &Path, built: &Path) {
    for path in [COMPILED_PACKAGES_DIR, DOCS_DIR, PUBLISHED_API_FILE] {
        let output = std::process::Command::new("diff")
            .args(["--brief", "--recursive"])
            .arg(checked_in.join(path))
            .arg(built.join(path))
            .output()
            .unwrap();
        if !output.status.success() {
            let header = "Generated and checked-in myso-framework packages and/or docs do not match.\n\
                 Re-run with `UPDATE=1` to update checked-in packages and docs. e.g.\n\n\
                 UPDATE=1 cargo test -p myso-framework --test build-system-packages";

            panic!(
                "{header}\n\n{}\n\n{}",
                String::from_utf8_lossy(&output.stdout),
                String::from_utf8_lossy(&output.stderr)
            );
        }
    }
}

async fn build_packages(
    bridge_path: &Path,
    deepbook_path: &Path,
    myso_system_path: &Path,
    myso_framework_path: &Path,
    stdlib_path: &Path,
    mydata_path: &Path,
    myso_social_path: &Path,
    out_dir: &Path,
) {
    let config = MoveBuildConfig {
        generate_docs: true,
        warnings_are_errors: true,
        install_dir: Some(PathBuf::from(".")),
        lint_flag: LintFlag::LEVEL_NONE,
        default_edition: Some(Edition::E2024_BETA),
        default_flavor: Some(Flavor::MySo),
        ..Default::default()
    };
    debug_assert!(!config.test_mode);
    build_packages_with_move_config(
        bridge_path,
        deepbook_path,
        myso_system_path,
        myso_framework_path,
        stdlib_path,
        mydata_path,
        myso_social_path,
        out_dir,
        "bridge",
        "deepbook",
        "myso-system",
        "myso-framework",
        "move-stdlib",
        "mydata",
        "myso-social",
        config,
    )
    .await;
}

async fn build_packages_with_move_config(
    bridge_path: &Path,
    deepbook_path: &Path,
    myso_system_path: &Path,
    myso_framework_path: &Path,
    stdlib_path: &Path,
    mydata_path: &Path,
    myso_social_path: &Path,
    out_dir: &Path,
    bridge_dir: &str,
    deepbook_dir: &str,
    system_dir: &str,
    framework_dir: &str,
    stdlib_dir: &str,
    mydata_dir: &str,
    myso_social_dir: &str,
    config: MoveBuildConfig,
) {
    let stdlib_pkg = BuildConfig {
        config: config.clone(),
        run_bytecode_verifier: true,
        print_diags_to_stderr: false,
        environment: mainnet_environment(), // Framework pkg addr is agnostic to chain, resolves from Move.toml
    }
    .build_async(stdlib_path)
    .await
    .unwrap();
    let framework_pkg = BuildConfig {
        config: config.clone(),
        run_bytecode_verifier: true,
        print_diags_to_stderr: false,
        environment: mainnet_environment(), // Framework pkg addr is agnostic to chain, resolves from Move.toml
    }
    .build_async(myso_framework_path)
    .await
    .unwrap();
    let system_pkg = BuildConfig {
        config: config.clone(),
        run_bytecode_verifier: true,
        print_diags_to_stderr: false,
        environment: mainnet_environment(), // Framework pkg addr is agnostic to chain, resolves from Move.toml
    }
    .build_async(myso_system_path)
    .await
    .unwrap();
    let deepbook_pkg = BuildConfig {
        config: config.clone(),
        run_bytecode_verifier: true,
        print_diags_to_stderr: false,
        environment: mainnet_environment(), // Framework pkg addr is agnostic to chain, resolves from Move.toml
    }
    .build_async(deepbook_path)
    .await
    .unwrap();
    let bridge_pkg = BuildConfig {
        config: config.clone(),
        run_bytecode_verifier: true,
        print_diags_to_stderr: false,
        environment: mainnet_environment(), // Framework pkg addr is agnostic to chain, resolves from Move.toml
    }
    .build_async(bridge_path)
    .await
    .unwrap();
    let mydata_pkg = BuildConfig {
        config: config.clone(),
        run_bytecode_verifier: true,
        print_diags_to_stderr: false,
        environment: mainnet_environment(),
    }
    .build_async(mydata_path)
    .await
    .unwrap();
    let myso_social_pkg = BuildConfig {
        config,
        run_bytecode_verifier: true,
        print_diags_to_stderr: false,
        environment: mainnet_environment(),
    }
    .build_async(myso_social_path)
    .await
    .unwrap();

    let move_stdlib = stdlib_pkg.get_stdlib_modules();
    let myso_system = system_pkg.get_myso_system_modules();
    let myso_framework = framework_pkg.get_myso_framework_modules();
    let deepbook = deepbook_pkg.get_deepbook_modules();
    let bridge = bridge_pkg.get_bridge_modules();
    let mydata = mydata_pkg.get_mydata_modules();
    let myso_social = myso_social_pkg.get_myso_social_modules();

    let compiled_packages_dir = out_dir.join(COMPILED_PACKAGES_DIR);

    let myso_system_members =
        serialize_modules_to_file(myso_system, &compiled_packages_dir.join(system_dir)).unwrap();
    let myso_framework_members =
        serialize_modules_to_file(myso_framework, &compiled_packages_dir.join(framework_dir))
            .unwrap();
    let deepbook_members =
        serialize_modules_to_file(deepbook, &compiled_packages_dir.join(deepbook_dir)).unwrap();
    let bridge_members =
        serialize_modules_to_file(bridge, &compiled_packages_dir.join(bridge_dir)).unwrap();
    let stdlib_members =
        serialize_modules_to_file(move_stdlib, &compiled_packages_dir.join(stdlib_dir)).unwrap();
    let mydata_members =
        serialize_modules_to_file(mydata, &compiled_packages_dir.join(mydata_dir)).unwrap();
    let myso_social_members =
        serialize_modules_to_file(myso_social, &compiled_packages_dir.join(myso_social_dir)).unwrap();

    // write out generated docs
    let docs_dir = out_dir.join(DOCS_DIR);
    let mut files_to_write = BTreeMap::new();
    relocate_docs(
        &stdlib_pkg.package.compiled_docs.unwrap(),
        &mut files_to_write,
    );
    relocate_docs(
        &deepbook_pkg.package.compiled_docs.unwrap(),
        &mut files_to_write,
    );
    relocate_docs(
        &system_pkg.package.compiled_docs.unwrap(),
        &mut files_to_write,
    );
    relocate_docs(
        &framework_pkg.package.compiled_docs.unwrap(),
        &mut files_to_write,
    );
    relocate_docs(
        &bridge_pkg.package.compiled_docs.unwrap(),
        &mut files_to_write,
    );
    relocate_docs(
        &mydata_pkg.package.compiled_docs.unwrap(),
        &mut files_to_write,
    );
    relocate_docs(
        &myso_social_pkg.package.compiled_docs.unwrap(),
        &mut files_to_write,
    );
    for (fname, doc) in files_to_write {
        let dst_path = docs_dir.join(fname);
        fs::create_dir_all(dst_path.parent().unwrap()).unwrap();
        fs::write(dst_path, doc).unwrap();
    }

    let published_api = [
        myso_system_members.join("\n"),
        myso_framework_members.join("\n"),
        deepbook_members.join("\n"),
        bridge_members.join("\n"),
        stdlib_members.join("\n"),
        mydata_members.join("\n"),
        myso_social_members.join("\n"),
    ]
    .join("\n");

    fs::write(out_dir.join(PUBLISHED_API_FILE), published_api).unwrap();
}

/// Post process the generated docs so that they are in a format that can be consumed by
/// docusaurus.
/// * Flatten out the tree-like structure of the docs directory that we generate for a package into
///   a flat list of packages;
/// * Deduplicate packages (since multiple packages could share dependencies); and
/// * Write out the package docs in a flat directory structure.
fn relocate_docs(files: &[(String, String)], output: &mut BTreeMap<String, String>) {
    // Turn on multi-line mode so that `.` matches newlines, consume from the start of the file to
    // beginning of the heading, then capture the heading and replace with the yaml tag for docusaurus. E.g.,
    // ```
    // -<a name="0x2_display"></a>
    // -
    // -# Module `0x2::display`
    // -
    // +---
    // +title: Module `0x2::display`
    // +---
    //```
    let re = regex::Regex::new(r"(?s).*\n#\s+(.*?)\n").unwrap();
    for (file_name, file_content) in files {
        if file_name.contains("dependencies") {
            // we don't need to keep the dependency version of each doc since it will be generated
            // on its own
            continue;
        };
        output.entry(file_name.to_owned()).or_insert_with(|| {
            re.replace_all(
                &file_content
                    .replace("../../dependencies/", "../")
                    .replace("../dependencies/", "../")
                    .replace("dependencies/", "../"),
                "---\ntitle: $1\n---\n",
            )
            .to_string()
        });
    }
}

fn serialize_modules_to_file<'a>(
    modules: impl Iterator<Item = &'a CompiledModule>,
    file: &Path,
) -> Result<Vec<String>> {
    let mut serialized_modules = Vec::new();
    let mut members = vec![];
    for module in modules {
        let module_name = module.self_id().short_str_lossless();
        for def in module.struct_defs() {
            let sh = module.datatype_handle_at(def.struct_handle);
            let sn = module.identifier_at(sh.name);
            members.push(format!("{sn}\n\tpublic struct\n\t{module_name}"));
        }

        for def in module.enum_defs() {
            let eh = module.datatype_handle_at(def.enum_handle);
            let en = module.identifier_at(eh.name);
            members.push(format!("{en}\n\tpublic enum\n\t{module_name}"));
        }

        for def in module.function_defs() {
            let fh = module.function_handle_at(def.function);
            let fn_ = module.identifier_at(fh.name);
            let viz = match def.visibility {
                Visibility::Public => "public ",
                Visibility::Friend => "public(package) ",
                Visibility::Private => "",
            };
            let entry = if def.is_entry { "entry " } else { "" };
            members.push(format!("{fn_}\n\t{viz}{entry}fun\n\t{module_name}"));
        }

        let mut buf = Vec::new();
        module.serialize_with_version(module.version, &mut buf)?;
        serialized_modules.push(buf);
    }
    assert!(
        !serialized_modules.is_empty(),
        "Failed to find system or framework or stdlib modules"
    );

    let binary = bcs::to_bytes(&serialized_modules)?;

    fs::write(file, binary)?;

    Ok(members)
}
