pub struct SocialMedia {
    links: Vec<Link>,
}

struct Link {
    url: String,
    provider: Provider,
}

enum Provider {
    None,
    Amazon,
    Instagram,
    Twitter,
}
