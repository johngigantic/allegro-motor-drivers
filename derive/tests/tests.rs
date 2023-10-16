use allegro_motor_derive::AllegroRegister;
use bilge::prelude::*;

#[test]
fn test_proc_macro() {
    #[derive(AllegroRegister)]
    #[bitsize(12)]
    struct MyStruct {
        field_1: u12,
    }
}
