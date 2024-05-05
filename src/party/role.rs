use std::ops::Not;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Role {
    Server,
    Client,
}

impl Role {
    pub fn index(&self) -> usize {
        match self {
            Role::Server => 0,
            Role::Client => 1,
        }
    }
}

impl Not for Role {
    type Output = Role;

    fn not(self) -> Self::Output {
        match self {
            Role::Server => Role::Client,
            Role::Client => Role::Server,
        }
    }
}