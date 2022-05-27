use std::io;
use std::pin::Pin;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use bytes::BytesMut;
use futures::{Sink, SinkExt, Stream, StreamExt};

use crate::database::PgPool;
use tokio::net::TcpStream;
use tokio::sync::Mutex;
use tokio_util::codec::Framed;
use uuid::Uuid;

use crate::codec::frame::MessageFrame;
use crate::codec::net::NetCodec;
use crate::message::handle_message;

use super::session_manager::SessionManager;
use super::session_state::SessionState;
use super::user::User;

pub type SessionSink = dyn Sink<BytesMut, Error = io::Error> + Send + Sync;
pub type SessionStream = dyn Stream<Item = Result<MessageFrame, io::Error>> + Send + Sync;

pub struct Session {
    id: String,
    ip_address: String,
    is_shutdown: AtomicBool,
    user: Mutex<Option<User>>,
    state: Arc<SessionState>,
    pool: Arc<PgPool>,
    sink: Mutex<Pin<Box<SessionSink>>>,
    stream: Mutex<Pin<Box<SessionStream>>>,
    // connection: TcpStream,
}

impl Session {
    pub fn new(connection: TcpStream, pool: Arc<PgPool>, ip_address: String) -> Self {
        let (sink, stream) = Framed::new(connection, NetCodec::new()).split();

        Self {
            id: Uuid::new_v4().to_string(),
            ip_address,
            is_shutdown: AtomicBool::new(false),
            user: Mutex::new(None),
            state: Arc::new(SessionState::new()),
            pool,
            sink: Mutex::new(Box::pin(Box::new(sink))),
            stream: Mutex::new(Box::pin(Box::new(stream))),
        }
    }

    pub async fn run(&self, session_manager: &SessionManager) {
        let mut stream = self.stream.lock().await;

        while let Some(result) = stream.next().await {
            if self.get_is_shutdown() {
                break;
            }

            if let Ok(frame) = result {
                match frame {
                    MessageFrame::Policy => {
                        let mut sink = self.sink.lock().await;

                        sink
                        .send(BytesMut::from(&b"<?xml version=\"1.0\"?>\r\n<!DOCTYPE cross-domain-policy \\ SYSTEM \"/xml/dtds/cross-domain-policy.dtd\">\r\n<cross-domain-policy>\r\n<allow-access-from domain=\"*\" to-ports=\"*\" />\r\n</cross-domain-policy>\0"[..]))
                        .await
                        .unwrap();
                    }

                    MessageFrame::Packet(mut message) => {
                        if let Some(result) =
                            handle_message(&mut message, self, session_manager).await
                        {
                            let mut sink = self.sink.lock().await;

                            log::info!("handler sending {:?} to {}", &result, self.id);

                            let result = sink.send(result).await;

                            if result.is_err() {
                                log::debug!("{:?}", result);
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn get_id(&self) -> String {
        self.id.clone()
    }

    pub fn get_ip_address(&self) -> String {
        self.ip_address.clone()
    }

    pub fn get_is_shutdown(&self) -> bool {
        self.is_shutdown.load(Ordering::Relaxed)
    }

    pub fn get_state(&self) -> Arc<SessionState> {
        self.state.clone()
    }

    pub fn get_pool(&self) -> Arc<PgPool> {
        self.pool.clone()
    }

    pub async fn get_user(&self) -> Option<User> {
        let lock = self.user.lock().await;

        lock.as_ref().cloned()
    }

    pub async fn set_user(&self, user: User) -> Option<User> {
        let mut lock = self.user.lock().await;

        *lock = Some(user);

        lock.as_ref().cloned()
    }

    pub async fn send_message(&self, bytes: BytesMut) -> bool {
        let mut sink = self.sink.lock().await;

        let result = sink.send(bytes.clone()).await;

        log::info!("sent {:?} to client {}", &bytes, self.id);

        result.is_ok()
    }

    pub fn end_session(&mut self) {
        self.is_shutdown.swap(false, Ordering::Relaxed);
    }
}
