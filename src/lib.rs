mod country_code;
mod email;
pub mod error;
mod password;
mod phone;

pub use country_code::CountryCode;
pub use email::Email;
pub use password::{HashedPassword, RawPassword};
pub use phone::PhoneNumber;

/// ======================================================================
/// ========================= Unit Test
/// ======================================================================
#[cfg(test)]
mod test {

    // TODO! Create Integration Test
}
