mod country_code;
mod email;
pub mod error;
mod password;
mod phone;

pub use country_code::CountryCode;
pub use email::Email;
pub use password::RawPassword;
pub use phone::PhoneNumber;

/// ======================================================================
/// ========================= Unit Test
/// ======================================================================
#[cfg(test)]
mod test {

    use crate::{error::TypeError, Email, RawPassword};

    use serde::Serialize;
    #[derive(Debug, Serialize)]
    struct User {
        email: Email,
        password: RawPassword,
    }

    impl User {
        fn new(email: String, password: String) -> Result<Self, TypeError> {
            let email = Email::parse(email)?;
            let password = RawPassword::parse_weak(password)?;

            Ok(Self { email, password })
        }
    }

    #[test]
    fn test_user() {
        let user = User::new(
            "77nurfianqodar@gmail.com".to_string(),
            "password123".to_string(),
        );

        match user {
            Ok(user) => println!("{:?}", serde_json::to_string(&user)),
            Err(e) => println!("{:?}", e),
        }
    }
}
