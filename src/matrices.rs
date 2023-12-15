#[derive(Debug, Clone, Copy)]
pub struct Vector {
    pub(crate) x: f64,
    pub(crate) y: f64,
    pub(crate) z: f64
}
impl Vector {
    pub fn x(&self) -> f64 {
        self.x
    }
    pub fn y(&self) -> f64 {
        self.y
    }
    pub fn z(&self) -> f64 {
        self.z
    }
    pub fn new(x:f64, y:f64, z:f64) -> Vector {
        Vector {
            x,
            y,
            z,
        }
    }
    pub fn origin() -> Vector {
        Vector {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn from_vec(from: Vec<f64>) -> Vector {
        Vector::new(from[0], from[1], from[2])
    }
    pub fn return_normalised(&self) -> Vector {
        let magnitude = Vector::magnitude(&self);
        Vector::new(self.x / magnitude, self.y / magnitude, self.z / magnitude)
    }
    pub fn magnitude_squared(&self) -> f64 {
        self.x.powi(2) + self.y.powi(2) + self.z.powi(2)
    }
    pub fn magnitude(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).powf(0.5)
    }
    pub fn return_multiply(&self, f: f64) -> Vector {
        Vector::new(self.x * f, self.y * f, self.z * f)
    }
    pub fn vector_between(from: &Vector, to: &Vector) -> Vector {
        Vector::new(to.x - from.x, to.y - from.y, to.z - from.z)
    }
    pub fn dot(a: &Vector, b: &Vector) -> f64 {
        a.x * b.x + a.y * b.y + a.z * b.z
    }
    pub fn copy(&self) -> Vector {
        Vector::new(self.x, self.y, self.z)
    }
    pub fn return_reflected(ray: &Vector, normal: &Vector) -> Vector {
        let mut to_ret = ray.return_normalised();
        let to_sub = normal.return_normalised();
        to_sub.return_multiply(2.0 * Vector::dot(&to_ret, &to_sub));
        to_ret.minus(&to_sub);
        to_ret
    }
    pub fn minus(&mut self, other: &Vector) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }

    pub fn plus(&mut self, other: &Vector) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
    pub fn return_plus(&self, other:&Vector) -> Vector {
        Vector::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
    pub fn return_three_matrix_mut(&self, mat: &ThreeMatrix) -> Vector {
        Vector::new(Vector::dot(&mat.row_zero, &self), Vector::dot(&mat.row_one, &self), Vector::dot(&mat.row_two, &self))
    }
    pub fn three_rotation_matrix_between(from: &Vector, to: &Vector) -> ThreeMatrix {
        //TODO generalise the first vector - for now it is only neg z
        let to = to.return_normalised();
        let sin_theta = to.y;
        let cos_theta = (1.0 - sin_theta.powi(2)).powf(0.5);
        let cos_phi = to.z / cos_theta;
        let sin_phi  = (1.0 - cos_phi.powi(2)).powf(0.5);
        let x_rot = ThreeMatrix {
            row_zero: Vector::new(1.0, 0.0, 0.0),
            row_one: Vector::new(0.0, cos_theta, -sin_theta),
            row_two: Vector::new(0.0, sin_theta, cos_theta)
        };
        let y_rot = ThreeMatrix {
            row_zero: Vector::new(-cos_phi, 0.0, -sin_phi),
            row_one: Vector::new(0.0, 1.0, 0.0),
            row_two: Vector::new(sin_phi, 0.0, -cos_phi)
        };
        let a = ThreeMatrix::return_multiply(&x_rot, &y_rot);
        let b = a.return_transpose();
        //let c= ThreeMatrix::return_multiply(&a, &b);
        a
    }

    pub fn to_string(&self) -> String {
        self.x.to_string() + " " + &*self.y.to_string() + " " + &*self.z.to_string()
    }
}
#[derive(Debug)]
pub struct ThreeMatrix {
    row_zero: Vector,
    row_one: Vector,
    row_two: Vector
}
impl ThreeMatrix {

    pub fn col_zero(&self) -> Vector {
        Vector::new(self.row_zero.x, self.row_one.x, self.row_two.x)
    }
    pub fn col_one(&self) -> Vector {
        Vector::new(self.row_zero.y, self.row_one.y, self.row_two.y)
    }
    pub fn col_two(&self) -> Vector {
        Vector::new(self.row_zero.z, self.row_one.z, self.row_two.z)
    }
    pub fn return_multiply(a: &ThreeMatrix, b: &ThreeMatrix) -> ThreeMatrix {
        ThreeMatrix {
            row_zero: Vector::new(Vector::dot(&a.row_zero, &b.col_zero()), Vector::dot(&a.row_zero, &b.col_one()), Vector::dot(&a.row_zero, &b.col_two())),
            row_one: Vector::new(Vector::dot(&a.row_one, &b.col_zero()), Vector::dot(&a.row_one, &b.col_one()), Vector::dot(&a.row_one, &b.col_two())),
            row_two: Vector::new(Vector::dot(&a.row_two, &b.col_zero()), Vector::dot(&a.row_two, &b.col_one()), Vector::dot(&a.row_two, &b.col_two())),
        }
    }

    pub fn determinant(&self) -> f64 {
        self.row_zero.x * (self.row_one.y * self.row_two.z - self.row_one.z - self.row_two.y)
            + self.row_zero.y * (self.row_one.x * self.row_two.z - self.row_one.z * self.row_two.x)
            + self.row_zero.z * (self.row_one.x * self.row_two.y - self.row_one.y * self.row_two.x)
    }

    pub fn return_transpose(&self) -> ThreeMatrix {
        ThreeMatrix {
            row_zero: self.col_zero(),
            row_one: self.col_one(),
            row_two: self.col_two(),
        }
    }
}