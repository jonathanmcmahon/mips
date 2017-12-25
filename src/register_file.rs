/// A register file with 32 x 32-bit registers.
///
/// # Examples
///
/// ```
/// let mut rf = RegisterFile::new();
/// rf.registers[31].set_bits("00000000000000000000000011111111");
/// ```

use register::Register;

pub struct RegisterFile<'a> {
    registers: Vec<Register<'a>>,
}

impl<'a> RegisterFile<'a> {
    pub fn new() -> RegisterFile<'a> {
        RegisterFile { registers : vec![Register { bits: "00000000000000000000000000000000", bitwidth: 32 }; 32] }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn registerfile_read() {
        let mut rf = RegisterFile::new();
        println!("{}", rf.registers[0]);
        assert!(rf.registers[0].bits == "00000000000000000000000000000000");
    }

    #[test]
    fn registerfile_writes() {
        let mut rf = RegisterFile::new();
        
        assert!(rf.registers[0].bits == "00000000000000000000000000000000");
        rf.registers[0].set_bits("01010101010101010101010101010101");
        assert!(rf.registers[0].bits == "01010101010101010101010101010101");
        
        assert!(rf.registers[31].bits == "00000000000000000000000000000000");
        rf.registers[31].set_bits("01010101010101010101010101010101");
        assert!(rf.registers[31].bits == "01010101010101010101010101010101");
    }

}