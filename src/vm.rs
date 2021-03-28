/// Module with the central vm structures.
use crate::stk::Stack;
use crate::fp;
use crate::opcodes::*;

const RAM_SIZE : usize = 1 << 15;
const NUM_INTERRUPTS : usize = 8;
const NUM_PORTS : usize = NUM_INTERRUPTS;
const INVALID_INTERRUPT : i16 = -1;

pub struct Vm {
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

    pub fn cycle_once(&mut self) {
         // Grab the next instruction.
         if self.pc >= RAM_SIZE {
             return;
         }
         let next_inst = self.ram[self.pc];
         self.pc += 1;
         // Figure out which group it belongs to.
         let fam : OpFamily = OpFamily::from(next_inst);
         match fam {
             OpFamily::StackOp => {stack_op_impl::cycle_op(self, next_inst); },
             _ => {},
         }
    }

} 

mod stack_op_impl {
    use crate::stk::Stack;
    use crate::fp;
    use crate::opcodes::*;
    pub (super) fn cycle_op(vm : &mut super::Vm, inst : u8) {
        let op_type = StackOpTypes::from(inst);
        let addr_mode = OpAddrMode::from(inst);
        match op_type {
            StackOpTypes::Push => op_push(vm, addr_mode),
            _ => {} 
        }
    }

    pub fn op_push(vm : &mut super::Vm, addr_mode : OpAddrMode) {
        let arg : Option<i32> = match addr_mode {
            OpAddrMode::Immediate => { 
                let mut val_arr : [u8; 4] = [0; 4];
                for i in 0..4 {
                   val_arr[i] = vm.ram[vm.pc + i]; 
                }
                let val = i32::from_ne_bytes(val_arr);
                vm.pc += 4;
                Some(val)
            },
            _ => None,
        };
        if let Some(p_val) = arg {
            vm.data_stack.push(p_val);
        }
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
       let code : [u8; RAM_SIZE + 10] = [TEST_VAL; RAM_SIZE + 10];
       assert!(!vm.load(&code));
   }

   #[test]
   fn test_push_op() {
       let test_val : i32 = fp::float_to_fix(66.0);
       let mut vm = init_vm();
       let mut code : [u8; RAM_SIZE] = [0; RAM_SIZE];
       // Push Immediate
       code[0] = OpCodes::PushImm as u8; 
       let val = test_val.to_ne_bytes();
       for i in 1..(1 + val.len()) {
           code[i] = val[i - 1];
       }
       assert!(vm.load(&code));
       vm.cycle_once();
       assert_eq!(vm.pc, 5, "Failed to increment program counter.");
       assert!(!vm.data_stack.empty(), "Data stack empty after push.");
       let top_val = vm.data_stack.peek();
       assert!(!top_val.is_none(), "Data stack peek returned None on a nonempty stack.");
       assert_eq!(top_val.unwrap(), test_val, "Data stack top was not expected value.");
       // End of the rope test (Push Immediate)
       code[RAM_SIZE-1] = OpCodes::PushImm as u8;
       vm.pc = RAM_SIZE-1;
       vm.cycle_once();
       assert_eq!(vm.pc, RAM_SIZE, "PC did not increment for last instruction."); 
       vm.cycle_once();
       assert_eq!(vm.pc, RAM_SIZE, "PC WAS MODIFIED AT END OF RAM INSTRUCTION");
      
   }
}
