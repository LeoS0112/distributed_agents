use std::sync::{Arc, Mutex};
use std::sync::mpsc::{Receiver, Sender};
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
pub struct BasicAgent<> {
    id: i64,
    body: Arc<Mutex<Box<dyn SceneObject + Send + Sync>>>,
    senders: Arc<Vec<Mutex<Sender<String>>>>, //each BasicAgent will have a sender channel for each other agent
    receiver: Mutex<Receiver<String>>,
}
impl BasicAgent<> {
    pub fn new<>(id: i64, body: Arc<Mutex<Box<dyn SceneObject + Send + Sync>>>, sender: Arc<Vec<Mutex<Sender<String>>>>, receiver: Mutex<Receiver<String>>) -> BasicAgent {
        return BasicAgent { id, body, senders: sender, receiver }
    }
}
impl Agent for BasicAgent<> {

    //act(_) will ask the other vectors where they are and go towards them
    fn act(&self, mut slf: Arc<Mutex<BasicAgent>>) -> JoinHandle<()>{
        let h = thread::spawn(move || {
            let to_change = Vector::origin();
            let mut slf_unlocked = slf.lock().unwrap();
            //let new_location = &slf_unlocked.get_location().return_plus(&to_change);
            //slf_unlocked.set_location(new_location);
            let location = slf_unlocked.get_location();
            let to_send = String::from(format!("{} {} {} {}", slf_unlocked.id, location.x, location.y, location.z));
            let num_senders = slf_unlocked.senders.len();
            for i in 0..num_senders{
                slf_unlocked.senders[i].lock().unwrap().send(to_send.clone()).unwrap();
            }
            for i in 0..num_senders {
                let received =  slf_unlocked.receiver.lock().unwrap().recv().unwrap();
                let info: Vec<&str> = received.split(' ').collect();
                let to_add = Vector::vector_between(&slf_unlocked.get_location(), &Vector::new(info[1].parse::<f64>().unwrap(), info[2].parse::<f64>().unwrap(), info[3].parse::<f64>().unwrap())).return_multiply(0.01);
                let new_location = &slf_unlocked.get_location().return_plus(&to_add);
                slf_unlocked.set_location(new_location);
                //println!("{}", info[0])
            }
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
