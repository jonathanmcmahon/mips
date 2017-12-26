/// A MIPS processor.
///
/// # Examples
///
/// ```
/// mips instructions.txt
/// ```

mod mux;
mod register;
mod register_file;

use mux::Mux;
use register::Register;

fn main() {
    let instruction1 = "000000-00111-01000-01100-00000-010100";
    let mut m = Mux::new(2, 4);
    m.set_input(0, "1010".to_string());
    println!("Outer read of switch: {}", m.switch);
    m.set_switch(1);
    println!("Outer read of switch: {}", m.switch);
    println!("Outer read of input0: {}", m.inputs[0]);
    println!("{}", instruction1);
    println!("Buzz");
}
