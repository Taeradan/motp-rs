/// tools to interact with mOTP (mobile One-Time Passwords: basically TOTP, but with an added pin
/// code) in rust
extern crate chrono;
extern crate md5;
use chrono::{DateTime, Utc};

/// A commonly used period for OTP checking
pub const DEFAULT_OTP_VALID_PERIOD: u8 = 30; //in seconds

#[cfg(test)]
mod tests {
    use super::{check_otp, create_otp};
    #[test]
    fn it_really_works() {
        let otp = create_otp("0123456789abcdef", "0000");
        assert!(check_otp(&otp, "0123456789abcdef", "0000"));
    }
}

/// Creates a 6-character OTP
/// ```rust
/// let otp: String = create_otp("0123456789abcdef", "6666");
/// println!("{}", otp); // fb96b4
/// ```
pub fn create_otp(secret: &str, pin: &str) -> String {
    let now: DateTime<Utc> = Utc::now();
    let to_hash = format!("{}{}{}", now.timestamp() / 10, secret, pin);
    let hash = format!("{:x}", md5::compute(to_hash));
    let s = String::from(&hash[0..6]);
    s
}

/// Verifies a give otp code according to shared secrets (secret & pin)
/// - `code`: the user input
/// - `secret`: the shared secret
/// - `pin`: the pin, as supposedly inputed by the user on his client
/// - `valid_period`: the length of validity of the token. token will be checked by this half this
///                   value in the past & future
pub fn check_otp(code: &str, secret: &str, pin: &str, valid_period: u8) -> bool {
    let now = Utc::now().timestamp();
    let mut past_limit = now - valid_period as i64 / 2;
    let future_limit = now + valid_period as i64 / 2;
    while past_limit < future_limit {
        let current_code = create_otp(secret, pin);
        if current_code == code {
            return true;
        }
        past_limit = past_limit + 1; //could reduce iteration by using 10 instead of 1, i suppose...
    }
    false
}
