//! A simple stack based virtual machine with input and output through....more stacks.

// TODO: Remove this directive and cleanup once stable.
#![allow(dead_code)]

mod fp;
mod stk;

#[cfg(test)]
mod test {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
