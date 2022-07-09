//! Types related to task management

use super::TaskContext;

#[derive(Copy, Clone)]
/// task control block structure
pub struct TaskControlBlock {
    pub task_status: TaskStatus,
    pub task_cx: TaskContext,
    // LAB1: Add whatever you need about the Task.
    pub yield_count: u32,
    pub exit_count: u32,
    pub gettime_count: u32,
    pub write_count: u32,
    pub taskinfo_count: u32,
    pub init_time: usize,
    pub time: usize,
}

#[derive(Copy, Clone, PartialEq)]
/// task status: UnInit, Ready, Running, Exited
pub enum TaskStatus {
    UnInit,
    Ready,
    Running,
    Exited,
}
