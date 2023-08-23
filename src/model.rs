use crate::{ctx::Ctx, Error, Result};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

// Todo Types
#[derive(Clone, Debug, Serialize)]
pub struct Todo {
    pub id: u64,
    pub creator_id: u64,
    pub content: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct TodoForCreate {
    pub content: String,
}

// Model Controller
#[derive(Clone)]
pub struct ModelController {
    todos_store: Arc<Mutex<Vec<Option<Todo>>>>,
}

impl ModelController {
    pub async fn new() -> Result<Self> {
        Ok(Self {
            todos_store: Arc::default(),
        })
    }

    pub async fn create_todo(&self, ctx: Ctx, todo_fc: TodoForCreate) -> Result<Todo> {
        let mut store = self.todos_store.lock().unwrap();

        let id = store.len() as u64;
        let todo = Todo {
            id,
            creator_id: ctx.get_user_id(),
            content: todo_fc.content,
        };

        store.push(Some(todo.clone()));

        Ok(todo)
    }

    pub async fn get_todos(&self, _ctx: Ctx) -> Result<Vec<Todo>> {
        let store = self.todos_store.lock().unwrap();

        let todos = store.iter().filter_map(|todo| todo.clone()).collect();

        Ok(todos)
    }

    pub async fn delete_todo(&self, _ctx: Ctx, id: u64) -> Result<Todo> {
        let mut store = self.todos_store.lock().unwrap();

        let todo = store.get_mut(id as usize).and_then(|todo| todo.take());

        todo.ok_or(Error::TodoDeleteFailIdNotFound { id })
    }
}
