mod mocked_blockchain;
mod mocked_memory;
mod receipt;

pub use mocked_blockchain::test_vm_config;

pub use self::mocked_blockchain::MockedBlockchain;
pub use self::receipt::{MockAction, Receipt};
use core::cell::RefCell;
use std::rc::Rc;

thread_local! {
    /// Low-level blockchain interface wrapped by the environment. Prefer using `env::*` and
    /// `testing_env` for interacting with the real and fake blockchains.
    static BLOCKCHAIN_INTERFACE: Rc<RefCell<MockedBlockchain>>
         = Rc::new(RefCell::new(MockedBlockchain::default()));
}

/// Perform function on a mutable reference to the [`MockedBlockchain`]. This can only be used
/// inside tests.
pub fn with_mocked_blockchain<F, R>(f: F) -> R
where
    F: FnOnce(&mut MockedBlockchain) -> R,
{
    BLOCKCHAIN_INTERFACE.with(|b| f(&mut b.borrow_mut()))
}
