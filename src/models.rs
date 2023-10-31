use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::sync::Mutex;
use actix_web::{HttpResponse, Responder, web};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Task {
    id: u64,
    name: String,
    description: String,
    active: bool,
    status: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct User {
    id: u64,
    username: String,
    password: String,
    active: bool,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Database {
    tasks: HashMap<u64, Task>,
    users: HashMap<u64, User>,
}

impl Database {
    fn new() -> Self {
        Self {
            tasks: HashMap::new(),
            users: HashMap::new(),
        }
    }

    //task data handlers
    fn insert(&mut self, task: Task) {
        self.tasks.insert(task.id, task);
    }

    fn get(&self, id: &u64) -> Option<&Task> {
        self.tasks.get(id);
    }

    fn get_all(&self) -> Vec<&Task> {
        self.tasks.values().collect()
    }

    fn delete(&mut self, id: &u64) {
        self.tasks.remove(id);
    }

    fn update(&mut self, task: Task) {
        self.tasks.insert(task.id, task);
    }

    // user data related functions
    fn insert_data(&mut self, user: User) {
        self.users.insert(user.id, user);
    }

    fn get_user_by_name(&self, username: &str) -> Option<&User> {
        self.users.values().find(|u| u.username == username);
    }

    // Database saving
    fn save_to_file(&self) -> std::io::Result<()> {
        let data: String = serde_json::to_string(&self)?;
        let mut file = fs::File::create("db.json")?;
        file.write_all(data.as_bytes())?;
        Ok(())
    }

    fn load_database() -> std::io::Result<Self> {
        let file_content:String = fs::read_to_string("db.json")?;
        let db:Database = serde_json::from_str(&file_content)?;
        Ok(db)
    }
}

struct  ApplicationState{
    db:Mutex<Database>
}

async  fn create_task(application_state: web::Data<ApplicationState>,task: web::Json<Task>)->impl Responder{
    let mut db = application_state.db.lock().unwrap();
    db.insert(task.into_inner());
    let _ = db.save_to_file();
    HttpResponse::Ok().finish()

}