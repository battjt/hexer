use fltk::{
    app::App,
    button::Button,
    enums::Color,
    group::Pack,
    input::Input,
    prelude::{GroupExt, InputExt, WidgetExt},
    window::Window,
};
#[derive(Clone)]
struct Field {
    input: Input,
    radix: u32,
}
fn main() {
    let radixes = [("bin", 2), ("dec", 10), ("hex", 16), ("7", 7), ("17", 17)];

    let application = App::default();
    let height = 20 * (radixes.len() + 1) as i32;
    let mut window = Window::default()
        .with_label("Hexer")
        .with_size(200, height + 20);
    let pack = Pack::default().with_size(100, height).center_of_parent();
    let fields = radixes.map(|d| Field {
        input: Input::default().with_label(d.0).with_size(100, 20),
        radix: d.1,
    });
    Button::default()
        .with_label("Close")
        .with_size(60, 20)
        .set_callback(move |_| application.quit());
    pack.end();
    window.make_resizable(true);
    window.end();
    window.show();

    for mut e in fields.clone() {
        let default_color = e.input.color();
        e.input.set_callback({
            let fields = fields.clone();
            move |input: &mut Input| {
                if let Some(value) = u64::from_str_radix(input.value().as_str(), e.radix).ok() {
                    for field in fields.clone() {
                        let mut field = field.clone();
                        field.input.set_color(default_color);
                        field.input.set_value(&match field.radix {
                            10 => format!("{}", value),
                            16 => format!("{:X}", value),
                            2 => format!("{:b}", value),
                            _ => "input only".to_string(),
                        })
                    }
                } else {
                    input.set_color(Color::Yellow);
                    input.redraw();
                };
            }
        })
    }

    application.run();
}
