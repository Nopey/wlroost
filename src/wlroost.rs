use std::rc::{Rc, Weak};

pub type WlrOutput = u32;

pub struct CallbackScope{
    _private: (),
}
impl CallbackScope{
    pub unsafe fn new() -> Self{
        CallbackScope{ _private: () }
    }
}

pub struct Handle{
    contents: Weak<*mut WlrOutput>,
}

impl Handle{
    pub unsafe fn new(contents: Weak<*mut WlrOutput>) -> Self{
        Self{ contents }
    }
    pub fn upgrade<'a>(&self, _scope: &'a CallbackScope) -> Option<&'a *mut WlrOutput>{
        self.contents
            .upgrade()
            .map(|rc|{
                let raw = Rc::into_raw(rc);

                // this prevents memory leak. Since we were able to upgrade,
                // the output will not be dropped until at least the end of 'a
                drop(unsafe{Rc::from_raw(raw)});

                // transmute to cast this function's local scope into 'a
                unsafe{std::mem::transmute(&*raw)}
            })
    }
}
