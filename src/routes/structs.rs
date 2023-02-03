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

#[derive(Serialize, Deserialize, Debug)]
pub struct DbUser {
    pub id: i32,
    pub lastName: String,
    pub firstName: String,
    pub mail: String,
    pub picture: String,
    pub company: DbCompanyData,
    pub interests: DbInterests,
    pub createdAt: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct DbCompanyData {
    pub userID: i32,
    pub isAssociated: bool,
    pub companyName: String,
    pub companyEmail: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DbInterests {
    pub userID: i32,
    pub webDevelopment: bool,
    pub cyberSecurity: bool,
    pub mobileDev: bool,
    pub design: bool,
    pub dataScience: bool,
    pub coding: bool,
}
#[derive(Serialize, Deserialize)]
pub struct CompanyData {
    pub isAssociated: bool,
    pub companyName: String,
    pub companyEmail: String,
}

#[derive(Serialize, Deserialize)]
pub struct Interests {
    pub webDevelopment: bool,
    pub cyberSecurity: bool,
    pub mobileDev: bool,
    pub design: bool,
    pub dataScience: bool,
    pub coding: bool,
}

#[derive(Deserialize, Debug)]
pub struct Info {
    pub amount_user: i32,
}
#[derive(Deserialize, Debug)]
pub struct Person {
    pub name: String,
    pub family_name: String,
}
