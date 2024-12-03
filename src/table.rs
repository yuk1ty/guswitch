use tabled::{settings::Style, Table, Tabled};

use crate::config::GitUser;

#[derive(Tabled)]
struct TabledGitUser {
    name: String,
    email: String,
    description: String, // when the original string is None, should be empty string
}

impl From<GitUser> for TabledGitUser {
    fn from(value: GitUser) -> Self {
        TabledGitUser {
            name: value.name,
            email: value.email,
            description: value.description.unwrap_or_default(),
        }
    }
}

pub fn make_table(git_users: Vec<GitUser>) -> Table {
    let tabled_users: Vec<TabledGitUser> = git_users.into_iter().map(TabledGitUser::from).collect();
    let mut table = Table::new(tabled_users);
    table.with(Style::modern_rounded());
    table
}
