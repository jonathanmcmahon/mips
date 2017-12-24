/// A multiplexer with two inputs.
///
/// # Examples
///
/// ```
/// let m = Mux { input0: '0000', input1: '1110', '0', 4};
/// ```

pub struct Mux<'a> {
    pub input0: &'a str,
    pub input1: &'a str,
    pub switch: char,
    pub bitwidth: usize,
}

impl<'a> Mux<'a> {
    pub fn set_input0(&mut self, input: &'a str) {
        let s = String::from(input);
        if self.get_bitwidth() == s.to_string().chars().count() { 
            self.input0 = input
        }
        else {
            panic!("Fiddlesticks!")
        }
        println!("self.input0: {}", self.input0)
    }

    fn set_input1(&mut self, input: &'a str) {
        let s = String::from(input);
        if self.get_bitwidth() == s.to_string().chars().count() { 
            self.input1 = input
        }
        else {
            panic!("Fiddlesticks!")
        }
        println!("self.input1: {}", self.input0)
    }

    pub fn set_switch(&mut self, v: char) {
        if v == '0' || v == '1' {
            self.switch = v
        }
        else {
            panic!("You did a bad.")
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
        let m = Mux { input0: "0000", input1: "1110", switch: '0', bitwidth: 4};
        assert!(m.input0.contains("0000"));
    }
}