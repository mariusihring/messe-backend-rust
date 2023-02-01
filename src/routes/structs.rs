use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct NewUser {
    pub lastName: String,
    pub firstName: String,
    pub mail: String,
    pub picture: String,
    pub company: CompanyData,
    pub interests: Interests,
}

#[derive(Serialize, Deserialize)]
pub struct CompanyData {
    pub userID: i32,
    pub isAssociated: bool,
    pub companyName: String,
    pub companyEmail: String,
}

#[derive(Serialize, Deserialize)]
pub struct Interests {
    pub userID: i32,
    pub webDevelopment: bool,
    pub cyberSecurity: bool,
    pub mobileDev: bool,
    pub design: bool,
    pub dataScience: bool,
    pub coding: bool,
}
