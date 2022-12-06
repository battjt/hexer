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
    let dec = Input::default()
        .with_label("dec")
        .size_of(&frame)
        .below_of(&frame, 0);
    let hex = Input::default()
        .with_label("hex")
        .size_of(&frame)
        .below_of(&dec, 0);
    let bin = Input::default()
        .with_label("bin")
        .size_of(&frame)
        .below_of(&hex, 0);
    window.make_resizable(true);
    window.end();
    window.show();

    // pass fields as tuple to reduce number of locks (and subsequent copy and paste)
    let fields = Arc::new(Mutex::new((dec, hex, bin)));

    let (mut dec, mut hex, mut bin) = fields.clone().lock().unwrap().to_owned();
    dec.set_callback(create_cb(|i| i.value().parse().unwrap(), &fields));
    hex.set_callback(create_cb(
        |i| u64::from_str_radix(i.value().as_str(), 16).unwrap(),
        &fields,
    ));
    bin.set_callback(create_cb(|i| i.value().parse().unwrap(), &fields));

    application.run();
}

fn create_cb(
    parse_fn: fn(&Input) -> u64,
    fields: &Arc<Mutex<(Input, Input, Input)>>,
) -> impl FnMut(&mut Input) {
    let fields = fields.clone();
    move |input: &mut Input| {
        update(parse_fn(input), &fields);
    }
}

fn update(value: u64, fields: &Arc<Mutex<(Input, Input, Input)>>) {
    let (mut dec, mut hex, mut bin) = fields.lock().unwrap().to_owned();
    dec.set_value(value.to_string().as_str());
    hex.set_value(&format!("{:X}", value));
    bin.set_value(&format!("{:b}", value));
}
