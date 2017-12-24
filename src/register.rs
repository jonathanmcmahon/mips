/// A fixed-width register.
///
/// # Examples
///
/// ```
/// let mut r = Register { bits: "0000", bitwidth: 4 };
/// ```

pub struct Register<'a> {
    bits: &'a str,
    bitwidth: usize,
}

impl<'a> Register<'a> {

    pub fn set_bits(&mut self, value: &'a str) {
        self.bits = value
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

}