//!Implementation of [`TaskManager`]
use super::TaskControlBlock;
use crate::config::BIGSTRIDE;
use crate::sync::UPSafeCell;
use alloc::collections::VecDeque;
use alloc::sync::Arc;
use lazy_static::*;
///A array of `TaskControlBlock` that is thread-safe
pub struct TaskManager {
    ready_queue: VecDeque<Arc<TaskControlBlock>>,
}

/// A simple FIFO scheduler.
impl TaskManager {
    ///Creat an empty TaskManager
    pub fn new() -> Self {
        Self {
            ready_queue: VecDeque::new(),
        }
    }
    /// Add process back to ready queue
    pub fn add(&mut self, task: Arc<TaskControlBlock>) {
        self.ready_queue.push_back(task);
    }
    /// Take a process out of the ready queue (stride scheduling)
    pub fn fetch(&mut self) -> Option<Arc<TaskControlBlock>> {
        if self.ready_queue.is_empty() {
            return None;
        }
        let min_task_index = self.ready_queue.iter().enumerate().min_by_key(|(_,x)|x.inner_exclusive_access().get_stride()).map(|(x,_)|x).unwrap();
        let min_task = self.ready_queue.remove(min_task_index).unwrap();
        // stride += pass
        
        let inner = min_task.inner_exclusive_access();
        let stride = inner.get_stride();
        let pass = BIGSTRIDE / inner.get_priority();
        // manual drop inner , otherwise inner has two mut borrow (the second is in set_stride) (linted by ChatGPT)
        drop(inner);
        min_task.set_stride(stride + pass);
        Some(min_task.clone())
    }
}

lazy_static! {
    /// TASK_MANAGER instance through lazy_static!
    pub static ref TASK_MANAGER: UPSafeCell<TaskManager> =
        unsafe { UPSafeCell::new(TaskManager::new()) };
}

/// Add process to ready queue
pub fn add_task(task: Arc<TaskControlBlock>) {
    //trace!("kernel: TaskManager::add_task");
    TASK_MANAGER.exclusive_access().add(task);
}

/// Take a process out of the ready queue
pub fn fetch_task() -> Option<Arc<TaskControlBlock>> {
    //trace!("kernel: TaskManager::fetch_task");
    TASK_MANAGER.exclusive_access().fetch()
}
