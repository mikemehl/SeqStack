/// Module with the central vm structures.
use crate::stk::Stack;

const RAM_SIZE : usize = 1 << 15;
const NUM_INTERRUPTS : usize = 8;
const NUM_PORTS : usize = NUM_INTERRUPTS;
const INVALID_INTERRUPT : i16 = -1;

struct Vm {
    ram : Box<[u8]>,
    pc : usize,
    data_stack : Box<Stack>,
    call_stack : Box<Stack>,
    interrupts : Box<[i16]>,
    ports : Box<[Stack]>,
}

impl Vm {
    pub fn new() -> Box<Vm> {
        let ram = vec![0; RAM_SIZE];
        let interrupts = vec![INVALID_INTERRUPT; NUM_INTERRUPTS];
        let stk_basis = Stack::new();
        let mut ports = Vec::<Stack>::with_capacity(NUM_PORTS);
        for _i in 0..NUM_PORTS {
            ports.push(stk_basis);
        }
        let data_stack = Box::new(stk_basis);
        let call_stack = Box::new(stk_basis);
        Box::new(Vm {
            ram : ram.into_boxed_slice(), 
            pc : 0,
            data_stack : data_stack,
            call_stack : call_stack,
            interrupts : interrupts.into_boxed_slice(),
            ports : ports.into_boxed_slice(),
        })
    }

    pub fn load(&mut self, code_in : &[u8]) -> bool {
        if code_in.len() > self.ram.len() {
            return false;
        }
        self.ram.clone_from_slice(code_in);
        true
    }
} 



#[cfg(test)]
mod test {
    use super::*;

    fn init_vm() -> Box<Vm> {
        let vm = Vm::new();
        assert!(vm.pc == 0);
        assert!(vm.data_stack.empty());
        assert!(vm.call_stack.empty());
        for p in vm.ports.iter() {
            assert!(p.empty());
        }
        for i in vm.interrupts.iter() {
            assert_eq!(*i, INVALID_INTERRUPT);
        }
        vm
    }
    
    #[test]
    fn test_init() {
        let _vm = init_vm();
    }

   #[test]
   fn test_load() {
       const TEST_VAL : u8 = 66;
       let mut vm = init_vm();
       let code : [u8; RAM_SIZE] = [TEST_VAL; RAM_SIZE];
       assert!(vm.load(&code));
       for b in vm.ram.iter() {
           assert_eq!(*b, TEST_VAL);
       }
   }
}
