use libdeno::{run_source_with_output, run_with_output, LibdenoOptions, SourceLang::JavaScript};
use clap::{Parser};

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

const MAIN_SCRIPT: &[u8] = include_bytes!("../dist/main.js");

#[derive(Parser, Default)]
#[clap(author = "Cassiopeia", version = "0.1.0", about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    script: Option<String>,
    #[clap(short, long)]
    json: Option<String>,
    #[clap(short, long, default_value = "2567")]
    port: u16,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let options = LibdenoOptions {
        permissions: vec![],
        extensions: None,
        allow_all_permissions: true,
        prompt: false,
        args: if let Some(s) = cli.json {vec![s]} else { vec![] },
        cwd: None,
        max_heap_bytes: None,
        execution_deadline: None,
        capture_stdout: false,
        capture_stderr: false,
        max_capture_bytes: None,
        features: Some(vec!["kv".to_string()]),
    };
    unsafe { std::env::set_var("PORT", cli.port.to_string()) };
    if let Some(script) = cli.script {
        if let Ok(b) = std::fs::exists(&script) && b {
            run_with_output(&script, &options)?;
        }
        return Err(anyhow::anyhow!("script not found"));
    } else {
        run_source_with_output(MAIN_SCRIPT, JavaScript, ".", &options)?;
    }
    Ok(())
}
