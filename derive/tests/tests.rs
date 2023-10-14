#![cfg(test)]


use allegro_motor_derive::AllegroRegister;
use bilge::prelude::*;

#[test]
fn test_proc_macro() {
    #[bitsize(12)]
    #[derive(AllegroRegister)]
    struct MyStruct {
        field_1: u12,
    }
}