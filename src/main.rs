use borsh::{BorshDeserialize, BorshSerialize};
use crosstown_bus::{CrosstownBus, MessageHandler, HandleError};

#[derive(Debug, Clone, BorshDeserialize, BorshSerialize)]
pub struct UserCreatedEventMessage {
    pub user_id: String,
    pub user_name: String
}

pub struct UserCreatedHandler;

impl MessageHandler<UserCreatedEventMessage> for UserCreatedHandler {
    fn handle(&self, message: Box<UserCreatedEventMessage>) -> Result<(), HandleError> {
        println!("Message received on handler 1: {:?}", message);
        Ok(())
    }

    fn get_handler_action(&self) -> String {
        "UserCreatedHandler".to_owned()
    }
}

fn main() {
    let mut p = CrosstownBus::new_queue_publisher("amqps://vrzbvskl:XcpfaLgOY71kb1uofvy59h3LVzaB_oHJ@armadillo.rmq.cloudamqp.com/vrzbvskl".to_owned()).unwrap();
    _ = p.publish_event("user_created".to_owned(), UserCreatedEventMessage {
    user_id: "1".to_owned(), user_name: "2306275746-Amir".to_owned() });
    _ = p.publish_event("user_created".to_owned(), UserCreatedEventMessage {
    user_id: "2".to_owned(), user_name: "2306275746-Budi".to_owned() });
    _ = p.publish_event("user_created".to_owned(), UserCreatedEventMessage {
    user_id: "3".to_owned(), user_name: "2306275746-Cica".to_owned() });
    _ = p.publish_event("user_created".to_owned(), UserCreatedEventMessage {
    user_id: "4".to_owned(), user_name: "2306275746-Dira".to_owned() });
    _ = p.publish_event("user_created".to_owned(), UserCreatedEventMessage {
    user_id: "5".to_owned(), user_name: "2306275746-Emir".to_owned() });
}