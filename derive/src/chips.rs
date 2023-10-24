//! Defines chip names available for the derive macro and their properties.

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Chip {
    pub name: &'static str,
    pub address_size: u16,
    pub register_size: u16,
    pub parity: bool,
}

pub const CHIPS: [Chip; 8] = [
    Chip {
        name: "a4910",
        address_size: 2,
        register_size: 13,
        parity: false,
    },
    Chip {
        name: "a4962",
        address_size: 3,
        register_size: 12,
        parity: false,
    },
    Chip {
        name: "a4963",
        address_size: 3,
        register_size: 12,
        parity: false,
    },
    Chip {
        name: "a4964",
        address_size: 5,
        register_size: 9,
        parity: true,
    },
    Chip {
        name: "amt49100",
        address_size: 5,
        register_size: 9,
        parity: true,
    },
    Chip {
        name: "amt49100",
        address_size: 5,
        register_size: 9,
        parity: true,
    },
    Chip {
        name: "amt49100",
        address_size: 5,
        register_size: 9,
        parity: true,
    },
    Chip {
        name: "amt49100",
        address_size: 5,
        register_size: 9,
        parity: true,
    },
];
