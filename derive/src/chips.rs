//! Defines chip names available for the derive macro and their properties.

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Chip {
    pub name: &'static str,
    pub register_size: u16,
}

pub const CHIPS: [Chip; 8] = [
    Chip {
        name: "a4910",
        register_size: 13,
    },
    Chip {
        name: "a4962",
        register_size: 12,
    },
    Chip {
        name: "a4963",
        register_size: 12,
    },
    Chip {
        name: "a4964",
        register_size: 9,
    },
    Chip {
        name: "amt49100",
        register_size: 9,
    },
    Chip {
        name: "amt49100",
        register_size: 9,
    },
    Chip {
        name: "amt49100",
        register_size: 9,
    },
    Chip {
        name: "amt49100",
        register_size: 9,
    },
];
