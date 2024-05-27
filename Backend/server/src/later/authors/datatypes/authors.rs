use uuid::Uuid;
use chronos::time;

use crate::accounts::datatypes::users::User;
use crate::authors::datatypes::social_media::SocialMedia;

pub struct Author {
    uuid: Uuid,
    information: Information,
    biography: Biography,
    works: Works,
    achievements: Achievements,
    genres: Genres,
    influences: Influences,
    multimedia: Vec<Multimedia>,
    engagement: Engagement,
    community: Community,
    quotes: String,
    publication_information: PublicationInformation,
    user: Option<User>,
    average_rating: f32,
}

pub struct Information {
    first_name: Option<String>,
    middle_names: Vec<String>,
    last_name: Option<String>,
    languages: Option<String>,
    date_of_birth: time,
    date_of_death: option<time>,
}

pub struct Biography {
    early_life: Option<String>,
    career_overview: Option<String>,
    personal_life: Option<String>,
}

pub struct Works {
    bibliography: Vec<Book>,
    notable_works: Option<Book>,
    collaborations: Vec<Collaboration>,
}

pub struct Collaboration{
    authors: vec<Author>,
    book: Book,
}

pub struct Achievements {
    awards: Vec<String>,
    bestsellers: Vec<Book>,
    critical_acclaim: Vec<String>,
}

pub struct Genres {
    primary: vec<Genre>,
    writing_style: String,
}

pub struct Influences {
    literary_influences: vec<LiteraryInfluences>,
    personal_inspirations: String,
}

pub enum LiteraryInfluences {
    Author,
    Book,
}

pub enum Multimedia {
// links
    Photos(String),
    Videos(String),
    Audio(String),
}

pub struct Engagement {
    website: String,
    social_media: Vec<SocialMedia>,
    interviews: Vec<String>, // links
    articles: Vec<String>, //links
}

pub struct Community {
}

pub struct PublicationInformation {
    publisher_details: Publishers,
}



impl Author {
    pub fn new(first_name: Option<String>, last_name: Option<String>) -> Author {
        Self {
            uuid: Uuid::new_v4(),
            first_name,
            last_name,
            bio: None,
            social_media: Vec::new(),
            user: None,
            average_rating: 0.0,
            media: Vec::new(),
        }
    }

    pub fn get_uuid(&self) -> Uuid {
        self.uuid.to_owned()
    }

    pub fn get_full_name(&self) -> String {
        let first_name = self.get_first_name();
        let last_name = self.get_last_name();

        return format!("{} {}", first_name.to_owned(), last_name.to_owned());
    }

    pub fn get_first_name(&self) -> String {
        let first_name = match self.first_name.clone() {
            Some(first_name) => first_name,
            None => String::new(),
        };
        return first_name;
    }

    pub fn get_last_name(&self) -> String {
        let last_name = match self.last_name.clone() {
            Some(last_name) => last_name,
            None => String::new(),
        };
        return last_name;
    }

    pub fn get_bio(&self) -> String {
        let bio = match self.bio.clone() {
            Some(bio) => bio,
            None => String::new(),
        };
        return bio;
    }

    pub fn get_social_media(&self) -> Vec<SocialMedia> {
        self.social_media
    }

    pub fn is_user(&self) -> bool {
        self.user.is_some()
    }

    pub fn get_user(&self) -> Option<String> {
        let id: Option<String> = match self.user {
            Ok(res) => res.get_id(&self).unwrap(),
            None => None,
        };

        return id;
    }
    pub fn get_average_rating(&self) -> f32 {
        self.average_rating
    }
        
}




