use actix_web::{dev::Payload, Error, FromRequest, HttpMessage, HttpRequest};
use futures::future::{ok, Ready};

use crate::users::UserIdentity;

impl FromRequest for UserIdentity {
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
