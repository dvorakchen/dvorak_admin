use crate::models::{User, UserError};

impl User {
    pub fn login(username: String, password: String) -> Result<Self, UserError> {
        //  your logic of valid username and password
        //  code below just some demonstrate 
        if username.is_empty() || password.is_empty() {
            Err(UserError::NotExist)
        } else {
            Ok(User {
                id: "123456".to_string(),
                username: "Dvorak".to_string(),
            })
        }
    }
}
