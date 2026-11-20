#[cfg(feature = "gguf")]
mod gguf_resolve;
mod harness;
mod registry;
mod scenarios;

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
    let tests = registry::all_tests();
    if tests.is_empty() {
        eprintln!("(no tests — enable crate features, e.g. --features gguf)");
        return Ok(());
    }
    for t in &tests {
        println!("{}\t{}", t.name(), t.description());
    }
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
