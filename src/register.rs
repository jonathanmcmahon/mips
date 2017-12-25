/// A fixed-width register.
///
/// # Examples
///
/// ```
/// let mut r = Register { bits: "0000", bitwidth: 4 };
/// ```

use std::fmt;

#[derive(Clone)]
pub struct Register<'a> {
    pub bits: &'a str,
    pub bitwidth: usize,
}

impl<'a> Register<'a> {

    pub fn set_bits(&mut self, value: &'a str) {
        let s = String::from(value);
        let len = s.to_string().chars().count();
        if self.bitwidth == len { 
            self.bits = value
        }
        else {
            panic!("Register takes {} bits, and you tried to store {}.", self.bitwidth, len)
        }
    }
}

impl<'a> fmt::Display for Register<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.bits)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn register_creation() {
        let mut r = Register { bits: "0000", bitwidth: 4 };
        assert!(r.bits.contains("0000"));
    }

    #[test]
    fn register_set_valid_value() {
        let mut r = Register { bits: "0000", bitwidth: 4 };
        assert!(r.bits.contains("0000"));
        r.set_bits("0001");
        assert!(r.bits.contains("0001"));
    }

    #[test]
    #[should_panic]
    fn register_overflow() {
        let mut r = Register { bits: "0000", bitwidth: 4 };
        r.set_bits("01111");
    }

    #[test]
    #[should_panic]
    fn register_underflow() {
        let mut r = Register { bits: "0000", bitwidth: 4 };
        r.set_bits("111");
    }


}