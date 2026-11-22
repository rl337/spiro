mod catalog;
#[cfg(feature = "gguf")]
mod gguf_resolve;
mod harness;
mod registry;
mod scenarios;
#[cfg(feature = "safetensors")]
mod spiro_image;

use std::env;
use std::path::PathBuf;
use std::process;

use harness::{E2eContext, E2eError};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .unwrap_or_else(|_| PathBuf::from("."))
}

fn usage() {
    eprintln!(
        "uie-e2e — end-to-end harness (application + command-pattern tests)\n\
         \n\
         uie-e2e list\n\
         uie-e2e list-models [--no-mock]\n\
         uie-e2e list-vl-bundles [--no-mock]   (gguf feature)\n\
         uie-e2e run <name-or-glob> [-- TEST_ARGS...]\n\
         \n\
         Examples:\n\
           cargo run -p uie-e2e --features gguf -- run qwen3_2b_chat_completion\n\
           cargo run -p uie-e2e --features gguf -- run 'qwen*' -- --prompt Hi --max-new 24"
    );
}

fn split_test_args(args: &[String]) -> (&[String], &[String]) {
    if let Some(pos) = args.iter().position(|a| a == "--") {
        (&args[..pos], &args[pos + 1..])
    } else {
        (args, &[])
    }
}

fn cmd_list() -> Result<(), E2eError> {
    for t in registry::all_tests() {
        println!("{}\t{}", t.name(), t.description());
    }
    Ok(())
}

fn cmd_list_models(no_mock: bool) -> Result<(), E2eError> {
    let repo = repo_root();
    let roots = catalog::roots_from_env(&repo).map_err(|e| E2eError::Failed(e.to_string()))?;
    let catalog = catalog::catalog_with_overlay(&repo, roots, !no_mock);
    for e in catalog.registry().entries() {
        println!(
            "{}\t{:?}\t{:?}\t{}",
            e.id,
            e.role,
            e.format,
            e.path.display()
        );
    }
    eprintln!(
        "(indexed {} files from {} roots; mock overlay: {})",
        catalog.registry().stats().files_indexed,
        catalog.registry().stats().roots_scanned,
        !no_mock
    );
    Ok(())
}

#[cfg(feature = "gguf")]
fn cmd_list_vl_bundles(no_mock: bool) -> Result<(), E2eError> {
    use crate::gguf_resolve::list_qwen_vl_bundles;

    let repo = repo_root();
    let roots = catalog::roots_from_env(&repo).map_err(|e| E2eError::Failed(e.to_string()))?;
    let catalog = catalog::catalog_with_overlay(&repo, roots, !no_mock);
    let bundles = list_qwen_vl_bundles(&catalog);
    if bundles.is_empty() {
        eprintln!("no Qwen .gguf + mmproj pairs found under SPIRO_MODEL_ROOTS");
        eprintln!("expected LM Studio layout: ~/.lmstudio/models/<publisher>/<repo>/*.gguf");
        return Ok(());
    }
    for b in &bundles {
        println!("llm\t{}", b.llm.display());
        println!("mmproj\t{}", b.mmproj.display());
        println!();
    }
    eprintln!(
        "({} vision bundle(s); auto-pick prefers qwen+2b+vl)",
        bundles.len()
    );
    Ok(())
}

fn cmd_run(pattern: &str, test_args: &[String]) -> Result<(), E2eError> {
    let tests = registry::all_tests();
    let selected = harness::select_tests(&tests, pattern)?;
    let mut ctx = E2eContext::new(repo_root());
    harness::run_selected(&selected, &mut ctx, test_args)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        usage();
        process::exit(2);
    }
    let cmd = args[1].as_str();
    let rest = &args[2..];
    let result = match cmd {
        "list" => cmd_list(),
        "list-models" => {
            let no_mock = rest.iter().any(|a| a == "--no-mock");
            cmd_list_models(no_mock)
        }
        "list-vl-bundles" => {
            #[cfg(not(feature = "gguf"))]
            {
                eprintln!("list-vl-bundles requires: cargo run -p uie-e2e --features gguf -- list-vl-bundles");
                process::exit(2);
            }
            #[cfg(feature = "gguf")]
            {
                let no_mock = rest.iter().any(|a| a == "--no-mock");
                cmd_list_vl_bundles(no_mock)
            }
        }
        "run" => {
            if rest.is_empty() {
                eprintln!("missing pattern for `run`");
                usage();
                process::exit(2);
            }
            let (run_args, test_args) = split_test_args(rest);
            let pattern = &run_args[0];
            cmd_run(pattern, test_args)
        }
        "help" | "-h" | "--help" => {
            usage();
            Ok(())
        }
        _ => {
            eprintln!("unknown subcommand: {cmd}");
            usage();
            process::exit(2);
        }
    };
    if let Err(e) = result {
        eprintln!("E2E failed: {e}");
        process::exit(1);
    }
}
