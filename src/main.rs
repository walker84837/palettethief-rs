use anyhow::{bail, Context, Result};
use clap::Parser;
use color_thief::{Color, ColorFormat as ThiefFormat};
use log::info;
use std::path::{Path, PathBuf};

#[derive(Parser)]
struct Args {
    #[clap(help = "Path the image from where to extract the color palette")]
    image_path: PathBuf,

    #[clap(
        short,
        long,
        default_value = "10",
        help = "Quality of the palette (1..10)"
    )]
    quality: u8,

    #[clap(
        short,
        long,
        default_value = "6",
        help = "Maximum colors of the palette (2..255)"
    )]
    max_colors: u8,

    #[clap(short, long, help = "Enable verbose mode (logging)")]
    verbose: bool,

    #[clap(long, help = "Output colors using RGB format [default: hex]")]
    rgb: bool,

    #[clap(
        long,
        help = "Add prefixes (to make output human-readable. ie. \"Color 1: #000000\")."
    )]
    prefix: Option<String>,
}

fn validate_args(args: &Args) -> Result<()> {
    info!("Checking whether arguments are provided correctly");

    if !(1..=10).contains(&args.quality) {
        bail!("The quality provided isn't valid: {}", args.quality);
    }
    if !(2..=255).contains(&args.max_colors) {
        bail!(
            "The maximum colours provided are incorrect: {}",
            args.max_colors
        );
    }
    if !args.image_path.exists() {
        bail!("The image path does not exist: {:?}", args.image_path);
    }

    Ok(())
}

fn extract_palette(image_path: &Path, quality: u8, max_colors: u8) -> Result<Vec<Color>> {
    info!("Extracting palette from {:?}", &image_path);

    let img = image::open(image_path)
        .with_context(|| format!("Failed to open image at {:?}", image_path))?;
    let rgb_img = img.to_rgb8();

    let palette = color_thief::get_palette(&rgb_img, ThiefFormat::Rgb, quality, max_colors)
        .context("Failed to get color palette")?;

    info!("Successfully extracted color palette");
    Ok(palette)
}

fn main() -> Result<()> {
    let args = Args::parse();

    if args.verbose {
        simple_logger::init().unwrap();
    }

    validate_args(&args)?;

    let palette = extract_palette(&args.image_path, args.quality, args.max_colors)?;

    for (i, color) in palette.iter().enumerate() {
        let color_str = if args.rgb {
            format!("rgb({}, {}, {})", color.r, color.g, color.b)
        } else {
            format!("#{:02x}{:02x}{:02x}", color.r, color.g, color.b)
        };

        let output = if let Some(ref prefix) = args.prefix {
            format!("{} {}: {}", prefix, i + 1, color_str)
        } else {
            color_str
        };

        println!("{}", output);
    }

    Ok(())
}
