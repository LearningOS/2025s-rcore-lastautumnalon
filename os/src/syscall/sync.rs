use crate::sync::{Condvar, Mutex, MutexBlocking, MutexSpin, Semaphore};
use crate::task::{block_current_and_run_next, current_process, current_task};
use crate::timer::{add_timer, get_time_ms};
use alloc::sync::Arc;
use alloc::vec::Vec;
/// sleep syscall
pub fn sys_sleep(ms: usize) -> isize {
    trace!(
        "kernel:pid[{}] tid[{}] sys_sleep",
        current_task().unwrap().process.upgrade().unwrap().getpid(),
        current_task()
            .unwrap()
            .inner_exclusive_access()
            .res
            .as_ref()
            .unwrap()
            .tid
    );
    let expire_ms = get_time_ms() + ms;
    let task = current_task().unwrap();
    add_timer(expire_ms, task);
    block_current_and_run_next();
    0
}
/// mutex create syscall
pub fn sys_mutex_create(blocking: bool) -> isize {
    trace!(
        "kernel:pid[{}] tid[{}] sys_mutex_create",
        current_task().unwrap().process.upgrade().unwrap().getpid(),
        current_task()
            .unwrap()
            .inner_exclusive_access()
            .res
            .as_ref()
            .unwrap()
            .tid
    );
    let process = current_process();
    let mutex: Option<Arc<dyn Mutex>> = if !blocking {
        Some(Arc::new(MutexSpin::new()))
    } else {
        Some(Arc::new(MutexBlocking::new()))
    };
    let mut process_inner = process.inner_exclusive_access();
    if let Some(id) = process_inner
        .mutex_list
        .iter()
        .enumerate()
        .find(|(_, item)| item.is_none())
        .map(|(id, _)| id)
    {
        process_inner.mutex_list[id] = mutex;
        id as isize
    } else {
        process_inner.mutex_list.push(mutex);
        process_inner.mutex_list.len() as isize - 1
    }
}
/// mutex lock syscall
pub fn sys_mutex_lock(mutex_id: usize) -> isize {
    trace!(
        "kernel:pid[{}] tid[{}] sys_mutex_lock",
        current_task().unwrap().process.upgrade().unwrap().getpid(),
        current_task()
            .unwrap()
            .inner_exclusive_access()
            .res
            .as_ref()
            .unwrap()
            .tid
    );
    let process = current_process();
    let mut process_inner = process.inner_exclusive_access();
    let mutex = Arc::clone(process_inner.mutex_list[mutex_id].as_ref().unwrap());
    if process_inner.detect {
    let tid = current_task()
        .unwrap()
        .inner_exclusive_access()
        .res
        .as_ref()
        .unwrap()
        .tid;
    for i in 0..process_inner.tasks.len() {
        while process_inner.alloc_matrix_mutex.len() < i + 1{
            process_inner.alloc_matrix_mutex.push(Vec::new());
        }
        while process_inner.alloc_matrix_mutex[i].len() < process_inner.mutex_list.len(){
            process_inner.alloc_matrix_mutex[i].push(0);
        }
        while process_inner.need_matrix_mutex.len() < i + 1{
            process_inner.need_matrix_mutex.push(Vec::new());
        }
        while process_inner.need_matrix_mutex[i].len() < process_inner.mutex_list.len() {
            process_inner.need_matrix_mutex[i].push(0);
        }
    }
    if mutex.available() > 0 {
        process_inner.alloc_matrix_mutex[tid][mutex_id] += 1;
    } else {
        process_inner.need_matrix_mutex[tid][mutex_id] += 1;
    }

    // deadlock algorithm
    let mut work: Vec<usize> = process_inner
        .mutex_list
        .iter()
        .map(|x: &Option<Arc<dyn Mutex>>| x.as_ref().unwrap().available())
        .collect();
    let allocation = &process_inner.alloc_matrix_mutex;
    let need = &process_inner.need_matrix_mutex;
    let mut finish: Vec<bool> = Vec::new();
    for _i in 0..process_inner.tasks.len() {
        finish.push(false);
    }
    let mut changed = true;
    while changed {
        changed = false;
        for i in 0..finish.len() {
            if !finish[i] {
                let can_finish = (0..work.len()).all(|j| need[i][j] <= work[j]);
                if can_finish {
                    for j in 0..work.len() {
                        work[j] += allocation[i][j];
                    }
                    finish[i] = true;
                    changed = true;
                }
            }
        }
    }
    if finish.iter().any(|&f| !f) {
        drop(process_inner);
        drop(process);
        return -0xDEAD;
    }
}
    drop(process_inner);
    drop(process);
    mutex.lock();
    0
}
/// mutex unlock syscall
pub fn sys_mutex_unlock(mutex_id: usize) -> isize {
    trace!(
        "kernel:pid[{}] tid[{}] sys_mutex_unlock",
        current_task().unwrap().process.upgrade().unwrap().getpid(),
        current_task()
            .unwrap()
            .inner_exclusive_access()
            .res
            .as_ref()
            .unwrap()
            .tid
    );
    let process = current_process();
    let mut process_inner = process.inner_exclusive_access();
    let mutex = Arc::clone(process_inner.mutex_list[mutex_id].as_ref().unwrap());
    if process_inner.detect {
    let tid = current_task()
        .unwrap()
        .inner_exclusive_access()
        .res
        .as_ref()
        .unwrap()
        .tid;
    process_inner.alloc_matrix_mutex[tid][mutex_id] -= 1;
    }
    drop(process_inner);
    drop(process);
    mutex.unlock();
    0
}
/// semaphore create syscall
pub fn sys_semaphore_create(res_count: usize) -> isize {
    trace!(
        "kernel:pid[{}] tid[{}] sys_semaphore_create",
        current_task().unwrap().process.upgrade().unwrap().getpid(),
        current_task()
            .unwrap()
            .inner_exclusive_access()
            .res
            .as_ref()
            .unwrap()
            .tid
    );
    let process = current_process();
    let mut process_inner = process.inner_exclusive_access();
    let id = if let Some(id) = process_inner
        .semaphore_list
        .iter()
        .enumerate()
        .find(|(_, item)| item.is_none())
        .map(|(id, _)| id)
    {
        process_inner.semaphore_list[id] = Some(Arc::new(Semaphore::new(res_count)));
        id
    } else {
        process_inner
            .semaphore_list
            .push(Some(Arc::new(Semaphore::new(res_count))));
        process_inner.semaphore_list.len() - 1
    };
    id as isize
}
/// semaphore up syscall
pub fn sys_semaphore_up(sem_id: usize) -> isize {
    trace!(
        "kernel:pid[{}] tid[{}] sys_semaphore_up",
        current_task().unwrap().process.upgrade().unwrap().getpid(),
        current_task()
            .unwrap()
            .inner_exclusive_access()
            .res
            .as_ref()
            .unwrap()
            .tid
    );
    let process = current_process();
    let process_inner = process.inner_exclusive_access();
    let sem = Arc::clone(process_inner.semaphore_list[sem_id].as_ref().unwrap());
    drop(process_inner);
    sem.up();
    0
}
/// semaphore down syscall
pub fn sys_semaphore_down(sem_id: usize) -> isize {
    trace!(
        "kernel:pid[{}] tid[{}] sys_semaphore_down",
        current_task().unwrap().process.upgrade().unwrap().getpid(),
        current_task()
            .unwrap()
            .inner_exclusive_access()
            .res
            .as_ref()
            .unwrap()
            .tid
    );
    let process = current_process();
    let mut process_inner = process.inner_exclusive_access();
    let sem = Arc::clone(process_inner.semaphore_list[sem_id].as_ref().unwrap());
    if process_inner.detect {
    let tid = current_task()
        .unwrap()
        .inner_exclusive_access()
        .res
        .as_ref()
        .unwrap()
        .tid;
    for i in 0..process_inner.tasks.len() {
        while process_inner.alloc_matrix_sem.len() < i + 1 {
            process_inner.alloc_matrix_sem.push(Vec::new());
        }
        while process_inner.alloc_matrix_sem[i].len() < process_inner.semaphore_list.len() {
            process_inner.alloc_matrix_sem[i].push(0);
        }
        while process_inner.need_matrix_sem.len() < i + 1 {
            process_inner.need_matrix_sem.push(Vec::new());
        }
        while process_inner.need_matrix_sem[i].len() < process_inner.semaphore_list.len() {
            process_inner.need_matrix_sem[i].push(0);
        }
    }

    if sem.available() > 0 {
        process_inner.alloc_matrix_sem[tid][sem_id] += 1;
    } else {
        process_inner.need_matrix_sem[tid][sem_id] += 1;
    }

    // deadlock algorithm
    let mut work: Vec<usize> = process_inner
        .semaphore_list
        .iter()
        .map(|x: &Option<Arc<Semaphore>>| x.as_ref().unwrap().available())
        .collect();
    let allocation = &process_inner.alloc_matrix_sem;
    let need = &process_inner.need_matrix_sem;
    let mut finish: Vec<bool> = Vec::new();
    for _i in 0..process_inner.tasks.len() {
        finish.push(false);
    }
    let mut changed = true;

    while changed {
        changed = false;
        for i in 0..finish.len() {
            if !finish[i] {
                let can_finish = (0..work.len()).all(|j| need[i][j] <= work[j]);
                if can_finish {
                    for j in 0..work.len() {
                        work[j] += allocation[i][j];
                    }
                    finish[i] = true;
                    changed = true;
                }
            }
        }
    }
    if finish.iter().any(|&f| !f) {
        drop(process_inner);
        return -0xDEAD;
    }
    }
    drop(process_inner);
    sem.down();
    0
}
/// condvar create syscall
pub fn sys_condvar_create() -> isize {
    trace!(
        "kernel:pid[{}] tid[{}] sys_condvar_create",
        current_task().unwrap().process.upgrade().unwrap().getpid(),
        current_task()
            .unwrap()
            .inner_exclusive_access()
            .res
            .as_ref()
            .unwrap()
            .tid
    );
    let process = current_process();
    let mut process_inner = process.inner_exclusive_access();
    let id = if let Some(id) = process_inner
        .condvar_list
        .iter()
        .enumerate()
        .find(|(_, item)| item.is_none())
        .map(|(id, _)| id)
    {
        process_inner.condvar_list[id] = Some(Arc::new(Condvar::new()));
        id
    } else {
        process_inner
            .condvar_list
            .push(Some(Arc::new(Condvar::new())));
        process_inner.condvar_list.len() - 1
    };
    id as isize
}
/// condvar signal syscall
pub fn sys_condvar_signal(condvar_id: usize) -> isize {
    trace!(
        "kernel:pid[{}] tid[{}] sys_condvar_signal",
        current_task().unwrap().process.upgrade().unwrap().getpid(),
        current_task()
            .unwrap()
            .inner_exclusive_access()
            .res
            .as_ref()
            .unwrap()
            .tid
    );
    let process = current_process();
    let process_inner = process.inner_exclusive_access();
    let condvar = Arc::clone(process_inner.condvar_list[condvar_id].as_ref().unwrap());
    drop(process_inner);
    condvar.signal();
    0
}
/// condvar wait syscall
pub fn sys_condvar_wait(condvar_id: usize, mutex_id: usize) -> isize {
    trace!(
        "kernel:pid[{}] tid[{}] sys_condvar_wait",
        current_task().unwrap().process.upgrade().unwrap().getpid(),
        current_task()
            .unwrap()
            .inner_exclusive_access()
            .res
            .as_ref()
            .unwrap()
            .tid
    );
    let process = current_process();
    let process_inner = process.inner_exclusive_access();
    let condvar = Arc::clone(process_inner.condvar_list[condvar_id].as_ref().unwrap());
    let mutex = Arc::clone(process_inner.mutex_list[mutex_id].as_ref().unwrap());
    drop(process_inner);
    condvar.wait(mutex);
    0
}
/// enable deadlock detection syscall
///
/// YOUR JOB: Implement deadlock detection, but might not all in this syscall
pub fn sys_enable_deadlock_detect(_enabled: usize) -> isize {
    trace!("kernel: sys_enable_deadlock_detect NOT IMPLEMENTED");
    if _enabled == 1 {
        current_process().inner_exclusive_access().detect = true;
        0
    } else if _enabled == 0 {
        current_process().inner_exclusive_access().detect = false;
        0
    } else {
        -1
    }
}
