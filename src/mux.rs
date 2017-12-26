/// A multiplexer with two inputs.
///
/// # Examples
///
/// ```
/// let m = Mux { input0: '0000', input1: '1110', '0', 4};
/// ```

pub struct Mux {
    pub inputs: Vec<String>,
    pub n_inputs: usize,
    pub switch: usize,
    pub bitwidth: usize,
}

impl Mux {
    pub fn new(n_inputs: usize, bitwidth: usize) -> Mux {
        let mut m = Mux { bitwidth, n_inputs, switch: 0, inputs : vec!["".to_string(); n_inputs] };
        for i in 0..n_inputs {
            m.set_input(i, "0".repeat(bitwidth))
        }
        m
    }

    pub fn set_input(&mut self, input: usize, bits: String) {
        if self.get_bitwidth() == bits.to_string().chars().count() { 
            self.inputs[input] = bits.to_string()
        }
        else {
            panic!("Input should be {} bits!", self.get_bitwidth())
        }
        println!("self.input[{}]: {}", input, self.inputs[input])
    }

    pub fn set_switch(&mut self, v: usize) {
        if 0 <= v && v < self.n_inputs {
            self.switch = v
        }
        else {
            panic!("Invalid switch selection.")
        }
    }

    fn get_bitwidth(&self) -> usize {
        self.bitwidth
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input0_holds_correct_value() {
        let m = Mux::new(2, 4);
        assert!(m.inputs[0].contains("0000"));
    }
}