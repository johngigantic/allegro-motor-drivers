//! Generic I/O functions

pub(crate) trait Parity {
    fn set_odd_parity(&mut self);
}

pub(crate) fn parity(value: u16) -> bool {
    let mut x = value;
    x ^= x >> 8;
    x ^= x >> 4;
    x ^= x >> 2;
    x ^= x >> 1;
    (x & 1) != 0
}
