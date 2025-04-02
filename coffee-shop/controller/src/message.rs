use nostr::EventId;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[repr(u8)]
pub enum CoffeeShopStatus {
    Available = 1,
    Working = 2,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[repr(u8)]
pub enum CoffeeShopStatusReason {
    UserRequest = 1,
    AdminRequest = 2,
    UserBehaviour = 3,
    Reset = 4,
    LockFailed = 5,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CoffeeShopMessage {
    Request {
        to_status: CoffeeShopStatus,
        reason: CoffeeShopStatusReason,
        initial_request: EventId,
        payload: String,
    },
    Status {
        status: CoffeeShopStatus,
        reason: CoffeeShopStatusReason,
        initial_request: EventId,
        payload: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoffeeShopMessageRequestPayload {
    pub user: String,
    pub recipe_id: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoffeeShopMessageStatusPayload {
    pub user: String,
    pub recipe_id: u64,
}
