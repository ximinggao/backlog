use super::base::Base;
use crate::enums::TaskStatus;

pub struct Pending {
    pub super_struct: Base,
}

impl Pending {
    pub fn new(title: &str) -> Self {
        Pending {
            super_struct: Base {
                title: title.to_string(),
                status: TaskStatus::PENDING,
            },
        }
    }
}
