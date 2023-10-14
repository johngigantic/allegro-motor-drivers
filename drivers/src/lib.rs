//! Embedded systems drivers for Allegro Microsystems motor controller integrated circuits

#![no_std]

pub mod a49xx;
pub mod amt49xxx;

pub(crate) use bilge::prelude::*;


pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
