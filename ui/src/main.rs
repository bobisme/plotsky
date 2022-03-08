pub(crate) mod cli;

use clap::Parser;
use tracing::info;

fn load_svg(input: &str) -> usvg::Tree {
    info!(input, "parsing file");
    let mut opt = usvg::Options {
        resources_dir: std::fs::canonicalize(input)
            .ok()
            .and_then(|p| p.parent().map(|p| p.to_path_buf())),
        ..Default::default()
    };
    opt.fontdb.load_system_fonts();
    let svg_data = std::fs::read(input).unwrap();
    usvg::Tree::from_data(&svg_data, &opt.to_ref()).unwrap()
}

fn svg_to_png(input: &str, output: &str) {
    let tree = load_svg(input);
    let pixmap_size = tree.svg_node().size.to_screen_size();
    let mut pixmap = tiny_skia::Pixmap::new(pixmap_size.width(), pixmap_size.height()).unwrap();
    resvg::render(
        &tree,
        usvg::FitTo::Original,
        tiny_skia::Transform::default(),
        pixmap.as_mut(),
    )
    .unwrap();
    pixmap.save_png(output).unwrap();
}

fn simplify_svg(input: &str, output: &str) -> Result<(), std::io::Error> {
    let tree = load_svg(input);
    let opt = usvg::XmlOptions::default();
    std::fs::write(output, tree.to_string(&opt))
}

fn svg_to_gcode(input: &str, output: &str) -> eyre::Result<()> {
    let tree = load_svg(input);
    let opt = usvg::XmlOptions::default();
    let svg = tree.to_string(&opt);
    let document = roxmltree::Document::parse(&svg)?;
    let settings = svg2gcode::Settings::default();
    let options = svg2gcode::ConversionOptions::default();
    let machine = svg2gcode::Machine::default();
    let program = svg2gcode::svg2program(&document, &settings.conversion, options, machine);

    g_code::emit::format_gcode_io(
        &program,
        g_code::emit::FormatOptions::default(),
        std::fs::File::create(output)?,
    )?;
    Ok(())
}

fn main() -> eyre::Result<()> {
    let args = cli::Args::parse();
    match args.command {
        cli::Command::SvgToPng(a) => svg_to_png(&a.input, &a.output),
        cli::Command::SimplifySvg(a) => simplify_svg(&a.input, &a.output)?,
        cli::Command::SvgToGcode(a) => svg_to_gcode(&a.input, &a.output)?,
    };
    Ok(())
}
