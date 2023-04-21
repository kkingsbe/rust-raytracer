use crate::{Vec3, vec3::Vec3Type};

pub enum ColorError {
    InvalidVector(Vec3Type)
}

pub fn write_color(pixel_color: Vec3) -> Result<(), ColorError> {
    if pixel_color.vec_type != Vec3Type::Color {
        return Err(ColorError::InvalidVector(pixel_color.vec_type));
    }
    println!("{} {} {}", (255.999 * pixel_color.x) as u8, (255.999 * pixel_color.y) as u8, (255.999 * pixel_color.z) as u8);
    Ok(())
}