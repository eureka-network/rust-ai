pub enum Task {
    Comment { comment: String },
    Context { context: String },
    Discussion { discussion: String, supports: bool },
    Voting { vote: String },
}

impl Task {
    pub fn prompt_task(&self) -> Prompt {
        Prompt
    }
}
