//! Error types

#[allow(clippy::module_name_repetitions)]
pub enum AllegroError {
    SpiError(embedded_hal::spi::ErrorKind),
    InvalidParity,
    InvalidRegister,
    MotorFault,
}

impl From<embedded_hal::spi::ErrorKind> for AllegroError {
    fn from(error: embedded_hal::spi::ErrorKind) -> Self {
        AllegroError::SpiError(error)
    }
}
