use crate::{
    media::Media,
    social_media::SocialMedia
};
use users::user::User;

pub struct Author {
    id: String,
    first_name: String,
    last_name: String,
    bio: String,
    social_media: Vec<SocialMedia>,
    user: Option<User>,
    average_rating: f32,
    media: Vec<Media>,
}

impl Author {
    fn new(first_name: String, last_name: String) -> Author {
        Self {
            id: String::new(),
            first_name,
            last_name,
            bio: String::new(),
            social_media: Vec::new(),
            user: None,
            average_rating: 0.0,
            media: Vec::new(),
        }
    }

    fn get_full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    fn get_first_name(&self) -> String {
        self.first_name.clone()
    }

    fn get_last_name(&self) -> String {
        self.last_name.clone()
    }

    fn is_user(&self) -> bool {
        self.user.is_some()
    }

    fn get_user_id(&self) -> Option<String> {
        let id: Option<String> = match self.user {
            Ok(res) => res.get_id(&self).unwrap(),
            None => None,
        };

        return id;
    }
}
