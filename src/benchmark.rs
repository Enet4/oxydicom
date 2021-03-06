use std::convert::TryInto;
use std::path::Path;
use std::fs;
use std::time::Instant;
use dicom::object::open_file;
use anyhow::{Result, anyhow};

mod utils;
mod decoding;

use utils::RawImage;
use decoding::get_image;

fn main() -> Result<()> {

    let path_str = std::env::args().skip(1).next().unwrap();
    let path = Path::new(&path_str);

    for category_path in fs::read_dir(path)? {

        let category_path = category_path?.path();
        let category_path_str = category_path.to_str().ok_or(anyhow!("Not unicode"))?;
        println!("\n{}", category_path_str);

        for dicom_path in fs::read_dir(category_path)? {

            let dicom_path = dicom_path?.path();
            let filename = dicom_path
                .file_name().ok_or(anyhow!("Incorrect folder structure"))?
                .to_str().ok_or(anyhow!("Not unicode"))?;

            let t0 = Instant::now();
            let dicom = open_file(dicom_path.as_os_str())?;
            let t1 = Instant::now();
            let decoded_image_data = get_image(&dicom)?;
            let RawImage { bytes, .. } = decoded_image_data;
            let t2 = Instant::now();

            let dt1: i32 = (t1 - t0).as_millis().try_into()?;
            let dt2: i32 = (t2 - t1).as_millis().try_into()?;

            let dt1: f64 = dt1.try_into()?;
            let dt2: f64 = dt2.try_into()?;

            let dt1: f64 = dt1 / 1000.0;
            let dt2: f64 = dt2 / 1000.0;

            println!("  {}", filename);
            println!("    {} bytes", bytes.len());
            println!("    Opening  : {}s", dt1);
            println!("    Decoding : {}s", dt2);
        }
    }

    Ok(())
}