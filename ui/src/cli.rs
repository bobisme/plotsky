#[derive(Debug, clap::Args)]
pub struct SvgToPngArgs {
    /// input svg file
    #[clap(short, long)]
    pub input: String,
    /// output png file
    #[clap(short, long)]
    pub output: String,
}

#[derive(Debug, clap::Args)]
pub struct SimplifySvgArgs {
    /// input svg file
    #[clap(short, long)]
    pub input: String,
    /// output svg file
    #[clap(short, long)]
    pub output: String,
}

#[derive(Debug, clap::Args)]
pub struct SvgToGcodeArgs {
    /// input svg file
    #[clap(short, long)]
    pub input: String,
    /// output gcode file
    #[clap(short, long)]
    pub output: String,
}

#[derive(Debug, clap::Subcommand)]
pub enum Command {
    SvgToPng(SvgToPngArgs),
    SimplifySvg(SimplifySvgArgs),
    SvgToGcode(SvgToGcodeArgs),
}

#[derive(Debug, clap::Parser)]
pub struct Args {
    #[clap(subcommand)]
    pub command: Command,
}
