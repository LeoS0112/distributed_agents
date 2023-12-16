use std::process::Command;
use std::sync::mpsc::{Sender, Receiver, channel};
use std::sync::{Arc, Mutex};
use show_image::{ImageView, ImageInfo, create_window};
use show_image::event::VirtualKeyCode::Mute;
use summer2023::matrices::{Vector};
use summer2023::raytracer::scene::{Camera, Contents, draw, LightSource, Screen};
use summer2023::raytracer::{Colour, scene_objects::Sphere, SceneObject};
use summer2023::agents::{Agent, BasicAgent};

#[show_image::main]
fn main() {
    // Creating the test spheres
    let mut test_sphere = Sphere { radius: 150.0, location: Vector::new(0.0, 0.0, 1200.0) };
    let mut test_sphere2 = Sphere { radius: 120.0, location: Vector::new(150.0, 150.0, 1400.0) };
    let mut test_sphere3 = Sphere { radius: 100.0, location: Vector::new(-100.0, -20.0, 1000.0) };

    // Creating the test Camera, Screen and Light Sources
    let test_cam = Camera {
        direction: Vector::new(0.0, 0.0, 1.0),
        location: Vector::new(0.0, 0.0, 0.0)
    };
    let test_screen = Screen {
        distance: 500.0,
        height: 100,
        width: 100,
    };
    let test_light = LightSource {
        location: Vector::new(-1000.0, 300.0, 10.0),
        colour: Colour::new(100, 0, 0),
        intensity: 19,
    };
    let other_test_light = LightSource {
        location: Vector::new(300.0, 0.0,0.0),
        colour: Colour::new(0, 100, 0),
        intensity: 0,
    };
    let (sa, ra) = channel();
    let (sb, rb) = channel();
    let (sc, rc) = channel();

    let mut s_channels: Arc<Vec<Mutex<Sender<String>>>> = Arc::new(vec![Mutex::new(sa.clone()),Mutex::new(sb.clone()), Mutex::new(sc.clone())]);
    // Creating the agents
    let mut a = Arc::new(Mutex::new(BasicAgent::new(0, Arc::new(Mutex::new(Box::new(test_sphere))), s_channels.clone(), Mutex::new(ra))));
    let mut b = Arc::new(Mutex::new(BasicAgent::new(1, Arc::new(Mutex::new(Box::new(test_sphere2))), s_channels.clone(), Mutex::new(rb))));
    let mut c = Arc::new(Mutex::new(BasicAgent::new(2, Arc::new(Mutex::new(Box::new(test_sphere3))), s_channels.clone(), Mutex::new(rc))));


    // Running the simulation
    let window = create_window("image", Default::default()).expect("Should work");
    let mut to_show = Vec::new();
    let mut agents: Vec<Arc<Mutex<BasicAgent>>> = vec![a.clone(), b.clone(), c.clone()];

    for _ in 0..100 {
        let mut handles = vec![];
        let num_agents = agents.len();
        for agent_num in 0..num_agents {
            handles.push(agents[agent_num].lock().unwrap().act(agents[agent_num].clone()));
        }
        for handle in handles {
            handle.join().unwrap();
        }

        let mut scene_objects = vec![];
        for agent_num in 0..num_agents {
            let current_agent = agents[agent_num].lock().unwrap();
            scene_objects.push(current_agent.get_body());
        }
        let contents = Contents { objects: scene_objects, light: vec![&test_light, &other_test_light] };
        let pixel_data = draw(&test_cam, &test_screen, &contents);
        to_show.push(pixel_data);
    }

    // Playing the simulation in a loop
    loop {
        for pd in 0..100{
            let image = ImageView::new(ImageInfo::rgb8((2 * test_screen.width) as u32, (2 * test_screen.height) as u32), &to_show[pd]);
            window.set_image("image-001", image).expect("set image");
            let mut child = Command::new("sleep").arg("0.02").spawn().unwrap();
            let _result = child.wait().unwrap();
        }
    }
}

