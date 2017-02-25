use std::borrow::Cow;

/// Login details
#[derive(Serialize, Debug)]
pub struct Details<'a> {
    /// The email or username to log in with (either works)
    pub email: Cow<'a, str>,
    /// The password to log in with (steam auth is not supported)
    pub password: Cow<'a, str>,
}

impl<'a> Details<'a> {
    /// Create a new login details with the given username and password
    pub fn new<'b, T1: Into<Cow<'b, str>>, T2: Into<Cow<'b, str>>>(email: T1, password: T2) -> Details<'b> {
        Details {
            email: email.into(),
            password: password.into(),
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct Response {
    pub ok: i32,
    pub token: Option<String>,
}
