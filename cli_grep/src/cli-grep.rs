use exitfailure::ExitFailure;
use failure::ResultExt;
use log::{info, warn};
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    pattern: String,

    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

#[test]
fn find_a_match() {
    let mut result = Vec::new();
    cli_grep::find_matches("lorem ipsum\ndolor sit amet", "lorem", &mut result);
    assert_eq!(result, b"lorem ipsum\n");
}

fn main() -> Result<(), ExitFailure> {
    env_logger::init();
    info!("starting up");
    let args = Cli::from_args();

    println!("{}", args.pattern);
    println!("{}", args.path.display());

    let content = std::fs::read_to_string(&args.path)
        .with_context(|_| format!("could not read file `{}`", args.path.display()))?;

        cli_grep::find_matches(&content, &args.pattern, &mut std::io::stdout());

    warn!("oops, nothing implemented!");

    Ok(())
}
