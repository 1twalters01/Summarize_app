use std::time::SystemTime;

pub struct Totp {
    pub verified: bool,
    pub verified_at: Option<SystemTime>,
    pub fields: Option<TotpFields>,
}

pub struct TotpFields {
    pub url: String,
    pub last_updated: SystemTime,
}

impl Totp {
    pub fn new() {
        Totp {
            verified: false,
            verified_at: None,
            fields: None,
        }
    }
    pub fn verify(&self) {
        self.verified = true;
        self.verified_at = SystemTime::now();
    }

    pub fn is_verified(&self) -> bool {
        self.verified;
    }

    pub fn get_url(&self) ->  Option<String> {
        match self.fields {
            Some(fields) => return fields.url,
            None => return None,
        }
    }

    pub fn get_last_updated(&self) {
        match self.fields {
            Some(fields) => return fields.last_updated,
            None => return None,
        }
    }

    pub fn get_verified_at(&self) {
        match self.fields {
            Some(fields) => return fields.verified_at,
            None => return None,
        }
    }

    pub fn set_url(&self, url: String) {
        let now = SystemTime::now();
        
        if self.verified == false {
            self.verified = true;
            let fields = TotpFields{url, verified_at = now , last_updated = now};
            self.fields = fields;
        } else {
            self.fields.url = url;
            self.fields.last_update = now;
        }
    }
}
