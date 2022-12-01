use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Args {
    #[arg(
        long,
        short = 'P',
        default_value = "hotkeys",
        help = "Hotkey present name"
    )]
    pub preset: Option<String>,
    #[arg(long, short, action, help = "Paused on launch")]
    pub paused: bool,
    #[arg(long, short, action, help = "Verbose output")]
    pub verbose: bool,
    #[arg(long, short, action, help = "Watch for any event")]
    pub watch: bool,
}
