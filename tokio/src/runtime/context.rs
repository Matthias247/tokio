//! Thread local runtime context
use crate::runtime::{self, Handle, Spawner};
use std::cell::RefCell;

thread_local! {
    static CONTEXT: RefCell<Option<Handle>> = RefCell::new(None)
}

pub(crate) fn current() -> Option<Handle> {
    CONTEXT.with(|ctx| ctx.borrow().clone())
}

pub(crate) fn io_handle() -> runtime::io::Handle {
    CONTEXT.with(|ctx| match *ctx.borrow() {
        Some(ref ctx) => ctx.io_handle.clone(),
        None => Default::default(),
    })
}

pub(crate) fn time_handle() -> crate::runtime::time::Handle {
    CONTEXT.with(|ctx| match *ctx.borrow() {
        Some(ref ctx) => ctx.time_handle.clone(),
        None => Default::default(),
    })
}

pub(crate) fn spawn_handle() -> Option<Spawner> {
    CONTEXT.with(|ctx| match *ctx.borrow() {
        Some(ref ctx) => Some(ctx.spawner.clone()),
        None => None,
    })
}

pub(crate) fn clock() -> Option<crate::runtime::time::Clock> {
    CONTEXT.with(|ctx| match *ctx.borrow() {
        Some(ref ctx) => Some(ctx.clock.clone()),
        None => None,
    })
}

/// Set this [`ThreadContext`] as the current active [`ThreadContext`].
///
/// [`ThreadContext`]: struct.ThreadContext.html
pub(crate) fn enter<F, R>(new: Handle, f: F) -> R
where
    F: FnOnce() -> R,
{
    struct DropGuard(Option<Handle>);

    impl Drop for DropGuard {
        fn drop(&mut self) {
            CONTEXT.with(|ctx| {
                *ctx.borrow_mut() = self.0.take();
            });
        }
    }

    let _guard = CONTEXT.with(|ctx| {
        let old = ctx.borrow_mut().replace(new);
        DropGuard(old)
    });

    f()
}
