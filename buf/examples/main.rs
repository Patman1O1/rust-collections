use vec::Vec;
use allocator_api2::{
    alloc::{Global}
};

fn main() {
    let my_vec = Vec::<i32, Global>::new();
    my_vec.ptr();
}
