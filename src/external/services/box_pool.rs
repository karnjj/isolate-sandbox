use crate::domain::error::{DomainError, DomainResult};
use std::sync::Arc;
use tokio::sync::Mutex;
use std::collections::VecDeque;

pub struct BoxPool {
    available: Arc<Mutex<VecDeque<u32>>>,
    size: u32,
}

impl BoxPool {
    pub fn new(size: u32) -> Self {
        let mut available = VecDeque::new();
        for i in 0..size {
            available.push_back(i);
        }

        Self {
            available: Arc::new(Mutex::new(available)),
            size,
        }
    }

    pub async fn acquire(&self) -> DomainResult<u32> {
        let mut available = self.available.lock().await;
        available
            .pop_front()
            .ok_or(DomainError::BoxPoolExhausted)
    }

    pub async fn release(&self, box_id: u32) -> DomainResult<()> {
        if box_id >= self.size {
            return Err(DomainError::Internal(format!(
                "Invalid box ID: {}",
                box_id
            )));
        }

        let mut available = self.available.lock().await;
        available.push_back(box_id);
        Ok(())
    }
}

