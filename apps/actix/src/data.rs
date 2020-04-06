use std::ops::Deref;
use std::sync::Arc;

use actix_web::{dev::Payload, error::ErrorInternalServerError, Error, FromRequest, HttpRequest};
use futures::future::{err, ok, Ready};

#[derive(Debug)]
pub struct Data<T: ?Sized>(Arc<T>);

impl<T: ?Sized> From<Arc<T>> for Data<T> {
    fn from(v: Arc<T>) -> Self {
        Data(v)
    }
}

impl<T> Data<T> {
    pub fn new(state: T) -> Data<T> {
        Data(Arc::new(state))
    }

    /// Get reference to inner app data.
    pub fn get_ref(&self) -> &T {
        self.0.as_ref()
    }

    /// Convert to the internal Arc<T>
    pub fn into_inner(self) -> Arc<T> {
        self.0
    }
}

impl<T: ?Sized> Deref for Data<T> {
    type Target = Arc<T>;

    fn deref(&self) -> &Arc<T> {
        &self.0
    }
}

impl<T: ?Sized> Clone for Data<T> {
    fn clone(&self) -> Data<T> {
        Data(self.0.clone())
    }
}

impl<T: 'static + ?Sized> FromRequest for Data<T> {
    type Config = ();
    type Error = Error;
    type Future = Ready<Result<Self, Error>>;

    #[inline]
    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        if let Some(st) = req.app_data::<Data<T>>() {
            ok(st.clone())
        } else {
            err(ErrorInternalServerError(
                "App data is not configured, to configure use App::data()",
            ))
        }
    }
}
