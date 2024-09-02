use std::io::Cursor;

use arboard::{Clipboard, ImageData, SetExtLinux};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "")]
    image_path: String,

    #[arg(short, long, default_value = "")]
    text_path: String,
}

fn main() {
    let args = Args::parse();

    if !args.image_path.is_empty() {
        let decoded_image = image::DynamicImage::from_decoder(image::codecs::png::PngDecoder::new(Cursor::new(std::fs::read(&args.image_path).expect(&format!("Could not read file at path: {}", args.image_path)).as_slice())).unwrap()).unwrap();

        let image_data = ImageData {
            width: decoded_image.width() as usize,
            height: decoded_image.height() as usize,
            bytes: decoded_image.as_bytes().into(),
        };

        if cfg!(target_os = "linux") {
            Clipboard::new().unwrap().set().wait().image(image_data).unwrap();
        } else {
            Clipboard::new().unwrap().set_image(image_data).unwrap();
        }
    } else if !args.text_path.is_empty() {
        let text = std::fs::read_to_string(args.text_path).unwrap();
        if cfg!(target_os = "linux") {
            Clipboard::new().unwrap().set().wait().text(text).unwrap();
        } else {
            Clipboard::new().unwrap().set_text(text).unwrap();
        }
    };
}