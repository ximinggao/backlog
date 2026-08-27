pub struct HeaderToken {
    pub message: String,
}

#[cfg(feature = "actix")]
mod actix_impl {
    use super::HeaderToken;
    use crate::errors::{NanoServiceError, NanoServiceErrorStatus};
    pub use actix_web::{FromRequest as ActixFromRequest, HttpRequest, dev::Payload};
    use futures::future::{Ready, err, ok};

    impl ActixFromRequest for HeaderToken {
        type Error = NanoServiceError;
        type Future = Ready<Result<Self, NanoServiceError>>;

        fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
            let raw_data = match req.headers().get("token") {
                Some(data) => data,
                None => {
                    return err(NanoServiceError {
                        message: "token not in header under key 'token'".to_string(),
                        status: NanoServiceErrorStatus::Unauthorized,
                    });
                }
            };

            let message = match raw_data.to_str() {
                Ok(token) => token.to_string(),
                Err(_) => {
                    return err(NanoServiceError {
                        message: "token not a valid string".to_string(),
                        status: NanoServiceErrorStatus::Unauthorized,
                    });
                }
            };

            ok(HeaderToken { message })
        }
    }
}

#[cfg(feature = "actix")]
pub use actix_impl::ActixFromRequest;
