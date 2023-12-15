use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;
use crate::raytracer::SceneObject;
use crate::matrices::Vector;

pub trait Agent {
    fn act(&self, slf: Arc<Mutex<BasicAgent>>) -> JoinHandle<()>;
    fn get_location(&self) -> Vector;
    fn set_location(&mut self, togo: &Vector);
    fn get_body(&self) -> Arc<Mutex<Box<dyn SceneObject + Send + Sync>>>;
    fn distance_from(&self, point: &Vector) -> f64;
}
pub struct BasicAgent {
    body: Arc<Mutex<Box<dyn SceneObject + Send + Sync>>>,
}

impl BasicAgent {
    pub fn new(body: Arc<Mutex<Box<dyn SceneObject + Send + Sync>>>) -> BasicAgent {
        return BasicAgent { body }
    }
}
impl Agent for BasicAgent {
    fn act(&self, mut slf: Arc<Mutex<BasicAgent>>) -> JoinHandle<()>{
        let h = thread::spawn(move || {
                let to_change = Vector::new(1.0, 1.0, 1.0);
                let mut s = slf.lock().unwrap();
                let new_location = &s.get_location().return_plus(&to_change);
                s.set_location(new_location);

        });
        return h;
    }

    fn get_location(&self) -> Vector {
        self.body.lock().unwrap().get_location()
    }

    fn set_location(&mut self, togo: &Vector) {
        self.body.lock().unwrap().set_location(&togo);
    }

    fn get_body(&self) -> Arc<Mutex<Box<dyn SceneObject + Send + Sync>>> {
        (&self).body.clone()
    }

    fn distance_from(&self, point: &Vector) -> f64 {
        let vector_between = Vector::vector_between(&self.get_location(), &point);
        let body = self.body.lock().unwrap();
        body.intersection(&vector_between, &body.get_location()).unwrap().distance
    }
}
