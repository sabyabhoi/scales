use scales::{Key, Scale};

fn main() {
    let s = Scale {
        scale_type: scales::ScaleType::Pentatonic,
        key: Key::Major(String::from("C")),
    };
    s.display();
}
