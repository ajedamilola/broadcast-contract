#![cfg_attr(not(feature = "export-abi"), no_main)]
extern crate alloc;

/// Import items from the SDK. The prelude contains common traits and macros.
use stylus_sdk::{alloy_primitives::*, block, msg, prelude::*};

sol_storage! {
    pub struct Message{
        address sender;
        string content;
        string name;
        uint time_stamp;
    }

    #[entrypoint]
    pub struct BroadCastApp{
        mapping(U256=>Message) data;
        uint number_of_messages;
        string message;
    }
}

#[public]
impl BroadCastApp {
    pub fn send_message(&mut self, content: String, name: String) -> U256 {
        let num = self.number_of_messages.get();
        let current_time = U256::from(block::timestamp());

        let mut message = self.data.setter(num);

        message.content.set_str(content);
        message.sender.set(msg::sender());
        message.time_stamp.set(current_time);
        message.name.set_str(name);

        self.number_of_messages.set(num + U256::from(1));
        self.number_of_messages.get()
    }

    pub fn get_recent_messages(&self) -> String {
        let mut total_messages = String::from("[");
        let num = self.number_of_messages.get();
        let mut first = true;
        for i in 0..201 {
            let current = U256::from(200 - i);
            if current >= num {
                continue;
            }
            let message = self.data.get(current);

            if !first {
                total_messages.push_str(",");
            } else {
                first = false;
            }

            total_messages.push_str(&format!(
                "{{\"sender\":\"{}\",\"content\":\"{}\",\"name\":\"{}\",\"timestamp\":\"{}\"}}",
                message.sender.get().to_string(),
                message.content.get_string().replace("\"", "\\\""),
                message.name.get_string().replace("\"", "\\\""),
                message.time_stamp.get().to_string()
            ));
        }
        total_messages.push_str("]");
        total_messages
    }
}

// cargo stylus cache bid ea9a0bb706c3f6701a5c7ad78f0663eef638ddd6
