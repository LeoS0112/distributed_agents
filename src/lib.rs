pub use scene_object::Sphere;
// FOR RAY TRACING

pub trait SceneObject {
    // All scene objects must be closed 2D surfaces
    fn get_location(&self) -> Vector;
    fn set_location(&mut self, goto: &Vector) -> ();
    fn intersection(&self, ray: &Vector, starting_point: &Vector) -> Option<IntersectionData>;
}
pub mod scene_object {
    use crate::{IntersectionData, Vector};

    pub struct Sphere {
        pub radius: f64,
        pub location: Vector
    }

    impl Sphere {

        pub fn intersection(&self, ray: &Vector, starting_point: &Vector) -> Option<IntersectionData> {
            let det = (2.0 * (starting_point.x - self.location.x) * ray.x + 2.0 * (starting_point.y - self.location.y) * ray.y + 2.0 * (starting_point.z - self.location.z) * ray.z).powi(2)- 4.0 * (ray.x.powi(2) + ray.y.powi(2) + ray.z.powi(2)) * ((starting_point.x - self.location.x).powi(2)+ (starting_point.y - self.location.y).powi(2) + (starting_point.z - self.location.z).powi(2) - self.radius.powi(2));
            if det < 0.0 {
                return None;
            }
            let t;
            let non_det = - 2.0 * (ray.x * (starting_point.x - self.location.x) + ray.y * (starting_point.y - self.location.y) + ray.z * (starting_point.z - self.location.z));
            let denominator = 2.0 * ray.magnitude_squared();
            if non_det - det.powf(0.5) > 0.0 {
                t = (non_det - det.powf(0.5)) / denominator;
            }
            else if non_det + det.powf(0.5) > 0.0 {
                t = (non_det + det.powf(0.5)) / denominator;
            }
            else {
                return None;
            }
            let location = ray.return_multiply(t);
            let normal = Vector::vector_between(&self.location, &location).return_normalised();
            Some(IntersectionData::new(location, normal, t * ray.magnitude()))
        }

        pub fn new(radius: f64, location: Vector) -> Sphere {
            Sphere { radius, location }
        }

        pub fn add_vector_to_location(&mut self, to_add: &Vector) {
            self.location = self.location.return_plus(to_add)
        }
    }
}

impl SceneObject for Sphere {
    fn get_location(&self) -> Vector {
        self.location
    }

    fn set_location(&mut self, goto: &Vector) -> () {
        self.location = *goto;
    }

    fn intersection(&self, ray: &Vector, starting_point: &Vector) -> Option<IntersectionData> {
        let det = (2.0 * (starting_point.x - self.location.x) * ray.x + 2.0 * (starting_point.y - self.location.y) * ray.y + 2.0 * (starting_point.z - self.location.z) * ray.z).powi(2)- 4.0 * (ray.x.powi(2) + ray.y.powi(2) + ray.z.powi(2)) * ((starting_point.x - self.location.x).powi(2)+ (starting_point.y - self.location.y).powi(2) + (starting_point.z - self.location.z).powi(2) - self.radius.powi(2));
        if det < 0.0 {
            return None;
        }
        let t;
        let non_det = - 2.0 * (ray.x * (starting_point.x - self.location.x) + ray.y * (starting_point.y - self.location.y) + ray.z * (starting_point.z - self.location.z));
        let denominator = 2.0 * ray.magnitude_squared();
        if non_det - det.powf(0.5) > 0.0 {
            t = (non_det - det.powf(0.5)) / denominator;
        }
        else if non_det + det.powf(0.5) > 0.0 {
            t = (non_det + det.powf(0.5)) / denominator;
        }
        else {
            return None;
        }
        let location = ray.return_multiply(t);
        let normal = Vector::vector_between(&self.location, &location).return_normalised();
        Some(IntersectionData::new(location, normal, t * ray.magnitude()))
    }

}
pub struct IntersectionData {
    location: Vector,
    normal: Vector,
    distance: f64
}
impl IntersectionData {
    pub fn new(location: Vector, normal: Vector, distance: f64) -> IntersectionData {
        IntersectionData { location, normal, distance}
    }

    pub fn location(&self) -> Vector {
        self.location.copy()
    }

    pub fn normal(&self) -> Vector {
        self.normal.copy()
    }

    pub fn distance(&self) -> f64 {
        self.distance
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Vector {
    x: f64,
    y: f64,
    z: f64
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
        println!("{}", a.determinant());
        let b = a.return_transpose();
        let c= ThreeMatrix::return_multiply(&a, &b);
        println!("{:?}", a);
        a
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

pub struct FourMatrix {

}
pub mod scene {
    use crate::{Colour, IntersectionData, SceneObject, Vector};
    use crate::scene_object::Sphere;

    pub struct Camera {
        pub direction: Vector,
        pub location: Vector
    }
    pub struct Screen {
        pub distance: f64,
        pub height: i64,
        pub width: i64
    }

    impl Screen {
        fn points_from_camera(&self, cam: &Camera) -> Vec<Vector> {
            let mut to_return = vec![];
            println!("{:?}", &cam.direction);
            let rotation_matrix = Vector::three_rotation_matrix_between(&Vector::new(0.0, 0.0, -1.0), &cam.direction);
            for y in -self.height..self.height {
                for x in -self.width..self.width {
                    let a = Vector::new(x as f64, y as f64, -self.distance);
                    to_return.push(Vector::new(x as f64, y as f64, -self.distance).return_three_matrix_mut(&rotation_matrix));
                    //to_return.push(Vector::new(x as f64, y as f64, self.distance));
                }
            }
            to_return
        }
    }
    pub struct LightSource {
        pub location: Vector,
        pub colour: Colour,
        pub intensity: u8
    }
    pub struct Contents<'a> {
        pub objects: Vec<&'a Box<dyn SceneObject>>,
        pub light: Vec<&'a LightSource>
    }
    pub fn nearest_intersection_data(content: &Contents, ray: &Vector, starting_point: &Vector) -> Option<IntersectionData> {
        let mut intersect = None;
        for object in &content.objects {
            match object.intersection(&ray, &starting_point) {
                None => {}
                Some(inter) => {
                    if let None = &intersect {
                        intersect = Some(inter);
                    }
                    else {
                        if inter.distance < intersect.as_ref().unwrap().distance {
                            intersect = Some(inter);
                        }
                    }
                }
            }
        }
        intersect
    }
    pub fn draw(cam: &Camera, screen: &Screen, content: &Contents) -> Vec<u8> {
        //TODO: add different surfaces for different objects
        let mut pixel_data = Vec::new();
        let screen_points = screen.points_from_camera(cam);
        println!("camera location {:?}", cam.location);
        for point in screen_points {
            let intersect = nearest_intersection_data(&content, &point, &cam.location);
            match intersect {
                None => {pixel_data.append(&mut vec![30, 30, 30]);}
                Some(interdata) => {
                    let mut diffuse: Colour = Colour::new(0, 0, 0);
                    for light in &content.light {
                        let distance_from_light = Vector::vector_between(&interdata.location, &light.location).magnitude();
                        let to_light = Vector::vector_between(&interdata.location, &light.location).return_normalised();
                        let nearest_intersection = nearest_intersection_data(&content, &to_light, &interdata.location().return_plus(&interdata.normal));
                        match nearest_intersection {
                            None => {
                                let mut to_add = Colour::new(light.colour.r, light.colour.g, light.colour.b);
                                to_add.multiply(Vector::dot(&to_light, &interdata.normal) * 2.0);
                                diffuse.add(&to_add);
                            }
                            Some(data) => {
                                if data.distance > distance_from_light {
                                    let mut to_add = Colour::new(light.colour.r, light.colour.g, light.colour.b);
                                    to_add.multiply(Vector::dot(&to_light, &interdata.normal) * 2.0);
                                    diffuse.add(&to_add);
                                }
                            }
                        }
                    }
                    let mut specular: Colour = Colour::new(0, 0, 0);
                    for light in &content.light {
                        let mut to_add = Colour::new(light.colour.r, light.colour.g, light.colour.b);
                        let distance_from_light = Vector::vector_between(&interdata.location, &light.location).magnitude();
                        let to_light = Vector::vector_between(&interdata.location, &light.location).return_normalised();
                        let nearest_intersection = nearest_intersection_data(&content, &to_light, &interdata.location().return_plus(&interdata.normal));
                        match nearest_intersection {
                            None => {
                                let reflected = Vector::return_reflected(&to_light, &interdata.normal);
                                to_add.multiply(Vector::dot(&reflected, &interdata.location.return_normalised()).powi(4));
                                specular.add(&to_add);
                            }
                            Some(data) => {
                                if data.distance > distance_from_light {
                                    let reflected = Vector::return_reflected(&to_light, &interdata.normal);
                                    to_add.multiply(Vector::dot(&reflected, &interdata.location.return_normalised()).powi(4));
                                    specular.add(&to_add);
                                }
                            }
                        }
                    }
                    diffuse.add(&specular);
                    pixel_data.append(&mut diffuse.get());
                }
            }
        }
        pixel_data
    }
}
#[derive(Copy, Clone, Debug)]
pub struct Colour {
    r: u8,
    g: u8,
    b: u8
}
impl Colour {

    fn add_saturating(a: u8, b: u8) -> u8 {
        if a as i32 + b as i32 > 255 {
            return 255
        }
        a + b
    }
    pub fn new(r: u8, g: u8, b: u8) -> Colour {
        Colour {
            r,
            g,
            b,
        }
    }
    pub fn get(&self) -> Vec<u8>{
        vec![self.r, self.g, self.b]
    }
    pub fn add(&mut self, c: &Colour) -> () {
        self.r = Colour::add_saturating(self.r, c.r);
        self.b = Colour::add_saturating(self.b, c.b);
        self.g = Colour::add_saturating(self.g, c.g);
    }
    pub fn multiply(&mut self, m: f64) {
        self.r = (self.r as f64 * m.abs()) as u8;
        self.b = (self.b as f64 * m.abs()) as u8;
        self.g = (self.g as f64 * m.abs()) as u8;
    }
}

// FOR BEHAVIOUR
pub trait Agent {
    fn act(&mut self, other_agents: &Vec<&dyn Agent>) -> ();
    fn get_location(&self) -> Vector;
    fn set_location(&mut self, togo: &Vector);
    fn get_body(&self) -> &Box<dyn SceneObject>;
    fn distance_from(&self, point: &Vector) -> f64;
}
pub struct BasicAgent {
    body: Box<dyn SceneObject>,
}

impl BasicAgent {
    pub fn new(body: Box<dyn SceneObject>) -> BasicAgent {
        return BasicAgent { body }
    }
}
impl Agent for BasicAgent {
    fn act(&mut self, other_agents: &Vec<&dyn Agent>) {
        let mut dl = Vector::origin();
        for other_agent in other_agents {
            let between = Vector::vector_between(&self.get_location(), &other_agent.get_location());
            if between.magnitude() < self.distance_from(&other_agent.get_location()) + other_agent.distance_from(&self.get_location()) {
                dl.minus(&between.return_normalised());
            }
            else {
                dl.plus(&between.return_normalised());
            }
        }
        self.set_location(&self.get_location().return_plus(&dl));
    }
    fn get_location(&self) -> Vector {
        self.body.get_location()
    }

    fn set_location(&mut self, togo: &Vector) {
        self.body.set_location(&togo);
    }

    fn get_body(&self) -> &Box<dyn SceneObject> {
        &self.body
    }

    fn distance_from(&self, point: &Vector) -> f64 {
        let vector_between = Vector::vector_between(&self.get_location(), &point);
        self.body.intersection(&vector_between, &self.body.get_location()).unwrap().distance
    }
}

pub struct SmartCamera {

}

impl Agent for SmartCamera {
    fn act(&mut self, other_agents: &Vec<&dyn Agent>) -> () {
        todo!()
    }

    fn get_location(&self) -> Vector {
        todo!()
    }

    fn set_location(&mut self, togo: &Vector) {
        todo!()
    }

    fn get_body(&self) -> &Box<dyn SceneObject> {
        todo!()
    }

    fn distance_from(&self, point: &Vector) -> f64 {
        todo!()
    }
}


