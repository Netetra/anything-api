#[derive(Clone)]
pub struct GreetService;

pub trait GreetServiceTrait {
    async fn hello_world(&self) -> &'static str;
    async fn good_morning(&self) -> &'static str;
    async fn good_afternoon(&self) -> &'static str;
    async fn good_night(&self) -> &'static str;
}

impl GreetService {
    pub fn new() -> GreetService {
        GreetService
    }
}

impl GreetServiceTrait for GreetService {
    async fn hello_world(&self) -> &'static str {
        "Hello World!"
    }

    async fn good_morning(&self) -> &'static str {
        "Good Morning!"
    }

    async fn good_afternoon(&self) -> &'static str {
        "Good Afternoon."
    }

    async fn good_night(&self) -> &'static str {
        "Good Night."
    }
}
