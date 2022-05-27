use std::sync::Arc;

use crate::database::PgPool;
use dashmap::DashMap;
use tokio::net::TcpStream;

use super::session::Session;

pub struct SessionManager {
    sessions: DashMap<String, Arc<Box<Session>>>,
}

impl SessionManager {
    pub fn new() -> Self {
        Self {
            sessions: DashMap::with_capacity(500),
        }
    }

    pub fn add_session<T: AsRef<str>>(&self, id: T, session: Arc<Box<Session>>) {
        self.sessions.insert(String::from(id.as_ref()), session);
    }

    pub fn remove_session<T: AsRef<str>>(&self, id: T) -> bool {
        let result = self.sessions.remove(id.as_ref());

        result.is_some()
    }

    pub async fn create_and_run_session(
        &self,
        connection: TcpStream,
        pool: Arc<PgPool>,
        ip_address: String,
    ) {
        let session = Arc::new(Box::new(Session::new(connection, pool, ip_address)));

        let session_id = session.get_id();

        self.add_session(&session_id, session.clone());

        session.run(self).await;

        self.remove_session(&session_id);
    }
}

impl Default for SessionManager {
    fn default() -> Self {
        Self::new()
    }
}
