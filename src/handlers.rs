use axum::{extract::State, Json};
use tracing::info;
use crate::{templates::*, AddTodo, AppError, CompletedTodo, Date, EditTodo, PendingTodo, TodoId, ActiveTodo};
use sqlx::{query_as, PgPool};
use chrono::Datelike;
use anyhow::Context;

pub async fn login() -> Result<LoginTemplate, AppError> {
    Ok(LoginTemplate {})
}


pub async fn home(State(pool): State<PgPool>) -> Result<HomeTemplate, AppError> {
    let pending_todos: Vec<PendingTodo> = sqlx::query_as!(
        PendingTodo,
        "
            SELECT todo_id, project, task, task_priority, created_at, time_spent FROM todos
                WHERE completed = false
                ORDER BY task_priority DESC
        "
    )
    .fetch_all(&pool)
    .await
    .context("SQL Error fetching pending todos")?;

    let mut pending = Vec::new();
    for todo in pending_todos {
        let date = Date {
            month: todo.created_at.month(),
            day: todo.created_at.day(),
            year: todo.created_at.year(),
        };
        pending.push(PendingTodoTemplate { todo, date });
    }

    let completed_todos = sqlx::query_as!(
        CompletedTodo,
        "
            SELECT todo_id, project, task, task_priority, completed_at, time_spent FROM todos
                WHERE completed = true
                ORDER BY completed_at DESC
        "
    )
    .fetch_all(&pool)
    .await
    .context("Error fetching completed todos")?;

    let mut completed = Vec::new();
    for todo in completed_todos {
        let datetime = todo.completed_at.unwrap();
        let date = Date {
            month: datetime.month(),
            day: datetime.day(),
            year: datetime.year(),
        };
        completed.push(CompletedTodoTemplate{ todo, date });
    }

    Ok(HomeTemplate { pending, completed })


}


pub async fn add_todo(State(pool): State<PgPool>,Json(payload): Json<AddTodo>) -> Result<PendingTodoTemplate, AppError> {
    let project: String = payload.project;
    let task: String = payload.task;
    let task_priority: i32 = payload.priority.parse::<i32>().unwrap();

    sqlx::query!(r#"INSERT INTO todos (project, task, task_priority) VALUES ($1, $2, $3);"#, &project, &task, &task_priority)
    .execute(&pool)
    .await
    .context("Error adding todo entry to database")?;

    let todo = query_as!(
        PendingTodo,
        "
            SELECT todo_id, project, task, task_priority, created_at, time_spent FROM todos
            ORDER BY todo_id DESC
            LIMIT 1
        "
    )
    .fetch_one(&pool)
    .await
    .context("SQL error fetching pending todo")?;

    let date = Date {
        month: todo.created_at.month(),
        day: todo.created_at.day(),
        year: todo.created_at.year(),
    };
    info!("{:?}", todo.created_at);

    Ok(PendingTodoTemplate { todo, date })

}


pub async fn complete_todo(State(pool): State<PgPool>, Json(data): Json<TodoId>) -> Result<CompletedTodoTemplate, AppError> {
    let todo_id = &data.id.parse::<i32>().context("Error parsing todo id")?;
    sqlx::query!(r#"
        UPDATE todos SET
            completed = true,
            completed_at = NOW() AT TIME ZONE 'America/Los_Angeles' 
                WHERE todo_id = $1"#, todo_id)
    .execute(&pool)
    .await
    .context("Error updating todo")?;

    let completed = sqlx::query_as!(
        CompletedTodo,
        "
            SELECT todo_id, project, task, task_priority, completed_at, time_spent FROM todos
            WHERE todo_id = $1
            ORDER BY completed_at DESC
        ",
        todo_id
    )
    .fetch_one(&pool)
    .await
    .context("Error fetching completed todo")?;

    let datetime = completed.completed_at.context("Error with datetime")?;
    let date = Date {
        month: datetime.month(),
        day: datetime.day(),
        year: datetime.year(),
    };
    info!("{:#?}", datetime);

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
            SELECT todo_id, project, task, task_priority FROM todos
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
        UPDATE todos SET project = $1, task = $2, task_priority = $3
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
            SELECT todo_id, project, task, task_priority, created_at, time_spent FROM todos
            WHERE todo_id = $1
            LIMIT 1
        ",
        todo_id
    )
    .fetch_one(&pool)
    .await
    .context("Error fetching todo")?;

    let date = Date {
        month: todo.created_at.month(),
        day: todo.created_at.day(),
        year: todo.created_at.year(),
    };

    Ok(PendingTodoTemplate { todo, date })
}


pub async fn delete_todo(State(pool): State<PgPool>, Json(data): Json<TodoId>) -> Result<&'static str, AppError> {
    let todo_id = &data.id.parse::<i32>().unwrap();
    sqlx::query!(r#"DELETE FROM todos WHERE todo_id = $1"#, todo_id)
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
            SELECT todo_id, project, task, task_priority, created_at, time_spent FROM todos
                WHERE todo_id = $1
        ",
        todo_id
    )
    .fetch_one(&pool)
    .await
    .context("Error fetching reverted todo from database")?;

    let date = Date {
        month: todo.created_at.month(),
        day: todo.created_at.day(),
        year: todo.created_at.year(),
    };

    sqlx::query!("UPDATE todos SET completed = false, completed_at = to_timestamp(0) WHERE todo_id = $1", todo_id)
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
    sqlx::query!(r#"UPDATE todos SET is_active = true WHERE todo_id = $1"#, todo_id)
    .execute(&pool)
    .await
    .context("Error activating todo in database")?;
    
    let active_todo = sqlx::query_as!(
        ActiveTodoTemplate,
        "
            SELECT todo_id, project, task, task_priority FROM todos
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
    let time_spent = data.duration.parse::<i32>().expect("Could not parse active todo duration");
    
    sqlx::query!(r#"UPDATE todos SET is_active = false, time_spent = time_spent + $2 WHERE todo_id = $1"#, todo_id, time_spent)
    .execute(&pool)
    .await
    .context("Error updating is_active to false in database")?;

    let todo = sqlx::query_as!(
        PendingTodo,
        "
        SELECT todo_id, project, task, task_priority, created_at, time_spent FROM todos
            WHERE todo_id = $1
    ",
    todo_id
    )
    .fetch_one(&pool)
    .await
    .context("Error fetching pending todo from database")?;

    let date = Date {
        month: todo.created_at.month(),
        day: todo.created_at.day(),
        year: todo.created_at.year(),
    };

    let todo = PendingTodoTemplate { todo, date };
    Ok(DeactivateTodoTemplate {todo_id, todo })
}