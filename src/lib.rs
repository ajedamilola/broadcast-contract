#![cfg_attr(not(feature = "export-abi"), no_main)]
extern crate alloc;

/// Import items from the SDK. The prelude contains common traits and macros.
use stylus_sdk::{alloy_primitives::*, msg, prelude::*};

sol_storage! {
    pub struct Message{
        address sender;
        string content;
        string name;
        string replying;
        uint128 time_stamp;
        mapping(U8=>string) stars;
    }

    #[entrypoint]
    pub struct BroadCastApp{
        mapping(U64=>Message) data;
        uint64 number_of_messages;
    }
}

#[public]
impl BroadCastApp {
    pub fn send_message(
        &mut self,
        content: String,
        name: String,
        date: u128,
        replying: String,
    ) -> U256 {
        let num = self.number_of_messages.get();

        let mut message = self.data.setter(num);

        message.content.set_str(content);
        message.sender.set(msg::sender());
        message.name.set_str(name);
        message.time_stamp.set(U128::from(date));
        message.replying.set_str(replying);

        self.number_of_messages.set(num + U64::from(1));
        U256::from(self.number_of_messages.get())
    }

    pub fn get_recent_messages(&self) -> String {
        let mut total_messages = String::from("[");
        let num = self.number_of_messages.get();
        let mut first = true;
        for i in 0..201 {
            let current = U64::from(200 - i);
            if current >= num {
                continue;
            }
            let message = self.data.get(current);

            if !first {
                total_messages.push_str(",");
            } else {
                first = false;
            }

            let mut stars = String::from("[");
            let mut stars_first = true;

            for j in 0..255 {
                let key = U8::from(j);
                let star = message.stars.get(key);
                if star.is_empty() {
                    break;
                }
                if !stars_first {
                    stars.push_str(",");
                } else {
                    stars_first = false;
                }
                stars.push_str(&format!("\"{}\"", star.get_string()));
            }
            stars.push_str("]");

            total_messages.push_str(&format!(
                "{{\"sender\":\"{}\",\"replying\":\"{}\",\"content\":\"{}\",\"name\":\"{}\",\"timestamp\":\"{}\",\"stars\":{}}}",
                message.sender.get().to_string(),
                message.replying.get_string().replace("\"", "\\\""),
                message.content.get_string().replace("\"", "\\\""),
                message.name.get_string().replace("\"", "\\\""),
                message.time_stamp.get().to_string(),
                stars
            ));
        }
        total_messages.push_str("]");
        total_messages
    }

    pub fn react_to_message(&mut self, message_id: u64) -> bool {
        let mut message = self.data.setter(U64::from(message_id));
        let sender = msg::sender().to_string();

        for i in 0..255 {
            let key = U8::from(i);
            if message.stars.get(key).is_empty() {
                message.stars.setter(key).set_str(&sender);
                return true;
            }
            if message.stars.get(key).get_string() == sender {
                message.stars.setter(key).set_str("");
                return true;
            }
        }
        false
    }
}

// cargo stylus cache bid ea9a0bb706c3f6701a5c7ad78f0663eef638ddd6
