#[feature(macro_rules)];
#[crate_id = "codegen#0.1"];

use std::os;
use std::io::BufferedWriter;
use std::io::File;
use std::io;
use std::io::stdio::println;
use std::io::fs::mkdir_recursive;
use std::path::GenericPath;
use std::vec_ng::Vec;
pub mod branchify;
pub mod keycode;
pub mod scancode;

fn main() {
    let args = Vec::from_slice(os::args());
    match args.len() {
        0 => {
            println("usage: codegen [keycode|scancode].rs destdir");
            os::set_exit_status(1);
        },
        3 => {
            let output_dir = GenericPath::new(args.get(2).clone());
            // TODO: maybe not 0777?
            mkdir_recursive(&output_dir, 0b111_111_111);

            if *args.get(1) == ~"keycode.rs" {
                keycode::generate(&output_dir);
            } else if *args.get(1) == ~"scancode.rs" {
                scancode::generate(&output_dir);
            } else {
                println!("unknown thing-to-generate '{}'", args.get(1));
                os::set_exit_status(1);
            }
        },
        _ => {
            println!("usage: {} [keycode|scancode].rs destdir", args.get(0));
            os::set_exit_status(1);
        }
    }
}

pub fn get_writer(output_dir: &Path, filename: &str) -> ~BufferedWriter<File> {
    match File::open_mode(&output_dir.join(filename), io::Truncate, io::Write) {
        Ok(writer) => ~BufferedWriter::new(writer),
        Err(e) => fail!("Unable to write file: {:s}", e.desc),
    }
}
