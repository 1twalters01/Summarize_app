use chrono::{DateTime, Utc};
use data_encoding::BASE32;
use hmac::{Hmac, Mac};
use rand::Rng;
use serde::{Deserialize, Serialize};
use sha1::Sha1;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Totp {
    pub activated: bool,
    pub verified: bool,
    pub verified_at: Option<DateTime<Utc>>,
    pub fields: Option<TotpFields>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TotpFields {
    pub key: String,
    pub last_updated: DateTime<Utc>,
}

impl Totp {
    pub fn from_all(
        activated: bool,
        verified: bool,
        verified_at: Option<DateTime<Utc>>,
        fields: Option<TotpFields>,
    ) -> Self {
        return Totp {
            activated,
            verified,
            verified_at,
            fields,
        };
    }

    pub fn new() -> Self {
        let mut rng = rand::thread_rng();

        let key_length: usize = rng.gen_range(21..=30);

        let charset: &str = "abcdefghijklmnopqrstuvwxyz0123456789!@#$^&*(-_=+)";
        let chars: Vec<char> = charset.chars().collect();
        let totp_url: String = (0..key_length)
            .map(|_| chars[rng.gen_range(0..chars.len())])
            .collect();

        let fields = Some(TotpFields {
            key: totp_url,
            last_updated: Utc::now(),
        });

        Totp {
            activated: false,
            verified: false,
            verified_at: None,
            fields,
        }
    }

    pub fn from_key(url: String) -> Totp {
        let mut totp = Totp {
            activated: false,
            verified: false,
            verified_at: None,
            fields: None,
        };
        totp.set_url(url);
        return totp;
    }

    pub fn generate_totp(&mut self) -> String {
        let step_in_seconds = 30;
        let length_of_otp = 6;

        // Get Totp time step and convert it to byte array
        let epoch_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let t = epoch_time / step_in_seconds;
        let t_bytes = t.to_be_bytes();
        println!("t_bytes: {:?}", t_bytes);

        // Get the key
        let binding = self.fields.clone().unwrap();
        let key: &[u8] = binding.key.as_bytes();
        println!("key: {:?}", std::str::from_utf8(key));

        // Hmac object
        let mut mac = Hmac::<Sha1>::new_from_slice(key).unwrap();
        mac.update(&t_bytes);
        let hmac_bytes = mac.finalize().into_bytes();
        println!(
            "hmac object: {:02X?}",
            hmac_bytes
                .iter()
                .map(|x| format!("{:02x}", x))
                .collect::<String>()
        );

        // Convert last nibble to an offset
        let offset = (hmac_bytes[hmac_bytes.len() - 1] & 0xf) as usize;
        println!("offset: {:#?}", offset);

        // Extract the 4-byte binary code starting at the offset
        let binary = ((hmac_bytes[offset] as u32 & 0x7f) << 24)
            | ((hmac_bytes[offset + 1] as u32) << 16)
            | ((hmac_bytes[offset + 2] as u32) << 8)
            | (hmac_bytes[offset + 3] as u32);
        println!("binary: {:#?}", binary);

        let totp = format!("{:01$}", binary, length_of_otp);
        totp[totp.len() - length_of_otp..].to_string()
    }

    pub fn generate_qr_string(
        &mut self,
        // email: &str,
    ) -> String {
        let email = env::var("SUMMARIZE_EMAIL".to_string())
        let length_of_otp = 6;
        let step_in_seconds = 30;
        let site = "Summarize";
        let binding = self.fields.clone().unwrap();
        let key = binding.key;
        let token = BASE32.encode(&key.as_bytes());

        let qr_string = format!(
            "otpauth://totp/{site}:{email}?secret={token}&issuer={site}&algorithm=SHA1&digits={length_of_otp}&counter=0&period={step_in_seconds}",
            site=site,
            email=email,
            token=token,
            length_of_otp=length_of_otp,
            step_in_seconds=step_in_seconds
        );
        return qr_string;
    }

    pub fn verify(
        &mut self,
        digit1: u32,
        digit2: u32,
        digit3: u32,
        digit4: u32,
        digit5: u32,
        digit6: u32,
    ) -> Result<(), ()> {
        let totp = format!(
            "{}{}{}{}{}{}",
            digit1, digit2, digit3, digit4, digit5, digit6
        );
        let totp_check = self.generate_totp();

        if totp != totp_check {
            return Err(());
        }

        self.verified = true;
        self.verified_at = Some(Utc::now());
        return Ok(());
    }

    pub fn is_verified(&self) -> bool {
        return self.verified;
    }

    pub fn get_url(&self) -> Option<String> {
        match self.fields.clone() {
            Some(fields) => return Some(fields.key),
            None => return None,
        }
    }

    pub fn get_last_updated(&self) -> Option<DateTime<Utc>> {
        match self.fields.clone() {
            Some(fields) => return Some(fields.last_updated),
            None => return None,
        }
    }

    pub fn get_verified_at(&self) -> Option<DateTime<Utc>> {
        return self.verified_at;
    }

    pub fn set_url(&mut self, key: String) {
        let now = Utc::now();

        if self.verified == false {
            self.verified = true;
            self.verified_at = Some(now);
        }
        let fields = TotpFields {
            key,
            last_updated: now,
        };
        self.fields = Some(fields);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dotenv::dotenv;
    use std::env;

    #[test]
    fn totp_test() {
        dotenv().ok();

        let key = "3mr)fxqz14e+awv&jo_!";
        let mut totp = Totp::from_key(key.to_string());
        // println!("{:#?}", totp);
        let values = totp.generate_totp();
        println!("{}", values);

        let email = env::var("TEST_EMAIL").unwrap();
        let qr_string = totp.generate_qr_string(&email);
        println!("{}", qr_string);
    }
}
