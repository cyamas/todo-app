use axum::{extract::State, Json};
use tracing::info;
use crate::{templates::*, AddTodo, AppError, CompletedTodo, Date, EditTodo, PendingTodo, TodoId, ActiveTodo, Progress, ProgressNote, DeletedProgress, TotalTime};
use sqlx::{query_as, PgPool};
use anyhow::Context;

pub async fn login() -> Result<LoginTemplate, AppError> {
    Ok(LoginTemplate {})
}


pub async fn home(State(pool): State<PgPool>) -> Result<HomeTemplate, AppError> {
    let pending_todos: Vec<PendingTodo> = sqlx::query_as!(
        PendingTodo,
        "
            SELECT todo_id, project, task, task_priority, created_at, total_time FROM todo
                WHERE completed = false
                ORDER BY task_priority DESC
        "
    )
    .fetch_all(&pool)
    .await
    .context("SQL Error fetching pending todos")?;

    let mut pending = Vec::new();
    for todo in pending_todos {
        let date = Date::from(todo.created_at);
        pending.push(PendingTodoTemplate { todo, date });
    }

    let completed_todos = sqlx::query_as!(
        CompletedTodo,
        "
            SELECT todo_id, project, task, task_priority, completed_at, total_time FROM todo
                WHERE completed = true
                ORDER BY completed_at DESC
        "
    )
    .fetch_all(&pool)
    .await
    .context("Error fetching completed todos")?;

    let mut completed = Vec::new();
    for todo in completed_todos {
        let date = Date::from(todo.completed_at.unwrap());
        completed.push(CompletedTodoTemplate{ todo, date });
    }

    Ok(HomeTemplate { pending, completed })


}


pub async fn add_todo(State(pool): State<PgPool>,Json(payload): Json<AddTodo>) -> Result<PendingTodoTemplate, AppError> {
    let project: &str = &payload.project;
    let task: &str = &payload.task;
    let task_priority: &i32 = &payload.priority.parse().unwrap();

    sqlx::query!(r#"INSERT INTO todo (project, task, task_priority) VALUES ($1, $2, $3);"#, &project, &task, &task_priority)
    .execute(&pool)
    .await
    .context("Error adding todo entry to database")?;

    info!("{:?}", payload);
    let todo = query_as!(
        PendingTodo,
        "
            SELECT todo_id, project, task, task_priority, created_at, total_time FROM todo
            ORDER BY todo_id DESC
            LIMIT 1
        "
    )
    .fetch_one(&pool)
    .await
    .context("SQL error fetching pending todo")?;

    let date = Date::from(todo.created_at);
    info!("{:?}", todo.created_at);

    Ok(PendingTodoTemplate { todo, date })

}


pub async fn complete_todo(State(pool): State<PgPool>, Json(data): Json<TodoId>) -> Result<CompletedTodoTemplate, AppError> {
    let todo_id = &data.id.parse::<i32>().context("Error parsing todo id")?;
    sqlx::query!(r#"
        UPDATE todo SET
            completed = true,
            completed_at = NOW() AT TIME ZONE 'America/Los_Angeles' 
                WHERE todo_id = $1"#, todo_id)
    .execute(&pool)
    .await
    .context("Error updating todo")?;

    let completed = sqlx::query_as!(
        CompletedTodo,
        "
            SELECT todo_id, project, task, task_priority, completed_at, total_time FROM todo
            WHERE todo_id = $1
            ORDER BY completed_at DESC
        ",
        todo_id
    )
    .fetch_one(&pool)
    .await
    .context("Error fetching completed todo")?;
    
    let date = Date::from(completed.completed_at.unwrap());

    Ok(CompletedTodoTemplate {
        todo: completed,
        date,
    })
}


pub async fn edit_form(State(pool): State<PgPool>, Json(data): Json<TodoId>) -> Result<EditFormTemplate, AppError> {
    let todo_id = &data.id.parse::<i32>().unwrap();
    let todo_data = sqlx::query_as!(
        EditFormTemplate,
        "
            SELECT todo_id, project, task, task_priority FROM todo
                WHERE todo_id = $1
        ",
        todo_id
    )
    .fetch_one(&pool)
    .await
    .context("Error fetching todo for edit")?;

    Ok(todo_data)
}


pub async fn edit_todo(State(pool):State<PgPool>, Json(data): Json<EditTodo>) -> Result<PendingTodoTemplate, AppError> {
    let todo_id = data.id.parse::<i32>().unwrap();
    let task_priority = data.priority.parse::<i32>().unwrap();

    sqlx::query!(r#"
        UPDATE todo SET project = $1, task = $2, task_priority = $3
            WHERE todo_id = $4
        "#,
        data.project,
        data.task,
        task_priority,
        todo_id)
    .execute(&pool)
    .await
    .context("Error updating todo")?;

    let todo = query_as!(
        PendingTodo,
        "
            SELECT todo_id, project, task, task_priority, created_at, total_time FROM todo
            WHERE todo_id = $1
            LIMIT 1
        ",
        todo_id
    )
    .fetch_one(&pool)
    .await
    .context("Error fetching todo")?;

    let date = Date::from(todo.created_at);

    Ok(PendingTodoTemplate { todo, date })
}


pub async fn delete_todo(State(pool): State<PgPool>, Json(data): Json<TodoId>) -> Result<&'static str, AppError> {
    let todo_id: &i32 = &data.id.parse().unwrap();
    sqlx::query!(r#"DELETE FROM todo WHERE todo_id = $1"#, todo_id)
    .execute(&pool)
    .await
    .context("Error deleting todo")?;

    Ok("")
}


pub async fn revert_todo(State(pool): State<PgPool>, Json(data): Json<TodoId>) -> Result<RevertTodoTemplate, AppError> {
    let todo_id = data.id.parse::<i32>().unwrap();
    let todo = sqlx::query_as!(
        PendingTodo,
        "
            SELECT todo_id, project, task, task_priority, created_at, total_time FROM todo
                WHERE todo_id = $1
        ",
        todo_id
    )
    .fetch_one(&pool)
    .await
    .context("Error fetching reverted todo from database")?;

    let date = Date::from(todo.created_at);

    sqlx::query!("UPDATE todo SET completed = false, completed_at = to_timestamp(0) WHERE todo_id = $1", todo_id)
    .execute(&pool)
    .await
    .context("Error updating reverted todo in database")?;

    Ok(RevertTodoTemplate {
        todo_id,
        todo: PendingTodoTemplate { todo, date},
    })

}


pub async fn activate_todo(State(pool): State<PgPool>, Json(data): Json<TodoId>) -> Result<ActiveTodoTemplate, AppError> {
    let todo_id = data.id.parse::<i32>().expect("could not parse todo id");
    sqlx::query!(r#"UPDATE todo SET is_active = true WHERE todo_id = $1"#, todo_id)
    .execute(&pool)
    .await
    .context("Error activating todo in database")?;
    
    let active_todo = sqlx::query_as!(
        ActiveTodoTemplate,
        "
            SELECT todo_id, project, task, task_priority FROM todo
                WHERE todo_id = $1
        ",
        todo_id
    )
    .fetch_one(&pool)
    .await
    .context("Error fetching active todo")?;

    Ok(active_todo)
}

pub async fn deactivate_todo(State(pool): State<PgPool>, Json(data): Json<ActiveTodo>) -> Result<DeactivateTodoTemplate, AppError> {
    let todo_id = data.id.parse::<i32>().expect("Could not parse todo id");
    let note = data.note;
    let time_spent = data.duration.parse::<i32>().expect("Could not parse active todo duration");

    sqlx::query!(r#" INSERT INTO progress (todo_id, note, time_spent) VALUES ($1, $2, $3)"#, todo_id, note, time_spent)
    .execute(&pool)
    .await
    .context("Error inserting into progress table")?;

    let progress = sqlx::query_as!(
        Progress,
        "
            SELECT * FROM progress
        "
    )
    .fetch_all(&pool)
    .await
    .context("Error retrieving data from progress table")?;

    info!("{:#?}", progress);
    
    sqlx::query!(r#"UPDATE todo SET is_active = false, total_time = total_time + $2 WHERE todo_id = $1"#, todo_id, time_spent)
    .execute(&pool)
    .await
    .context("Error updating is_active to false in database")?;

    let todo = sqlx::query_as!(
        PendingTodo,
        "
        SELECT todo_id, project, task, task_priority, created_at, total_time FROM todo
            WHERE todo_id = $1
    ",
    todo_id
    )
    .fetch_one(&pool)
    .await
    .context("Error fetching pending todo from database")?;

    let date = Date::from(todo.created_at);

    let todo = PendingTodoTemplate { todo, date };
    Ok(DeactivateTodoTemplate {todo_id, todo })
}


pub async fn show_progress(State(pool): State<PgPool>, Json(data): Json<TodoId>) -> Result<ProgressTemplate, AppError> {
    let todo_id: i32 = data.id.parse().expect("Error parsing todo id");
    let notes = sqlx::query_as!(
        ProgressNote,
        "
            SELECT progress_id, note, time_spent, made_at FROM progress
                WHERE todo_id = $1
        ",
        todo_id
    )
    .fetch_all(&pool)
    .await
    .context("Error fetching progress data")?;

    let mut progress = Vec::new();

    for note in notes.iter() {
        let date = Date::from(note.made_at);
        progress.push(ProgressNoteTemplate {
            progress_id: note.progress_id,
            note: String::from(&note.note),
            time_spent: note.time_spent,
            date,
            todo_id,
        });
    }

    Ok(ProgressTemplate { todo_id, progress })    

}

pub async fn hide_progress(Json(data): Json<TodoId>) -> Result<HideProgressTemplate, AppError> {
    let todo_id: i32 = data.id.parse().expect("Error parsing todo id");
    Ok(HideProgressTemplate { todo_id })
}


pub async fn delete_progress(State(pool): State<PgPool>, Json(data): Json<DeletedProgress>) -> Result<DeleteProgressTemplate, AppError> {
    let progress_id: i32 = data.id.parse().expect("Error parsing progress id");
    let time_spent: i32 = data.time_spent.parse().expect("Error parsing time spent");
    let todo_id: i32 = data.todo_id.parse().expect("Error parsing todo id");
    
    sqlx::query!(r#"DELETE FROM progress WHERE progress_id = $1"#, progress_id)
    .execute(&pool)
    .await
    .context("Error deleting entry in progress table")?;

    sqlx::query!(r#"UPDATE todo SET total_time = total_time - $1 WHERE todo_id = $2"#, time_spent, todo_id)
    .execute(&pool)
    .await
    .context("Error updating total_time")?;

    let total_time = sqlx::query_as!(TotalTime, "SELECT total_time FROM todo WHERE todo_id = $1", todo_id)
    .fetch_one(&pool)
    .await
    .context("Error fetching total time from todo table")?;

    Ok(DeleteProgressTemplate { todo_id, total_time: total_time.total_time })
}