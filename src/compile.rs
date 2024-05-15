use std::path::PathBuf;

use cargo::core::compiler::CompileMode;
use cargo::core::Workspace;
use cargo::ops::CompileOptions;
use cargo::GlobalContext;

pub(crate) fn compile_protogen<'a>(path: PathBuf) -> anyhow::Result<Vec<PathBuf>> {
    let config = GlobalContext::default()?;
    let ws = Workspace::new(path.as_path(), &config)?;
    let compile = cargo::ops::compile(&ws, &CompileOptions::new(&config, CompileMode::Build)?)?;
    Ok(compile.binaries.into_iter().map(|x| x.path).collect())
}

// pub(crate) fn compile_protogen_old(path: PathBuf) -> CliResult {
//     let mut config = GlobalContext::default()?;

//     let subcommand = subcommand_build("cbuild", "Build the crate C-API");
//     let mut app = clap::command!()
//         .dont_collapse_args_in_usage(true)
//         .allow_external_subcommands(true)
//         .subcommand(subcommand);

//     let args = app.clone().get_matches();

//     let subcommand_args = match args.subcommand() {
//         Some(("cbuild", args)) => args,
//         Some((cmd, args)) => {
//             return run_cargo_fallback(cmd, args);
//         }
//         _ => {
//             // No subcommand provided.
//             app.print_help()?;
//             return Ok(());
//         }
//     };

//     if subcommand_args.flag("version") {
//         println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
//         return Ok(());
//     }

//     global_context_configure(&mut config, subcommand_args)?;

//     let mut ws = subcommand_args.workspace(&config)?;

//     let _ = cbuild(&mut ws, &config, subcommand_args, "dev")?;

//     Ok(())
// }