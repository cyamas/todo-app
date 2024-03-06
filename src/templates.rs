use askama::Template;
use crate::{PendingTodo, CompletedTodo, Date};

#[derive(Template)]
#[template(path = "login.html")]
pub struct LoginTemplate {}

#[derive(Template)]
#[template(path = "index.html")]
pub struct HomeTemplate {
    pub pending: Vec<PendingTodoTemplate>,
    pub completed: Vec<CompletedTodoTemplate>,
}

#[derive(Template, Debug)]
#[template(path = "pending.html")]
pub struct PendingTodoTemplate {
    pub todo: PendingTodo,
    pub date: Date,
}

#[derive(Template)]
#[template(path = "completed.html")]
pub struct CompletedTodoTemplate {
    pub todo: CompletedTodo,
    pub date: Date,
}

#[derive(Template, Debug)]
#[template(path = "editform.html")]
pub struct EditFormTemplate {
    pub todo_id: i32,
    pub project: String,
    pub task: String,
    pub task_priority: i32,
}

#[derive(Template)]
#[template(path = "revert.html")]
pub struct RevertTodoTemplate {
    pub todo_id: i32,
    pub todo: PendingTodoTemplate,
}

pub struct CompletedTodosTemplate {
    pub completed: Vec<CompletedTodoTemplate>
}



