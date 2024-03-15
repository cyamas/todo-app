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


#[derive(Template)]
#[template(path = "active.html")]
pub struct ActiveTodoTemplate {
    pub todo_id: i32,
    pub project: String,
    pub task: String,
    pub task_priority: i32, 
}

#[derive(Template)]
#[template(path = "deactivatetodo.html")]
pub struct DeactivateTodoTemplate {
    pub todo_id: i32,
    pub todo: PendingTodoTemplate,
}

#[derive(Template)]
#[template(path = "progressnote.html")]
pub struct ProgressNoteTemplate {
    pub progress_id: i32,
    pub note: String,
    pub time_spent: i32,
    pub date: Date,
    pub todo_id: i32,
}

#[derive(Template)]
#[template(path = "progress.html")]
pub struct ProgressTemplate {
    pub todo_id: i32,
    pub progress: Vec<ProgressNoteTemplate>,
}

#[derive(Template)]
#[template(path = "hideprogress.html")]
pub struct HideProgressTemplate {
    pub todo_id: i32,
}

#[derive(Template)]
#[template(path = "deleteprogress.html")]
pub struct DeleteProgressTemplate {
    pub todo_id: i32,
    pub total_time: i32,
}




