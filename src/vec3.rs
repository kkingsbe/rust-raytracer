#[derive(PartialEq, Clone, Debug)]
pub enum Vec3Type {
    Point,
    Color,
}

#[derive(Debug, Clone)]
pub struct Vec3 {
    pub x: f32, 
    pub y: f32, 
    pub z: f32,
    pub vec_type: Vec3Type
}

#[derive(PartialEq, Debug)]
pub enum Vec3Error {
    MismatchedTypes(Vec3Type, Vec3Type)
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32, vec_type: Vec3Type) -> Vec3 {
        Vec3 { x, y, z, vec_type }
    }

    pub fn to_string(&self) -> String {
        format!("{} {} {}", self.x, self.y, self.z)
    }

    pub fn length(&self) -> f32 {
        return ((self.x * self.x) + (self.y * self.y) + (self.z * self.z)).sqrt();
    }

    pub fn add(self, other: Vec3) -> Result<Vec3, Vec3Error> {
        if self.vec_type != other.vec_type {
            return Err(Vec3Error::MismatchedTypes(self.vec_type, other.vec_type));
        }

        Ok(Vec3::new(self.x + other.x, self.y + other.y, self.z + other.z, self.vec_type))
    }

    pub fn sub(self, other: Vec3) -> Result<Vec3, Vec3Error> {
        if self.vec_type != other.vec_type {
            return Err(Vec3Error::MismatchedTypes(self.vec_type, other.vec_type));
        }
        Ok(Vec3::new(self.x - other.x, self.y - other.y, self.z - other.z, self.vec_type))
    }

    pub fn mul(self, other: Vec3) -> Result<Vec3, Vec3Error> {
        if self.vec_type != other.vec_type {
            return Err(Vec3Error::MismatchedTypes(self.vec_type, other.vec_type));
        }
        Ok(Vec3::new(self.x * other.x, self.y * other.y, self.z * other.z, self.vec_type))
    }

    pub fn mul_scalar(self, other: f32) -> Vec3 {
        Vec3::new(self.x * other, self.y * other, self.z * other, self.vec_type)
    }

    pub fn div(self, other: Vec3) -> Result<Vec3, Vec3Error> {
        if self.vec_type != other.vec_type {
            return Err(Vec3Error::MismatchedTypes(self.vec_type, other.vec_type));
        }
        Ok(Vec3::new(self.x / other.x, self.y / other.y, self.z / other.z, self.vec_type))
    }

    pub fn div_scalar(self, other: f32) -> Vec3 {
        self.mul_scalar(1.0/other)
    }

    pub fn dot(&self, other: Vec3) -> Result<f32, Vec3Error> {
        if self.vec_type != other.vec_type {
            return Err(Vec3Error::MismatchedTypes(self.vec_type.clone(), other.vec_type));
        }
        Ok((self.x * other.x) + (self.y * other.y) + (self.z * other.z))
    }

     pub fn cross(&self, other: Vec3) -> Result<Vec3, Vec3Error> {
        if self.vec_type != other.vec_type {
            return Err(Vec3Error::MismatchedTypes(self.vec_type.clone(), other.vec_type));
        }
        Ok(Vec3::new(
            (self.y * other.z) - (self.z * other.y),
            (self.z * other.x) - (self.x * other.z),
            (self.x * other.y) - (self.y * other.x),
            self.vec_type.clone()
        ))
    }

    pub fn unit_vector(&self) -> Vec3 {
        let length = self.length();
        Vec3::new(self.x / length, self.y / length, self.z / length, self.vec_type.clone())
    }
}

