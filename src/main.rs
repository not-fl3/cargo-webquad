use cargo::{
    core::{
        self,
        compiler::{BuildConfig, CompileKind, CompileMode, CompileTarget, RustcTargetData},
        resolver::{
            features::{ForceAllTargets, HasDevUnits},
            CliFeatures,
        },
    },
    ops,
    util::{self, important_paths::find_root_manifest_for_wd, interning::InternedString},
};

mod cli;

fn main() {
    let args = cli::parse_cli();

    let mut config = util::Config::default().unwrap();

    config
        .configure(0, false, None, false, false, false, &None, &[], &[])
        .unwrap();

    let root_manifest = find_root_manifest_for_wd(config.cwd()).unwrap();

    let ws = core::Workspace::new(&root_manifest, &config).unwrap();

    let target_dir = root_manifest
        .parent()
        .unwrap()
        .join("target")
        .join("web-artifacts");

    std::fs::create_dir_all(&target_dir).unwrap();

    //let root_build_dir = util::get_root_build_directory(&ws, config);
    //println!("{:?}", root_build_dir);

    let targets = &[CompileKind::Target(
        CompileTarget::new("wasm32-unknown-unknown").unwrap(),
    )];
    let target_data = RustcTargetData::new(&ws, targets).unwrap();

    let cli_features = CliFeatures::new_all(false);

    let specs = ops::Packages::Default.to_package_id_specs(&ws).unwrap();

    let ws_resolve = ops::resolve_ws_with_opts(
        &ws,
        &target_data,
        targets,
        &cli_features,
        &specs,
        HasDevUnits::No,
        ForceAllTargets::No,
    )
    .unwrap();

    let jobs = None;
    let compile_mode = CompileMode::Build;
    let mut build_config = BuildConfig::new(
        &config,
        jobs,
        &["wasm32-unknown-unknown".to_string()],
        compile_mode,
    )
    .unwrap();

    if args.release {
        build_config.requested_profile = InternedString::new("release");
    }

    let bins = vec![];
    let examples = match args.example {
        Some(example) => vec![example.to_owned()],
        _ => vec![],
    };
    let compile_opts = ops::CompileOptions {
        build_config,
        cli_features: CliFeatures::new_all(false),
        spec: ops::Packages::Default,
        filter: ops::CompileFilter::from_raw_arguments(
            false, //lib_only
            bins,
            false, //all_bins
            vec![],
            false,
            examples,
            false,
            vec![],
            false,
            false,
        ),
        target_rustdoc_args: None,
        target_rustc_args: None,
        local_rustdoc_args: None,
        rustdoc_document_private_items: false,
        honor_rust_version: false,
    };

    let compiled = ops::compile(&ws, &compile_opts).unwrap();

    assert!(compiled.binaries.len() == 1, "More than one binaries in build artifacts, not supported yet (check the code of ops::CompileFilter::from_raw_arguments..)");

    let wasm_path = &compiled.binaries.first().unwrap().path;

    let macroquad_pkg = ws_resolve
        .pkg_set
        .packages()
        .find(|package| package.name() == "macroquad");

    assert!(
        macroquad_pkg.is_some(),
        "No sapp-wasm in dependencies tree!"
    );

    let js_bundle_path = macroquad_pkg
        .unwrap()
        .root()
        .join("js")
        .join("mq_js_bundle.js");

    std::fs::copy(js_bundle_path, target_dir.join("mq_js_bundle.js")).unwrap();
    if let Some(assets_folder) = args.assets_folder {
        println!("Copying {} to {:?}", assets_folder, target_dir);
        fs_extra::dir::copy(
            assets_folder,
            &target_dir,
            &fs_extra::dir::CopyOptions {
                overwrite: true,
                ..fs_extra::dir::CopyOptions::new()
            },
        )
        .unwrap();
    }

    std::fs::copy(wasm_path, target_dir.join("a.wasm")).unwrap();
    std::fs::write(target_dir.join("index.html"), include_str!("index.html")).unwrap();

    if args.command == cli::Command::Serve {
        let _ = std::env::set_current_dir(target_dir);
        println!("addr: http://localhost:8080");
        devserver_lib::run(&"localhost", 8080, "", false, "");
    }
}
