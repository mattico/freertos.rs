//! # FreeRTOS for Rust
//!
//! Rust interface for the FreeRTOS embedded operating system. Requires nightly Rust.
//! It is assumed that dynamic memory allocation is provided on the target system.
//!
//! This library interfaces with FreeRTOS using a C shim library which provides function
//! wrappers for FreeRTOS macros. The compiled Rust application should be linked to the
//! base C/C++ firmware binary. Check the subdirectory ``shim``. Copy the source file to
//! your firmware's sources directory and modify it to include the appropriate headers for
//! target your system.
//!
//! For a complete example, check the enclosed GCC ARM/Rust/QEMU based unit tests. The Rust
//! cargo crate [xargo](https://crates.io/crates/xargo) is used for cross-compilation. The project
//! ``qemu_runner`` cross-compiles this library, compiles the main firmware using GCC ARM and links
//! in the appropriate entry points for unit tests. [GNU ARM Eclipse QEMU](http://gnuarmeclipse.github.io/qemu/)
//! is used to run the test binaries.
//!
//! Be sure to check the [FreeRTOS documentation](http://www.freertos.org/RTOS.html).
//!
//! # Samples
//!
//! Spawning a new task
//!
//! ```rust
//! # use freertos_rs::*;
//! Task::new().name("hello").stack_size(128).start(|| {
//! 	loop {
//! 		println!("Hello world!");
//! 		CurrentTask::delay(Duration::infinite());
//! 	}
//! }).unwrap();
//! ```
//!
//! Queue
//!
//! ```rust
//! # use freertos_rs::*;
//! let q = Queue::new(10).unwrap();
//! q.send(10, Duration::ms(5)).unwrap();
//! q.receive(Duration::infinite()).unwrap();
//! ```
//!
//! Mutex
//!
//! ```rust
//! # use freertos_rs::*;
//! let m = Mutex::new(0).unwrap();
//! {
//! 	let mut v = m.lock(Duration::infinite()).unwrap();
//! 	*v += 1;
//! }
//! ```

#![no_std]

#![feature(alloc)]
#![feature(collections)]
#![feature(fnbox)]

extern crate alloc;
extern crate collections;




mod prelude;
mod shim;

mod base;
mod task;
mod compute_task;
mod queue;
mod semaphore;
mod mutex;
mod units;
mod utils;
mod isr;
mod pub_sub;
mod delays;


pub use base::FreeRtosError;
pub use task::*;
pub use compute_task::*;
pub use queue::*;
pub use units::*;
pub use mutex::*;
pub use semaphore::*;
pub use pub_sub::*;
pub use isr::*;
pub use delays::*;

pub use utils::shim_sanity_check;
