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

#[cfg(test)]
mod tests {
    use testing_table::assert_table;

    use super::*;

    #[test]
    fn test_make_table() {
        let git_users = vec![
            GitUser {
                name: "Alice".to_string(),
                email: "alice.dummy@dummy.com".to_string(),
                description: None,
            },
            GitUser {
                name: "Bob".to_string(),
                email: "bob.dummy@dummy.com".to_string(),
                description: Some("A person".to_string()),
            },
        ];
        let table = make_table(git_users);
        assert_table!(
            table,
            "╭───────┬───────────────────────┬─────────────╮"
            "│ name  │ email                 │ description │"
            "├───────┼───────────────────────┼─────────────┤"
            "│ Alice │ alice.dummy@dummy.com │             │"
            "├───────┼───────────────────────┼─────────────┤"
            "│ Bob   │ bob.dummy@dummy.com   │ A person    │"
            "╰───────┴───────────────────────┴─────────────╯"
        );
    }
}
