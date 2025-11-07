use clap::Parser;
use color_thief::{Algorithm, Color, ColorFormat};
use image::{ImageBuffer, Rgb};
use log::info;
use miette::{Diagnostic, IntoDiagnostic, Result};
use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Debug, Error, Diagnostic)]
enum AppError {
    #[error("Invalid quality value provided: {0}")]
    #[diagnostic(help("Please provide a quality value between 1 and 10."))]
    InvalidQuality(u8),

    #[error("Invalid maximum colors value provided: {0}")]
    #[diagnostic(help("Please provide a maximum colors value between 2 and 255."))]
    InvalidMaxColors(u8),

    #[error("The image path does not exist: {0:?}")]
    #[diagnostic(help("Please provide a valid image path."))]
    InvalidImagePath(PathBuf),

    #[error("Failed to open image at {0:?}")]
    #[diagnostic(help("Ensure the file exists and is a supported image format."))]
    ImageOpenError(PathBuf),

    #[error("Failed to get color palette from the image.")]
    PaletteExtractionError,

    #[error("Failed to save the generated color grid image.")]
    ImageSaveError,
}

#[derive(Parser)]
struct Args {
    #[clap(help = "Path to the image from where to extract the color palette")]
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
        help = "Add prefixes (to make output human-readable, e.g., \"Color 1: #000000\")."
    )]
    prefix: Option<String>,

    #[clap(
        long,
        help = "Path to save the generated image showing the palette colors."
    )]
    grid_output: Option<PathBuf>,
}

fn validate_args(args: &Args) -> Result<()> {
    info!("Validating provided arguments");

    if !(1..=10).contains(&args.quality) {
        return Err(AppError::InvalidQuality(args.quality).into());
    }
    if !(2..=255).contains(&args.max_colors) {
        return Err(AppError::InvalidMaxColors(args.max_colors).into());
    }
    if !args.image_path.exists() {
        return Err(AppError::InvalidImagePath(args.image_path.clone()).into());
    }

    Ok(())
}

fn extract_palette(image_path: &Path, quality: u8, max_colors: u8) -> Result<Vec<Color>> {
    info!("Extracting palette from {:?}", &image_path);

    let img =
        image::open(image_path).map_err(|_| AppError::ImageOpenError(image_path.to_path_buf()))?;
    let rgb_img = img.to_rgb8();

    let palette = color_thief::get_palette(
        // TODO: make this configurable in the future
        Algorithm::KMeans,
        &rgb_img,
        ColorFormat::Rgb,
        quality,
        max_colors,
    )
    .map_err(|_| AppError::PaletteExtractionError)?;

    info!("Successfully extracted color palette");
    Ok(palette)
}

fn save_palette_grid(palette: &[Color], output_path: &Path) -> Result<()> {
    let cell_size = 50; // Each color block is 50x50 pixels
    let columns = palette.len() / 2; // Number of blocks per row
    let rows = (palette.len() + columns - 1) / columns;

    let width = columns * cell_size;
    let height = rows * cell_size;

    let mut img = ImageBuffer::new(width as u32, height as u32);

    for (i, color) in palette.iter().enumerate() {
        let x = (i % columns) * cell_size;
        let y = (i / columns) * cell_size;

        for dx in 0..cell_size {
            for dy in 0..cell_size {
                img.put_pixel(
                    (x + dx) as u32,
                    (y + dy) as u32,
                    Rgb([color.r, color.g, color.b]),
                );
            }
        }
    }

    img.save(output_path)
        .map_err(|_| AppError::ImageSaveError.into())
}

fn main() -> Result<()> {
    let args = Args::parse();

    if args.verbose {
        simple_logger::init().into_diagnostic()?;
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

    if let Some(output_path) = args.grid_output {
        info!("Saving palette grid image to {:?}", output_path);
        save_palette_grid(&palette, &output_path)?;
        println!("Palette grid saved to {:?}", output_path);
    }

    Ok(())
}
