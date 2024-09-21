#![cfg_attr(not(feature = "export-abi"), no_main)]
extern crate alloc;
/// Import items from the SDK. The prelude contains common traits and macros.
use stylus_sdk::prelude::{entrypoint, public, sol_storage};

sol_storage! {
    #[entrypoint]
    struct Database{
        string data;
    }
}

#[public]
impl Database {
    pub fn push_message(&mut self, new_message: String) {
        self.data.set_str(new_message);
    }

    pub fn data(&mut self) -> String {
        self.data.get_string()
    }
}

// cargo stylus cache bid ea9a0bb706c3f6701a5c7ad78f0663eef638ddd6
