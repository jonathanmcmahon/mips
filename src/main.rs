/// A MIPS processor.
///
/// # Examples
///
/// ```
/// mips instructions.txt
/// ```

mod mux;
mod register;

use mux::Mux;
use register::Register;

fn main() {
    let instruction1 = "000000-00111-01000-01100-00000-010100";
    let mut m = Mux { input0: "0000", input1: "1110", switch: '0', bitwidth: 4};
    m.set_input0("1010");
    println!("Outer read of switch: {}", m.switch);
    m.set_switch('1');
    println!("Outer read of switch: {}", m.switch);
    println!("Outer read of input0: {}", m.input0);
    println!("{}", instruction1);
    println!("Buzz");
}
