use nalgebra::{Point3, Vector3};
pub struct Ray {
    origin: Point3::<f64>,
    direction: Vector3::<f64>,
}

impl Ray {
    //construction
    pub fn new(origin: Point3::<f64>, direction: Vector3::<f64>) -> Ray {
        Ray {
            origin: origin,
            direction: direction,
        }
    }

    #[allow(dead_code)]
    pub fn get_origin(&self) -> Point3::<f64> {
        self.origin.clone()
    }

    #[allow(dead_code)]
    pub fn get_direction(&self) -> Vector3::<f64> {
        self.direction.clone()
    }

    #[allow(dead_code)]
    pub fn at(&self, t: f64) -> Point3::<f64> {
        self.origin + t * self.direction
    }

}
