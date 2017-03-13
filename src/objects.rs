#[derive(Deserialize, Debug)]
pub struct Error {
    pub error: String,
    #[serde(rename = "errorMessage")]
    pub error_message: String,
    pub cause: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Profile {
    pub id: String,
    pub name: String,
    pub legacy: Option<bool>,
}

impl Profile {
    pub fn new(id: String, name: String) -> Profile {
        Profile {
            id: id,
            name: name,
            legacy: None,
        }
    }

    pub fn new_legacy(id: String, name: String) -> Profile {
        Profile {
            id: id,
            name: name,
            legacy: Some(true),
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct Property {
    pub name: String,
    pub value: String,
}

#[derive(Deserialize, Debug)]
pub struct User {
    pub id: String,
    pub properties: Vec<Property>,
}

#[derive(Deserialize, Debug)]
pub struct Authenticate {
    #[serde(rename = "accessToken")]
    pub access_token: String,
    #[serde(rename = "clientToken")]
    pub client_token: String,
    #[serde(rename = "availableProfiles")]
    pub available_profiles: Vec<Profile>,
    #[serde(rename = "selectedProfile")]
    pub selected_profile: Option<Profile>,
    pub user: Option<User>,
}

#[derive(Deserialize, Debug)]
pub struct Refresh {
    #[serde(rename = "accessToken")]
    pub access_token: String,
    #[serde(rename = "clientToken")]
    pub client_token: String,
    #[serde(rename = "selectedProfile")]
    pub selected_profile: Profile,
    pub user: Option<User>,
}
