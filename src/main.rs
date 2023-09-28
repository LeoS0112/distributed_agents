use std::process::Command;
use show_image::{ImageView, ImageInfo, create_window};
use summer2023::{Agent, Vector, scene_object::Sphere, Colour, BasicAgent};
use summer2023::scene::{Camera, Contents, draw, LightSource, Screen};

#[show_image::main]
fn main() {
    let mut test_sphere = Sphere { radius: 150.0, location: Vector::new(0.0, 0.0, 1200.0) };
    let mut test_sphere2 = Sphere { radius: 120.0, location: Vector::new(150.0, 150.0, 1400.0) };
    let mut test_sphere3 = Sphere { radius: 10.0, location: Vector::new(-130.0, -30.0, 1020.0) };
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
    let mut a = BasicAgent::new(Box::new(test_sphere));
    let mut b = BasicAgent::new(Box::new(test_sphere2));
    let mut c = BasicAgent::new(Box::new(test_sphere3));
    let agents: Vec<&dyn Agent> = vec![&a, &b, &c];
    let window = create_window("image", Default::default()).expect("Should work");
    let mut to_show = Vec::new();
    for n in 1..100 {
        println!("{}", n);
        a.act(&vec![&b, &c]);
        b.act(&vec![&a, &c]);
        c.act(&vec![&a, &b]);
        let contents = Contents{ objects: vec![a.get_body(), b.get_body(), c.get_body()], light: vec![&test_light, &other_test_light] };
        let pixel_data = draw(&test_cam, &test_screen, &contents);
        to_show.push(pixel_data);

    }
    loop {
        for pd in 0..99{
            let image = ImageView::new(ImageInfo::rgb8((2 * test_screen.width) as u32, (2 * test_screen.height) as u32), &to_show[pd]);
            window.set_image("image-001", image).expect("set image");
            let mut child = Command::new("sleep").arg("0.02").spawn().unwrap();
            let _result = child.wait().unwrap();
        }
    }
}

