use std::time::SystemTime;

#[derive(Debug)]
pub struct Totp {
    pub verified: bool,
    pub verified_at: Option<SystemTime>,
    pub fields: Option<TotpFields>,
}

#[derive(Debug)]
pub struct TotpFields {
    pub url: String,
    pub last_updated: SystemTime,
}

impl Totp {
    pub fn new() -> Totp {
        Totp {
            verified: false,
            verified_at: None,
            fields: None,
        }
    }

    pub fn verify(&self) {
        self.verified = true;
        self.verified_at = Some(SystemTime::now());
    }

    pub fn is_verified(&self) -> bool {
        return self.verified;
    }

    pub fn get_url(&self) ->  Option<String> {
        match self.fields {
            Some(fields) => return Some(fields.url),
            None => return None,
        }
    }

    pub fn get_last_updated(&self) -> Option<SystemTime> {
        match self.fields {
            Some(fields) => return Some(fields.last_updated),
            None => return None,
        }
    }

    pub fn get_verified_at(&self) -> Option<SystemTime> {
        return self.verified_at;
    }

    pub fn set_url(&self, url: String) {
        let now = SystemTime::now();
        
        if self.verified == false {
            self.verified = true;
            self.verified_at = Some(now);
        }
        let fields = TotpFields{url, last_updated: now};
        self.fields = Some(fields);
    }
}
