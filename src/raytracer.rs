use crate::matrices::Vector;
pub trait SceneObject: Send {
    fn get_location(&self) -> Vector;
    fn set_location(&mut self, goto: &Vector) -> ();
    fn intersection(&self, ray: &Vector, starting_point: &Vector) -> Option<IntersectionData>;
}
pub mod scene_objects {
    use crate::matrices::Vector;
    use crate::raytracer::{IntersectionData, SceneObject};

    pub struct Sphere {
        pub radius: f64,
        pub location: Vector
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
}
pub struct IntersectionData {
    location: Vector,
    normal: Vector,
    pub(crate) distance: f64
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
pub mod scene {
    use std::sync::{Arc, Mutex};
    use crate::matrices::Vector;
    use crate::raytracer::{Colour, IntersectionData, SceneObject};

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
            let rotation_matrix = Vector::three_rotation_matrix_between(&Vector::new(0.0, 0.0, -1.0), &cam.direction);
            for y in -self.height..self.height {
                for x in -self.width..self.width {
                    //let a = Vector::new(x as f64, y as f64, -self.distance);
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
        pub objects: Vec<Arc<Mutex<Box<dyn SceneObject + Send + Sync>>>>,
        pub light: Vec<&'a LightSource>
    }
    pub fn nearest_intersection_data(content: &Contents, ray: &Vector, starting_point: &Vector) -> Option<IntersectionData> {
        let mut intersect = None;
        for object in &content.objects {
            match object.lock().unwrap().intersection(&ray, &starting_point) {
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
