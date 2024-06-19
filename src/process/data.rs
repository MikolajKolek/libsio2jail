use std::ffi::{c_int, CString};
use std::os::fd::{BorrowedFd, OwnedFd};
use std::path::PathBuf;
use std::ptr::null;
use std::time::Duration;

use libc::c_char;
use nix::errno::Errno;

use crate::listener::Listener;
use crate::process::execution_result::ExecutionResult;
use crate::process::executor::Sio2jailExecutor;

#[derive(Debug)]
pub(crate) struct ExecutionContext<'a> {
	pub(crate) settings: ExecutionSettings<'a>,
	pub(crate) data: ExecutionData,
	pub(crate) listeners: Vec<Box<dyn Listener>>
}

#[allow(dead_code)]
#[readonly::make]
#[derive(Debug)]
pub(crate) struct ExecutionSettings<'a> {
	pub(crate) real_time_limit: Option<Duration>,
	pub(crate) instruction_count_limit: Option<i64>,
	pub(crate) executable_path: CString,
	pub(crate) args: Vec<CString>,
	pub(crate) args_ptr: Vec<*const c_char>,
	pub(crate) working_dir: PathBuf,
	pub(crate) stdin_fd: Option<BorrowedFd<'a>>,
	pub(crate) stdout_fd: Option<BorrowedFd<'a>>,
	pub(crate) stderr_fd: Option<BorrowedFd<'a>>
}

#[derive(Debug)]
pub(crate) struct ExecutionData {
	pub(crate) pid_fd: Option<OwnedFd>,
	pub(crate) pid: Option<c_int>,
	pub(crate) execution_result: ExecutionResult,
	pub(crate) child_error: Option<Errno>
}

impl ExecutionSettings<'_> {
	pub(crate) fn new(executor: Sio2jailExecutor) -> ExecutionSettings {
		let mut result = ExecutionSettings {
			real_time_limit: executor.real_time_limit,
			instruction_count_limit: executor.instruction_count_limit,
			executable_path: executor.executable_path,
			args: executor.args,
			args_ptr: vec![],
			working_dir: executor.working_dir,
			stdin_fd: executor.stdin_fd,
			stdout_fd: executor.stdout_fd,
			stderr_fd: executor.stderr_fd
		};
		
		for arg in &result.args {
			result.args_ptr.push(arg.as_ptr());
		}
		result.args_ptr.push(null());
		
		result
	}
}

impl ExecutionData {
	pub(crate) fn new() -> ExecutionData {
		ExecutionData {
			pid_fd: None,
			pid: None,
			execution_result: ExecutionResult::new(),
			child_error: None,
		}
	}
}