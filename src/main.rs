use std::env;
use std::fs::File;
use std::io::{Error, ErrorKind, Read, Seek, SeekFrom};

mod fileheader;
mod r2i;

fn main() -> Result<(), Error> {
    // Get command-line arguments
    let args: Vec<String> = env::args().collect();

    // Check if the number of arguments is correct
    if args.len() != 3 {
        eprintln!("Usage: {} <input_file> <output_image>", args[0]);
        return Ok(());
    }

    // Extract input and output file paths
    let input_path = &args[1];
    let output_path = &args[2];

    // Open the save file
    let mut file = File::open(input_path)?;

    // Read file header
    let header = fileheader::Fileheader::read_from_file(&mut file)?;

    // Check if the file is compressed
    if header.compressed {
        return Err(Error::new(
            ErrorKind::InvalidData,
            "Compressed files are not supported.",
        ));
    }

    // Read the raw pixel data
    let mut raw_data = Vec::new();
    file.seek(SeekFrom::Start(header.header_end))?;
    file.take(19200).read_to_end(&mut raw_data)?;

    // Create the image
    let image = r2i::create_image(&raw_data);

    // Attempt to save the image as PNG
    match image.save(output_path) {
        Ok(_) => println!("Image saved successfully."),
        Err(err) => {
            // Convert ImageError to std::io::Error
            return Err(Error::new(ErrorKind::Other, format!("Error saving image: {}", err)));
        }
    }

    Ok(())
}
