use crate::vec3::Vec3;
use std::fs::File;
use std::io::prelude::*;

pub fn write(filename: String, width: u32, height: u32, pixels: Vec<Vec3>) -> std::io::Result<()> {
    let mut file = File::create(filename)?;
    file.write(format!("P3\n{} {}\n255\n", width, height).as_bytes())?;
    //Convert the vector of pixels into an iterator. Run to_string on each pixel. Collect back into a vector. Join with a space between. Convert to byte array
    file.write(pixels.iter().map(|p| p.clone().mul_scalar(255.999).to_string()).collect::<Vec<String>>().join("\n").as_bytes())?;
    Ok(())
}