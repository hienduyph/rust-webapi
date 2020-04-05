use actix_web::{dev::Payload, Error, FromRequest, HttpRequest};
use futures::future::{ok, Ready};

pub struct UserIdentity {
    pub email: String,
    pub user_id: String,
}

impl FromRequest for UserIdentity {
    type Config = ();
    type Error = Error;
    type Future = Ready<Result<UserIdentity, Error>>;

    #[inline]
    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let data = req
            .extensions()
            .get::<UserIdentity>()
            .map(|u| UserIdentity {
                email: u.email.clone(),
                user_id: u.user_id.clone(),
            })
            .unwrap();
        ok(data)
    }
}
