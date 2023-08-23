use crate::ctx::Ctx;
use crate::model::{ModelController, Todo, TodoForCreate};
use crate::Result;
use axum::extract::Path;
use axum::routing::{delete, post};
use axum::Router;
use axum::{extract::State, Json};

async fn create_todo(
    State(mc): State<ModelController>,
    ctx: Ctx,
    Json(todo_fc): Json<TodoForCreate>,
) -> Result<Json<Todo>> {
    let todo = mc.create_todo(ctx, todo_fc).await?;

    Ok(Json(todo))
}

async fn get_todos(State(mc): State<ModelController>, ctx: Ctx) -> Result<Json<Vec<Todo>>> {
    let todos = mc.get_todos(ctx).await?;

    Ok(Json(todos))
}

async fn delete_todo(
    State(mc): State<ModelController>,
    ctx: Ctx,
    Path(id): Path<u64>,
) -> Result<Json<Todo>> {
    let todo = mc.delete_todo(ctx, id).await?;

    Ok(Json(todo))
}

pub fn routes(mc: ModelController) -> Router {
    Router::new()
        .route("/todos", post(create_todo).get(get_todos))
        .route("/todos/:id", delete(delete_todo))
        .with_state(mc)
}
