use std::path::PathBuf;
use std::str::FromStr;
use std::{process, sync::Arc};
use std::{path, str};

use rustc_errors::DIAGNOSTICS;
use rustc_errors::registry;
use rustc_hash::FxHashMap;
use rustc_session::config;

pub(crate) fn compile_protogen(path: PathBuf) -> Result<String, Box<dyn std::error::Error>> {
    let out = process::Command::new("rustc")
        .arg("--print=sysroot")
        .arg("--edition 2021")
        .current_dir(".")
        .output()?;
    let sysroot = str::from_utf8(&out.stdout).unwrap().trim();
    let errors = registry::Registry::new(&DIAGNOSTICS);
    let config = rustc_interface::interface::Config {
        // Command line options
        opts: config::Options {
            maybe_sysroot: Some(path::PathBuf::from(sysroot)),
            ..config::Options::default()
        },
        // cfg! configuration in addition to the default ones
        crate_cfg: Vec::new(),       // FxHashSet<(String, Option<String>)>
        crate_check_cfg: Vec::new(), // CheckCfg
        input: config::Input::Str {
            name: rustc_span::FileName::Custom("mod.rs".into()),
            input: std::fs::read_to_string(path)?,
        },
        output_dir: Some(PathBuf::from_str("./built")?),  // Option<PathBuf>
        output_file: None, // Option<PathBuf>
        file_loader: None, // Option<Box<dyn FileLoader + Send + Sync>>
        locale_resources: rustc_driver::DEFAULT_LOCALE_RESOURCES,
        lint_caps: FxHashMap::default(), // FxHashMap<lint::LintId, lint::Level>
        // This is a callback from the driver that is called when [`ParseSess`] is created.
        psess_created: None, //Option<Box<dyn FnOnce(&mut ParseSess) + Send>>
        // This is a callback from the driver that is called when we're registering lints;
        // it is called during plugin registration when we have the LintStore in a non-shared state.
        //
        // Note that if you find a Some here you probably want to call that function in the new
        // function being registered.
        register_lints: None, // Option<Box<dyn Fn(&Session, &mut LintStore) + Send + Sync>>
        // This is a callback from the driver that is called just after we have populated
        // the list of queries.
        //
        // The second parameter is local providers and the third parameter is external providers.
        override_queries: None, // Option<fn(&Session, &mut ty::query::Providers<'_>, &mut ty::query::Providers<'_>)>
        // Registry of diagnostics codes.
        registry: errors,
        make_codegen_backend: None,
        expanded_args: Vec::new(),
        ice_file: None,
        hash_untracked_state: None,
        using_internal_features: Arc::default(),
    };
    rustc_interface::run_compiler(config, |compiler| {
        compiler.enter(|queries| {
            // Parse the program and print the syntax tree.
            let parse = queries.parse().unwrap().get_mut().clone();
            println!("{parse:?}");
            // Analyze the program and inspect the types of definitions.
            queries.global_ctxt().unwrap().enter(|tcx| {
                for id in tcx.hir().items() {
                    let hir = tcx.hir();
                    let item = hir.item(id);
                    match item.kind {
                        rustc_hir::ItemKind::Static(_, _, _) | rustc_hir::ItemKind::Fn(_, _, _) => {
                            let name = item.ident;
                            let ty = tcx.type_of(item.hir_id().owner.def_id);
                            println!("{name:?}:\t{ty:?}")
                        }
                        _ => (),
                    }
                }
            })
        });
    });

    Ok("".into())
}