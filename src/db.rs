
use rusqlite::{Connection, Result};
use std::sync::{Arc, Mutex};
use once_cell::sync::Lazy;
use std::time::Duration;

pub static DB_CONN: Lazy<Arc<Mutex<Connection>>> = Lazy::new(|| {
    let conn = Connection::open("bank.db").expect("Failed to open database");
    
    // Configure connection
    conn.busy_timeout(Duration::from_secs(5))
        .expect("Failed to set busy timeout");
    conn.pragma_update(None, "journal_mode", &"WAL")
        .expect("Failed to enable WAL mode");
    conn.pragma_update(None, "synchronous", &"NORMAL")
        .expect("Failed to set synchronous mode");
    
    Arc::new(Mutex::new(conn))
});

pub fn execute_with_retry<T, F>(operation: F, max_retries: u32) -> Result<T>
where
    F: Fn() -> Result<T>,
{
    let mut last_error = None;
    
    for retry in 0..max_retries {
        match operation() {
            Ok(result) => return Ok(result),
            Err(err) => {
                if let rusqlite::Error::SqliteFailure(error, _) = &err {
                    if error.code == rusqlite::ErrorCode::DatabaseBusy {
                        std::thread::sleep(Duration::from_millis(100 * (retry + 1) as u64));
                        last_error = Some(err);
                        continue;
                    }
                }
                return Err(err);
            }
        }
    }
    
    Err(last_error.unwrap())
}


