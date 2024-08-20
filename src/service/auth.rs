pub struct AuthService;

pub trait AuthServiceTrait {
    async fn verify_password(&self, input_password: &str, stored_password: &str) -> Result<(), ()>;
}

impl AuthService {
    pub fn new() -> AuthService {
        AuthService
    }
}

impl AuthServiceTrait for AuthService {
    async fn verify_password(&self, input_password: &str, stored_password: &str) -> Result<(), ()> {
        if input_password == stored_password {
            Ok(())
        } else {
            Err(())
        }
    }
}
