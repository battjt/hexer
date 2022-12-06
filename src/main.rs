use std::sync::{Arc, Mutex};

use fltk::{
    app::App,
    frame::Frame,
    input::Input,
    prelude::{GroupExt, InputExt, WidgetExt},
    window::Window,
};

fn main() {
    let application = App::default();
    let mut window = Window::default().with_label("Hexer").with_size(200, 100);
    let frame = Frame::default().with_size(100, 20).center_x(&window);
    let  dec = Input::default()
        .with_label("dec")
        .size_of(&frame)
        .below_of(&frame, 0);
    let  hex = Input::default()
        .with_label("hex")
        .size_of(&frame)
        .below_of(&dec, 0);
    let  bin = Input::default()
        .with_label("bin")
        .size_of(&frame)
        .below_of(&hex, 0);
    window.make_resizable(true);
    window.end();
    window.show();

    let d = Arc::new(Mutex::new(dec));
    let h = Arc::new(Mutex::new(hex));
    let b = Arc::new(Mutex::new(bin));

    d.lock()
        .unwrap()
        .set_callback(create_cb(|i| i.value().parse().unwrap(), &d, &h, &b));

    h.lock().unwrap().set_callback(create_cb(
        |i| u64::from_str_radix(i.value().as_str(), 16).unwrap(),
        &d,
        &h,
        &b,
    ));

    application.run();
}

fn create_cb(
    var_name: fn(&Input) -> u64,
    d: &Arc<Mutex<Input>>,
    h: &Arc<Mutex<Input>>,
    b: &Arc<Mutex<Input>>,
) -> impl FnMut(&mut Input) {
    let d = d.clone();
    let h = h.clone();
    let b = b.clone();
    move |e: &mut Input| {
        let n = (var_name)(e);
        update(n, &d, &h, &b);
    }
}

fn update(n: u64, d: &Arc<Mutex<Input>>, h: &Arc<Mutex<Input>>, b: &Arc<Mutex<Input>>) {
    d.lock().unwrap().set_value(n.to_string().as_str());
    h.lock().unwrap().set_value(&format!("{:X}", n));
    b.lock().unwrap().set_value(&format!("{:b}", n));
}
