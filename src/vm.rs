/// Module with the central vm structures.
use crate::stk::Stack;

const RAM_SIZE : usize = 1 << 15;
const NUM_INTERRUPTS : usize = 8;
const NUM_PORTS : usize = NUM_INTERRUPTS;
const INVALID_INTERRUPT : i16 = -1;

struct Vm {
    ram : Box<[u8]>,
    pc : usize,
    data_stack : Box<[Stack]>,
    call_stack : Box<[Stack]>,
    interrupts : Box<[i16]>,
    ports : Box<[Stack]>,
}

impl Vm {
    pub fn new() -> Box<Vm> {
        Box::new(Vm {
            ram : vec![0; RAM_SIZE].into_boxed_slice(),
            pc : 0,
            data_stack : vec![Stack::new(); 1].into_boxed_slice(),
            call_stack : vec![Stack::new(); 1].into_boxed_slice(),
            interrupts : vec![INVALID_INTERRUPT; NUM_INTERRUPTS].into_boxed_slice(),
            ports : vec![Stack::new(); NUM_PORTS].into_boxed_slice(),
        })
    }
} 



#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn test_init() {
        let vm = Vm::new();
        assert!(vm.pc == 0);
        assert!(vm.data_stack[0].empty());
        assert!(vm.call_stack[0].empty());
        for p in vm.ports.iter() {
            assert!(p.empty());
        }
        for i in vm.interrupts.iter() {
            assert_eq!(*i, INVALID_INTERRUPT);
        }
    }
}
