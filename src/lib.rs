//! A simple stack based virtual machine with input and output through....more stacks.

// TODO: Remove this directive and cleanup once stable.
#![allow(dead_code)]
#![allow(unused_imports)]

mod fp;
mod stk;
mod vm;

#[cfg(test)]
mod test {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
