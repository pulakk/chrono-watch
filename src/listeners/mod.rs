use std::io;

pub fn listen_stdin_for_floats<H: FnMut (f32)>(mut handler:H) {
    loop {
        let mut new_value_serialized = String::new();
        let len = io::stdin().read_line(&mut new_value_serialized).expect("Erorr in readine line");
        if len == 0 {
            return;
        }

        handler(new_value_serialized.trim().parse::<f32>().expect("Non floating point number found in stdin stream"));
    }
}