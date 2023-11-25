use auxiliary::Audit;
use group::Group;
use user::UserId;

pub mod auxiliary;
pub mod group;
pub mod tag;
pub mod user;

pub fn test() {
    let user_id: UserId = 123.into();

    let mut group = Group {
        id: 1.into(),
        title: "AI-art AKA 6 finger horrors".into(),
        tags: [].into(),
        audit: Audit::created(&user_id),
    };

    group.audit.updated(&user_id);
}
