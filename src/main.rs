mod wlroost;
use std::rc::Rc;
use std::boxed::Box;
use crate::wlroost::{CallbackScope, Handle};

fn main() {
    //creating a handle. the handle is a weak ref to a mut pointer to a u32..
    let real_output = Rc::new(Box::into_raw(Box::new(32u32)));
    let output_handle = unsafe{Handle::new(Rc::downgrade(&real_output))};

    // my callback captures output_handle
    let my_callback = |scope: &CallbackScope, argument: u32| -> u32 {
        if let Some(ptr) = output_handle.upgrade(scope){
            argument+unsafe{**ptr}
        }else{
            0
        }
    };

    //pretend we're in the wlroots wrapper, calling a callback.
    let scope_raw = unsafe{CallbackScope::new()};
    let scope = &scope_raw;

    println!("Calling callback on 32, while changing the handle's state");
    println!("1. cb(32) = `{}` Handle has initial value of 32", my_callback(scope, 32));

    unsafe{**real_output = 31;}
    println!("2. cb(32) = `{}` after modifying handle value to 31", my_callback(scope, 32));

    // freeing the *mut u32. wlroots_free_output(real_*output)
    unsafe{
        drop(Box::from_raw(*real_output));
    }
    drop(real_output);
    println!("3. cb(32) = `{}` after dropping the 'output'", my_callback(scope, 32));
}
