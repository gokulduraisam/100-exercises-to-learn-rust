mod ticket {
    pub(crate) struct Ticket {
        title: String,
        description: String,
        status: String,
    }

    impl Ticket {
        pub(crate) fn new(
            title: String,
            description: String,
            status: String,
        ) -> Ticket {
            if title.is_empty() {
                panic!("Title cannot be empty");
            }

            if title.len() > 50 {
                panic!("Title cannot be longer than 50 bytes");
            }

            if description.is_empty() {
                panic!("Description cannot be empty");
            }

            if description.len() > 500 {
                panic!("Description cannot be longer than 500 bytes");
            }

            if status != "To-Do"
                && status != "In Progress"
                && status != "Done"
            {
                panic!(
                    "Only `To-Do`, `In Progress`, and `Done` statuses are allowed"
                );
            }

            Ticket {
                title,
                description,
                status,
            }
        }
    }
}

// 👈 ADD THIS
use ticket::Ticket;

fn create_todo_ticket(title: String, description: String) -> Ticket {
    Ticket::new(title, description, "To-Do".into())
}

#[cfg(test)]
mod tests {
    use super::ticket::Ticket;

    // This should fail because `description` is private.
    /*
    fn should_not_be_possible() {
        let ticket = Ticket::new(
            "A title".into(),
            "A description".into(),
            "To-Do".into(),
        );

        assert_eq!(ticket.description, "A description");
    }
    */

    // This should fail because the fields are private.
    /*
    fn encapsulation_cannot_be_violated() {
        let ticket = Ticket {
            title: "A title".into(),
            description: "A description".into(),
            status: "To-Do".into(),
        };
    }
    */
}