use crate::{enums::TaskStatus, structs::base::Base};

pub struct Done {
    pub super_struct: Base,
}

impl Done {
    pub fn new(input_title: &str) -> Self {
        Done {
            super_struct: Base {
                title: input_title.to_string(),
                status: TaskStatus::DONE,
            },
        }
    }
}
