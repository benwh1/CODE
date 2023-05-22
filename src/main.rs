use std::{fs::File, io::Read};

use code::{interpreter::interpreter::InterpreterState, parser::program::program};

fn main() {
    let Some(file_name) = std::env::args().nth(1) else { return };
    let source_code = {
        let mut string = String::new();
        File::open(file_name)
            .expect("File not found")
            .read_to_string(&mut string)
            .expect("Failed to read file");
        string
    };
    let program = program(&source_code);
    let mut interpreter = InterpreterState::new();
    interpreter.run(program);
}
