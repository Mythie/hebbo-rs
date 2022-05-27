use std::sync::atomic::{AtomicBool, AtomicI32, Ordering};

pub struct SessionState {
    current_room: AtomicI32,
    is_room_authenticated: AtomicBool,
    is_teleporting: AtomicBool,
}

impl SessionState {
    pub fn new() -> Self {
        Self {
            current_room: AtomicI32::new(-1),
            is_room_authenticated: AtomicBool::new(false),
            is_teleporting: AtomicBool::new(false),
        }
    }

    pub fn get_current_room(&self) -> i32 {
        self.current_room.load(Ordering::Relaxed)
    }

    pub fn set_current_room(&self, room_id: i32) {
        self.current_room.swap(room_id, Ordering::Relaxed);
    }

    pub fn reset_current_room(&self) {
        self.current_room.swap(-1, Ordering::Relaxed);
    }

    pub fn get_is_room_authenticated(&self) -> bool {
        self.is_room_authenticated.load(Ordering::Relaxed)
    }

    pub fn set_is_room_authenticated(&self, is_room_authenticated: bool) {
        self.is_room_authenticated
            .swap(is_room_authenticated, Ordering::Relaxed);
    }

    pub fn get_is_teleporting(&self) -> bool {
        self.is_teleporting.load(Ordering::Relaxed)
    }

    pub fn set_is_teleporting(&self, is_teleporting: bool) {
        self.is_teleporting.swap(is_teleporting, Ordering::Relaxed);
    }
}

impl Default for SessionState {
    fn default() -> Self {
        Self::new()
    }
}
