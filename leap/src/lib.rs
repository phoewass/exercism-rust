trait IsDiv {
    fn is_div(&self, n: u16) -> bool;
}

impl IsDiv for u16 {
    fn is_div(&self, n: u16) -> bool {
        self % n == 0
    }
}

pub fn is_leap_year(y: u16) -> bool {
     y.is_div(4) && !y.is_div(100) || y.is_div(400)
}
