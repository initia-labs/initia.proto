//! Build CosmosSDK/Tendermint/IBC proto files. This build script clones the CosmosSDK version
//! specified in the COSMOS_SDK_REV constant and then uses that to build the required
//! proto files for further compilation. This is based on the proto-compiler code
//! in github.com/informalsystems/ibc-rs

use regex::Regex;
use std::{
    env,
    ffi::{OsStr, OsString},
    fs::{self, create_dir_all, remove_dir_all},
    io,
    path::{Path, PathBuf},
    process,
    sync::atomic::{self, AtomicBool},
};
use walkdir::WalkDir;

/// Suppress log messages
// TODO(tarcieri): use a logger for this
static QUIET: AtomicBool = AtomicBool::new(false);

const INITIA_REV: &str = "main";

// All paths must end with a / and either be absolute or include a ./ to reference the current
// working directory.

/// The directory generated cosmos-sdk proto files go into in this repo
const OUT_DIR: &str = "./src/proto";
/// Directory where the cosmos-sdk submodule is located
const INITIA_DIR: &str = "../initia";
/// A temporary directory for proto building
const TMP_BUILD_DIR: &str = "/tmp/tmp-protobuf/";

// Patch strings used by `copy_and_patch`

/// Protos belonging to these Protobuf packages will be excluded
/// (i.e. because they are sourced from `tendermint-proto`)
const EXCLUDED_PROTO_PACKAGES: &[&str] = &["gogoproto", "google", "tendermint"];

/// Log info to the console (if `QUIET` is disabled)
// TODO(tarcieri): use a logger for this
macro_rules! info {
    ($msg:expr) => {
        if !is_quiet() {
            println!("[info] {}", $msg)
        }
    };
    ($fmt:expr, $($arg:tt)+) => {
        info!(&format!($fmt, $($arg)+))
    };
}

fn main() {
    if is_github() {
        set_quiet();
    }

    let tmp_build_dir: PathBuf = TMP_BUILD_DIR.parse().unwrap();
    let output_dir: PathBuf = OUT_DIR.parse().unwrap();

    if tmp_build_dir.exists() {
        fs::remove_dir_all(tmp_build_dir.clone()).unwrap();
    }

    let temp_initia_dir = tmp_build_dir.join("initia");
    fs::create_dir_all(&temp_initia_dir).unwrap();

    set_initia_version(&temp_initia_dir);
    compile_initia_protos_and_services(&temp_initia_dir);
    compile_thirdparty_protos_and_services(&temp_initia_dir);

    copy_generated_files(&temp_initia_dir, &output_dir);

    apply_patches(&output_dir);

    info!("Running rustfmt on prost/tonic-generated code");
    run_rustfmt(&output_dir);

    if is_github() {
        println!("Rebuild protos with proto-build (initia rev: {})", INITIA_REV);
    }
}

fn is_quiet() -> bool {
    QUIET.load(atomic::Ordering::Relaxed)
}

fn set_quiet() {
    QUIET.store(true, atomic::Ordering::Relaxed);
}

/// Parse `--github` flag passed to `proto-build` on the eponymous GitHub Actions job.
/// Disables `info`-level log messages, instead outputting only a commit message.
fn is_github() -> bool {
    env::args().any(|arg| arg == "--github")
}

fn run_cmd(cmd: impl AsRef<OsStr>, args: impl IntoIterator<Item = impl AsRef<OsStr>>) {
    let stdout = if is_quiet() {
        process::Stdio::null()
    } else {
        process::Stdio::inherit()
    };

    let exit_status = process::Command::new(&cmd)
        .args(args)
        .stdout(stdout)
        .status()
        .expect("exit status missing");

    if !exit_status.success() {
        panic!(
            "{:?} exited with error code: {:?}",
            cmd.as_ref(),
            exit_status.code()
        );
    }
}

fn run_rustfmt(dir: &Path) {
    let mut args = ["--edition", "2021"]
        .iter()
        .map(Into::into)
        .collect::<Vec<OsString>>();

    args.extend(
        WalkDir::new(dir)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file() && e.path().extension() == Some(OsStr::new("rs")))
            .map(|e| e.into_path())
            .map(Into::into),
    );

    run_cmd("rustfmt", args);
}

fn set_initia_version(out_dir: &Path) {
    let path = out_dir.join("INITIA_COMMIT");
    fs::write(path, INITIA_REV).unwrap();
}

fn compile_initia_protos_and_services(out_dir: &Path) {
    info!(
        "Compiling cosmos-sdk .proto files to Rust into '{}'...",
        out_dir.display()
    );

    //let root = env!("CARGO_MANIFEST_DIR");
    let sdk_dir = Path::new(INITIA_DIR);

    let proto_includes_paths = [
        //format!("{}/../proto", root),
        format!("{}/proto", sdk_dir.display()),
        format!("{}/third_party/proto", sdk_dir.display()),
    ];

    // Paths
    let proto_paths = [
        format!("{}/proto", sdk_dir.display()),
        /* 
        format!("{}/proto/initia/distribution", sdk_dir.display()),
        format!("{}/proto/initia/mint", sdk_dir.display()),
        format!("{}/proto/initia/move", sdk_dir.display()),
        format!("{}/proto/initia/mstaking", sdk_dir.display()),
        */
    ];

    // List available proto files
    let mut protos: Vec<PathBuf> = vec![];
    collect_protos(&proto_paths, &mut protos);

    // List available paths for dependencies
    let includes: Vec<PathBuf> = proto_includes_paths.iter().map(PathBuf::from).collect();

    // Compile all of the proto files, along with grpc service clients
    info!("Compiling proto definitions and clients for GRPC services!");
    tonic_build::configure()
        .build_client(true)
        .build_server(true)
        .out_dir(out_dir)
        .extern_path(".tendermint", "::tendermint_proto")
        .compile(&protos, &includes)
        .unwrap();

    info!("=> Done!");
}

fn compile_thirdparty_protos_and_services(out_dir: &Path) {
    //let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let sdk_dir = PathBuf::from(INITIA_DIR);

    let proto_includes_paths = [
        //root.join("../proto"),
        sdk_dir.join("proto"),
        sdk_dir.join("third_party/proto"),
    ];

    let thirdparty_dir = sdk_dir.join("third_party");

    // List available paths for dependencies
    let includes: Vec<PathBuf> = proto_includes_paths.iter().map(PathBuf::from).collect();

        // Paths
    let proto_paths = [
        format!("{}/proto", thirdparty_dir.display()),
        /* 
        format!("{}/proto/confio/auth", thirdparty_dir.display()),
        format!("{}/proto/cosmos/auth", thirdparty_dir.display()),
        format!("{}/proto/cosmos/authz", thirdparty_dir.display()),
        format!("{}/proto/cosmos/bank", thirdparty_dir.display()),
        format!("{}/proto/cosmos/base", thirdparty_dir.display()),
        format!("{}/proto/cosmos/base/tendermint", thirdparty_dir.display()),
        format!("{}/proto/cosmos/capability", thirdparty_dir.display()),
        format!("{}/proto/cosmos/crisis", thirdparty_dir.display()),
        format!("{}/proto/cosmos/crypto", thirdparty_dir.display()),
        format!("{}/proto/cosmos/distribution", thirdparty_dir.display()),
        format!("{}/proto/cosmos/evidence", thirdparty_dir.display()),
        format!("{}/proto/cosmos/feegrant", thirdparty_dir.display()),
        format!("{}/proto/cosmos/genutil", thirdparty_dir.display()),
        format!("{}/proto/cosmos/gov", thirdparty_dir.display()),
        format!("{}/proto/cosmos/mint", thirdparty_dir.display()),
        format!("{}/proto/cosmos/params", thirdparty_dir.display()),
        format!("{}/proto/cosmos/slashing", thirdparty_dir.display()),
        format!("{}/proto/cosmos/staking", thirdparty_dir.display()),
        format!("{}/proto/cosmos/tx", thirdparty_dir.display()),
        format!("{}/proto/cosmos/upgrade", thirdparty_dir.display()),
        format!("{}/proto/cosmos/vesting", thirdparty_dir.display()),
        format!("{}/proto/cosmos_proto", thirdparty_dir.display()),
        format!("{}/proto/gogoproto", thirdparty_dir.display()),
        format!("{}/proto/google", thirdparty_dir.display()),
        format!("{}/proto/ibc", thirdparty_dir.display()),
        format!("{}/proto/tendermint", thirdparty_dir.display()),
        */

    ];

    // List available proto files
    let mut protos: Vec<PathBuf> = vec![];
    collect_protos(&proto_paths, &mut protos);

    // Compile all proto client for GRPC services
    info!("Compiling thirdparty proto clients for GRPC services!");
    tonic_build::configure()
        .build_client(true)
        .build_server(false)
        .out_dir(out_dir)
        .compile(&protos, &includes)
        .unwrap();

    info!("=> Done!");
}

/// collect_protos walks every path in `proto_paths` and recursively locates all .proto
/// files in each path's subdirectories, adding the full path of each file to `protos`
///
/// Any errors encountered will cause failure for the path provided to WalkDir::new()
fn collect_protos(proto_paths: &[String], protos: &mut Vec<PathBuf>) {
    for proto_path in proto_paths {
        protos.append(
            &mut WalkDir::new(proto_path)
                .into_iter()
                .filter_map(|e| e.ok())
                .filter(|e| {
                    e.file_type().is_file()
                        && e.path().extension().is_some()
                        && e.path().extension().unwrap() == "proto"
                })
                .map(|e| e.into_path())
                .collect(),
        );
    }
}

fn copy_generated_files(from_dir: &Path, to_dir: &Path) {
    info!("Copying generated files into '{}'...", to_dir.display());

    // Remove old compiled files
    remove_dir_all(&to_dir).unwrap_or_default();
    create_dir_all(&to_dir).unwrap();

    let mut filenames = Vec::new();

    // Copy new compiled files (prost does not use folder structures)
    let errors = WalkDir::new(from_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .map(|e| {
            let filename = e.file_name().to_os_string().to_str().unwrap().to_string();
            filenames.push(filename.clone());
            copy_and_patch(e.path(), format!("{}/{}", to_dir.display(), &filename))
        })
        .filter_map(|e| e.err())
        .collect::<Vec<_>>();

    if !errors.is_empty() {
        for e in errors {
            eprintln!("[error] Error while copying compiled file: {}", e);
        }

        panic!("[error] Aborted.");
    }
}

fn copy_and_patch(src: impl AsRef<Path>, dest: impl AsRef<Path>) -> io::Result<()> {
    /// Regex substitutions to apply to the prost-generated output
    const REPLACEMENTS: &[(&str, &str)] = &[
        // Use `tendermint-proto` proto definitions
        ("(super::)+tendermint", "tendermint_proto"),
        // Feature-gate gRPC client modules
        (
            "/// Generated client implementations.",
            "/// Generated client implementations.\n\
             #[cfg(feature = \"grpc\")]\n\
             #[cfg_attr(docsrs, doc(cfg(feature = \"grpc\")))]",
        ),
        // Feature-gate gRPC impls which use `tonic::transport`
        (
            "impl(.+)tonic::transport(.+)",
            "#[cfg(feature = \"grpc-transport\")]\n    \
             #[cfg_attr(docsrs, doc(cfg(feature = \"grpc-transport\")))]\n    \
             impl${1}tonic::transport${2}",
        ),
        // Feature-gate gRPC server modules
        (
            "/// Generated server implementations.",
            "/// Generated server implementations.\n\
             #[cfg(feature = \"grpc\")]\n\
             #[cfg_attr(docsrs, doc(cfg(feature = \"grpc\")))]",
        ),
    ];
    


    // Skip proto files belonging to `EXCLUDED_PROTO_PACKAGES`
    for package in EXCLUDED_PROTO_PACKAGES {
        if let Some(filename) = src.as_ref().file_name().and_then(OsStr::to_str) {
            if filename.starts_with(&format!("{}.", package)) {
                return Ok(());
            }
        }
    }

    let mut contents = fs::read_to_string(src)?;

    for &(regex, replacement) in REPLACEMENTS {
        contents = Regex::new(regex)
            .unwrap_or_else(|_| panic!("invalid regex: {}", regex))
            .replace_all(&contents, replacement)
            .to_string();
    }

    fs::write(dest, &contents)
}

fn patch_file(path: impl AsRef<Path>, pattern: &Regex, replacement: &str) -> io::Result<()> {
    let mut contents = fs::read_to_string(&path)?;
    contents = pattern.replace_all(&contents, replacement).to_string();
    fs::write(path, &contents)
}

/// Fix clashing type names in prost-generated code. See cosmos/cosmos-rust#154.
fn apply_patches(output_dir: &Path) {
    for (pattern, replacement) in [
        ("enum Validators", "enum Policy"),
        (
            "stake_authorization::Validators",
            "stake_authorization::Policy",
        ),
    ] {
        patch_file(
            &output_dir.join("cosmos.staking.v1beta1.rs"),
            &Regex::new(pattern).unwrap(),
            replacement,
        )
        .expect("error patching cosmos.staking.v1beta1.rs");
        patch_file(
            &output_dir.join("initia.mstaking.v1.rs"),
            &Regex::new(pattern).unwrap(),
            replacement,
        )
        .expect("error patching initia.mstaking.v1.rs");
    }
    fs::rename(
        &output_dir.join("initia.r#move.v1.rs"),
        &output_dir.join("initia.move.v1.rs"),
    ).unwrap();
}
