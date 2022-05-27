use std::{
    collections::HashMap,
    net::{IpAddr, SocketAddr},
    sync::{
        atomic::{AtomicU32, Ordering},
        Arc, Mutex,
    },
};

use tokio::net::TcpListener;

use crate::{database::PgPool, game::session_manager::SessionManager};

type ConnCounter = Arc<AtomicU32>;

type ConnTracker = Arc<Mutex<HashMap<IpAddr, u8>>>;

#[derive(Debug)]
pub struct Server {
    listener: TcpListener,
    conn_count: ConnCounter,
    conn_tracker: ConnTracker,
    max_conns_per_ip: u8,
}

impl Server {
    pub async fn new(addr: SocketAddr, max_conns_per_ip: u8) -> Self {
        let listener = TcpListener::bind(addr)
            .await
            .expect("Unable to start server");

        let conn_tracker = Arc::new(Mutex::new(HashMap::new()));

        Self {
            listener,
            conn_count: Arc::new(AtomicU32::new(0)),
            conn_tracker,
            max_conns_per_ip,
        }
    }

    pub async fn run(&self, session_manager: Arc<SessionManager>, pool: Arc<PgPool>) {
        loop {
            let session_manager = session_manager.clone();
            let pool = pool.clone();

            if let Ok((connection, peer)) = self.listener.accept().await {
                log::info!("received connection from {}", peer.ip().to_string());

                let conn_tracker = Arc::clone(&self.conn_tracker);
                let conn_count = Arc::clone(&self.conn_count);

                let success = self.add_conn(peer.ip(), &conn_tracker, &conn_count);

                if !success {
                    log::info!(
                        "too many existing connections from {}",
                        peer.ip().to_string()
                    );

                    drop(connection);

                    continue; // Effectively an early return avoiding the below block
                }

                if let Err(err) = connection.set_nodelay(true) {
                    log::debug!(
                        "unable to set nodelay for {} reason {}",
                        peer.ip().to_string(),
                        err
                    );
                }

                tokio::spawn(async move {
                    session_manager
                        .create_and_run_session(connection, pool, peer.ip().to_string())
                        .await;

                    log::info!("received disconnect from {}", peer.ip().to_string());

                    Self::remove_conn(peer.ip(), &conn_tracker, &conn_count);
                });
            }
        }
    }

    fn add_conn(&self, addr: IpAddr, conn_track: &ConnTracker, conn_count: &ConnCounter) -> bool {
        if let Ok(mut conn_map) = conn_track.lock() {
            conn_map
                .entry(addr)
                .and_modify(|entry| *entry += 1)
                .or_insert(1);

            conn_count.fetch_add(1, Ordering::Relaxed);

            return true;
        }

        false
    }

    fn remove_conn(addr: IpAddr, conn_track: &ConnTracker, conn_count: &ConnCounter) -> bool {
        if let Ok(mut conn_map) = conn_track.lock() {
            conn_map.entry(addr).and_modify(|entry| *entry -= 1);

            conn_count.fetch_sub(1, Ordering::Relaxed);

            return true;
        }

        false
    }
}
