use crate::service::greet::GreetService;

pub struct AppState {
    pub greet: GreetService,
}

impl AppState {
    pub fn new() -> AppState {
        AppState {
            greet: GreetService::new(),
        }
    }
}
