pub struct Adder{
    a: i32,
    b:i32,
    total: i32,
}

impl Adder {
    pub fn new() -> Self {
        Adder { 
            a: 0,
            b: 0,
            total: 0,
        }
    }

    pub fn add(&mut self, a: i32, b: i32) {
        self.a = a;
        self.b = b;
        self.total += a + b;
    }

    pub fn result(&self) -> i32 {
        self.total
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adder() {
        let mut adder = Adder::new();
        adder.add(5,10);

        let result = adder.result();
        assert_eq!(result, adder.a + adder.b );
        
    }
}
