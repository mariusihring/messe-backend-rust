use crate::TOKENS;
use actix_web::{
    cookie::Display,
    error::ErrorUnauthorized,
    guard::{self, GuardContext},
    web, HttpRequest, HttpResponse, Result,
};
use std::fmt;

pub fn check_token(req: &GuardContext<'_>) -> bool {
    let itter = req.head().headers().iter();
    for i in itter {
        if i.0 == "token" && unsafe { TOKENS.contains(&i.1.to_str().unwrap().to_owned()) } {
            return true;
        }
    }
    false
}
