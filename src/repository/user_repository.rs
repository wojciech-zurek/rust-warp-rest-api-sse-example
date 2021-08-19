use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::Mutex;
use uuid::Uuid;
use warp::Filter;

use crate::models::user::User;

pub type Database = Arc<Mutex<UserRepository>>;

pub fn create_db() -> Database {
    Arc::new(Mutex::new(UserRepository::new()))
}

pub fn with_db(db: Database) -> impl Filter<Extract=(Database, ), Error=std::convert::Infallible> + Clone {
    warp::any().map(move || db.clone())
}

pub struct UserRepository {
    inner: HashMap<Uuid, User>,
}

impl UserRepository {
    pub fn new() -> Self {
        let mut inner = HashMap::new();
        inner.insert(Uuid::new_v4(), User::new("Wojtek".to_owned(), 30));
        UserRepository {
            inner
        }
    }

    pub fn exists(&self, key: &Uuid) -> bool {
        self.inner.contains_key(key)
    }

    pub fn find_all(&self) -> &HashMap<Uuid, User> {
        &self.inner
    }

    pub fn find_by_id(&self, key: &Uuid) -> Option<&User> {
        self.inner.get(key)
    }

    pub fn insert(&mut self, key: Uuid, value: User) {
        self.inner.insert(key, value);
    }

    pub fn update(&mut self, key: Uuid, value: User) {
        self.insert(key, value);
    }

    pub fn delete(&mut self, key: &Uuid) {
        self.inner.remove(key);
    }
}