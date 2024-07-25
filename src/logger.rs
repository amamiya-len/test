use log::{info, error, debug, LevelFilter};
use env_logger::{Builder, Env};
use std::io::Write;
use chrono;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use serde::{Serialize, Deserialize};



pub fn initialize_logging() -> i32 {
    let env = Env::default().filter_or("MY_LOG_LEVEL", "info");
    Builder::from_env(env)
        .format(|buf, record| {
            writeln!(
                buf,
                "{} [{}] - {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                record.args()
            )
        })
        .init();
	1
}

pub fn log_info(message: &str) {
    info!("{}", message);
}

pub fn log_error(message: &str) {
    error!("{}", message);
}

pub fn log_debug(message: &str) {
    debug!("{}", message);
}



#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LogEntry {
    term: i32,
    data: String,
}

impl LogEntry {
    pub fn new(term: i32, data: String) -> Self {
        LogEntry { term, data }
    }

    pub fn get_term(&self) -> i32 {
        self.term
    }

    pub fn get_data(&self) -> &str {
        &self.data
    }
}

#[derive(Default)]
pub struct InMemoryLogStore {
    logs: RwLock<HashMap<i32, Arc<LogEntry>>>,
}

impl InMemoryLogStore {
    pub fn new() -> Self {
        InMemoryLogStore {
            logs: RwLock::new(HashMap::new()),
        }
    }

    pub fn append(&self, index: i32, entry: Arc<LogEntry>) {
        let mut logs = self.logs.write().unwrap();
        logs.insert(index, entry);
    }

    pub fn get(&self, index: i32) -> Option<Arc<LogEntry>> {
        let logs = self.logs.read().unwrap();
        logs.get(&index).cloned()
    }

    pub fn remove(&self, index: i32) {
        let mut logs = self.logs.write().unwrap();
        logs.remove(&index);
    }

    pub fn size(&self) -> usize {
        let logs = self.logs.read().unwrap();
        logs.len()
    }
}
