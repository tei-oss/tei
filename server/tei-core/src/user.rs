use derive_more::Display;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Display)]
pub struct UserId {
    id: i32,
}

impl UserId {
    #[must_use]
    pub fn as_i32(&self) -> i32 {
        self.id
    }

    #[must_use]
    pub fn to_uid(&self) -> String {
        // TODO: Base64URL
        self.id.to_string()
    }
}

impl From<i32> for UserId {
    fn from(val: i32) -> Self {
        UserId { id: val }
    }
}

#[derive(Debug, Clone)]
pub struct User {
    pub id: UserId,
    pub alias: Box<str>,
    pub provider_id: Box<str>,
}
