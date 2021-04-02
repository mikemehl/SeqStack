/// Module with the central vm structures.

mod opcodes;
mod stack_op_impl;

use crate::stk::Stack;
use crate::fp;
use opcodes::*;

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
            data_stack,
            call_stack,
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

// Extracts the value in memory or on the stack based on the addressing mode. Increments the program counter if necessary.
// Used in *_op_impl modules.
fn get_addr_val(vm : &mut Vm, addr_mode : &OpAddrMode) -> Option<i32> {
    match addr_mode {
        OpAddrMode::Immediate => { 
            if vm.pc + 4 >= RAM_SIZE {
                return None;
            }
            let mut val_arr : [u8; 4] = [0; 4];
            val_arr[0..4].clone_from_slice(&vm.ram[vm.pc as usize..vm.pc as usize + 4]);
            let val = i32::from_ne_bytes(val_arr);
            vm.pc += 4;
            Some(val)
        },
        OpAddrMode::IndexStack => {
            if vm.pc + 2 >= RAM_SIZE {
                return None;
            }
            let arg : Option<i32>;
            let mut base_arr : [u8; 2] = [0; 2];
            base_arr[0..2].clone_from_slice(&vm.ram[vm.pc as usize..vm.pc as usize + 2]);
            let base = i16::from_ne_bytes(base_arr) as isize;
            if let Some(offset) = vm.data_stack.pop() {
                let off = offset >> 16;
                let val_addr : isize = base + off as isize;
                if val_addr > 0 && val_addr < RAM_SIZE as isize {
                    let mut val_arr : [u8; 4] = [0; 4];
                    val_arr[0..4].clone_from_slice(&vm.ram[val_addr as usize..val_addr as usize + 4]);
                    let val = i32::from_ne_bytes(val_arr);
                    arg = Some(val);
                } else {
                    arg = None;
                }
            } else {
                arg = None;
            }
            vm.pc += 2;
            arg
        },
        OpAddrMode::IndexImmediate => {
            if vm.pc + 2 >= RAM_SIZE {
                return None;
            }
            let arg : Option<i32>;
            let mut offset_arr : [u8; 2] = [0; 2];
            offset_arr[0..2].clone_from_slice(&vm.ram[vm.pc as usize..vm.pc as usize + 2]);
            let offset = i16::from_ne_bytes(offset_arr) as isize;
            if let Some(base) = vm.data_stack.pop() {
                let base = base >> 16;
                let val_addr : isize = base as isize + offset;
                if val_addr > 0 && val_addr < RAM_SIZE as isize {
                    let mut val_arr : [u8; 4] = [0; 4];
                    val_arr[0..4].clone_from_slice(&vm.ram[val_addr as usize..val_addr as usize + 4]);
                    let val = i32::from_ne_bytes(val_arr);
                    arg = Some(val);
                } else {
                    arg = None;
                }
            } else {
                arg = None;
            }
            vm.pc += 2;
            arg
        },
        OpAddrMode::Stack => vm.data_stack.pop(),
        _ => None,
    }
}

fn get_addr(vm : &mut Vm, addr_mode : &OpAddrMode) -> Option<isize> {
    match addr_mode {
        OpAddrMode::Immediate => { 
            if vm.pc + 4 >= RAM_SIZE {
                return None;
            }
            let mut val_arr : [u8; 4] = [0; 4];
            val_arr[0..4].clone_from_slice(&vm.ram[vm.pc as usize..vm.pc as usize + 4]);
            let val = (i32::from_ne_bytes(val_arr) >> 16) as isize;
            vm.pc += 4;
            Some(val)
        },
        OpAddrMode::IndexStack => {
            if vm.pc + 2 >= RAM_SIZE {
                return None;
            }
            let mut arg : Option<isize> = None;
            let mut base_arr : [u8; 2] = [0; 2];
            base_arr[0..2].clone_from_slice(&vm.ram[vm.pc as usize..vm.pc as usize + 2]);
            let base = i16::from_ne_bytes(base_arr) as isize;
            if let Some(offset) = vm.data_stack.pop() {
                let off = offset >> 16;
                let val_addr : isize = base + off as isize;
                if val_addr > 0 && val_addr < RAM_SIZE as isize {
                    arg = Some(val_addr);
                }
            }
            vm.pc += 2;
            arg
        },
        OpAddrMode::IndexImmediate => {
            if vm.pc + 2 >= RAM_SIZE {
                return None;
            }
            let mut arg : Option<isize> = None;
            let mut offset_arr : [u8; 2] = [0; 2];
            offset_arr[0..2].clone_from_slice(&vm.ram[vm.pc as usize..vm.pc as usize + 2]);
            let offset = i16::from_ne_bytes(offset_arr) as isize;
            if let Some(base) = vm.data_stack.pop() {
                let base = base >> 16;
                let val_addr : isize = base as isize + offset;
                if val_addr > 0 && val_addr < RAM_SIZE as isize {
                    arg = Some(val_addr);
                }
            }
            vm.pc += 2;
            arg
        },
        OpAddrMode::Stack => {
            let mut arg : Option<isize> = None;
            if let Some(addr) = vm.data_stack.pop() {
                arg = Some((addr >> 16) as isize);
            }
            arg
        },
        _ => None,
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
    fn test_push_imm_op() {
        let test_val : i32 = fp::float_to_fix(66.0);
        let mut vm = init_vm();
        let mut code : [u8; RAM_SIZE] = [0; RAM_SIZE];
        // Push Immediate
        code[0] = OpCodes::PushImm as u8; 
        code[1..5].clone_from_slice(&test_val.to_ne_bytes());
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
    
    #[test]
    fn test_push_idx_stk_op() {
        // Push Index Stack
        let mut vm = init_vm();
        let mut code : [u8; RAM_SIZE] = [0; RAM_SIZE];
        let base : u16 = 0x123;
        let offset = fp::float_to_fix(2.0);
        let target_addr = base + 2;
        vm.data_stack.push(offset);
        let test_val = 666.0;
        let test_val_fp = fp::float_to_fix(test_val);
        code[target_addr as usize..(target_addr as usize + 4)]
            .clone_from_slice(&test_val_fp.to_ne_bytes());
        code[0] = OpCodes::PushIndStk as u8;
        code[1..3].clone_from_slice(&base.to_ne_bytes());
        assert!(vm.load(&code));
        vm.cycle_once();
        assert_eq!(vm.pc, 3, "Failed to increment program counter.");
        assert!(!vm.data_stack.empty(), "Data stack empty after push.");
        let top_val = vm.data_stack.peek();
        assert!(!top_val.is_none(), "Data stack peek returned None on a nonempty stack.");
        assert_eq!(top_val.unwrap(), test_val_fp, "Data stack top was not expected value.");
    }
    
    #[test]
    fn test_push_idx_imm_op() {
        // Push Index Immediate 
        let mut vm = init_vm();
        let mut code : [u8; RAM_SIZE] = [0; RAM_SIZE];
        let base : usize = 0x123;
        let offset : u16 = 2;
        let target_addr = base + offset as usize;
        vm.data_stack.push(fp::float_to_fix(base as f32));
        let test_val = 666.0;
        let test_val_fp = fp::float_to_fix(test_val);
        code[target_addr as usize..(target_addr as usize + 4)]
            .clone_from_slice(&test_val_fp.to_ne_bytes());
        code[0] = OpCodes::PushIndImm as u8;
        code[1..3].clone_from_slice(&offset.to_ne_bytes());
        assert!(vm.load(&code));
        vm.cycle_once();
        assert_eq!(vm.pc, 3, "Failed to increment program counter.");
        assert!(!vm.data_stack.empty(), "Data stack empty after push.");
        let top_val = vm.data_stack.peek();
        assert!(!top_val.is_none(), "Data stack peek returned None on a nonempty stack.");
        assert_eq!(top_val.unwrap(), test_val_fp, "Data stack top was not expected value.");
    }
    
    #[test]
    fn test_push_stk_op() {
        // Push Stack
        let mut vm = init_vm();
        let mut code : [u8; RAM_SIZE] = [0; RAM_SIZE];
        let target_addr : usize = 0x123;
        vm.data_stack.push(fp::float_to_fix(target_addr as f32));
        let test_val = 666.0;
        let test_val_fp = fp::float_to_fix(test_val);
        code[target_addr as usize..(target_addr as usize + 4)]
            .clone_from_slice(&test_val_fp.to_ne_bytes());
        code[0] = OpCodes::PushStk as u8;
        assert!(vm.load(&code));
        vm.cycle_once();
        assert_eq!(vm.pc, 1, "Failed to increment program counter.");
        assert!(!vm.data_stack.empty(), "Data stack empty after push.");
        let top_val = vm.data_stack.peek();
        assert!(!top_val.is_none(), "Data stack peek returned None on a nonempty stack.");
        assert_eq!(top_val.unwrap(), test_val_fp, "Data stack top was not expected value.");
    }
    
    #[test]
    fn test_push_end_of_ram() {
        let mut vm = init_vm();
        let mut code : [u8; RAM_SIZE] = [0; RAM_SIZE];
        code[RAM_SIZE -1] = OpCodes::PushImm as u8;
        assert!(vm.load(&code));
        vm.pc = RAM_SIZE - 1;
        vm.cycle_once();
        assert_eq!(vm.pc, RAM_SIZE, "PC not properly incremented in end of ram test: PUSH_IMM");
        code[RAM_SIZE -1] = OpCodes::PushIndStk as u8;
        assert!(vm.load(&code));
        vm.pc = RAM_SIZE - 1;
        vm.cycle_once();
        assert_eq!(vm.pc, RAM_SIZE, "PC not properly incremented in end of ram test: PUSH_IND_STK");
        code[RAM_SIZE -1] = OpCodes::PushIndImm as u8;
        assert!(vm.load(&code));
        vm.pc = RAM_SIZE - 1;
        vm.cycle_once();
        assert_eq!(vm.pc, RAM_SIZE, "PC not properly incremented in end of ram test: PUSH_IND_IMM");
    }
    
    #[test]
    fn test_store_imm_op() {
        let test_addr : i32 = fp::float_to_fix(66.0);
        let mut vm = init_vm();
        let mut code : [u8; RAM_SIZE] = [0; RAM_SIZE];
        // Store Immediate
        code[0] = OpCodes::StoreImm as u8; 
        code[1..5].clone_from_slice(&test_addr.to_ne_bytes());
        assert!(vm.load(&code));
        let test_val : i32 = fp::float_to_fix(1234.0);
        vm.data_stack.push(test_val);
        vm.cycle_once();
        assert_eq!(vm.pc, 5, "Failed to increment program counter.");
        assert!(vm.data_stack.empty(), "Data stack not empty after store.");
        let mut chk_val_arr = [0u8; 4];
        chk_val_arr[0..4]
            .clone_from_slice(&vm.ram[(test_addr >> 16) as usize.. (test_addr >> 16) as usize + 4]);
        let chk_val = i32::from_ne_bytes(chk_val_arr);
        assert_eq!(chk_val, test_val, "Store failed to save value in ram!");
    }
    
    #[test]
    fn test_store_idx_stk_op() {
        let mut vm = init_vm();
        let test_val = 666.0;
        let test_val_fp = fp::float_to_fix(test_val);
        vm.data_stack.push(test_val_fp);
        let mut code : [u8; RAM_SIZE] = [0; RAM_SIZE];
        let base : u16 = 0x123;
        let offset = fp::float_to_fix(2.0);
        let target_addr = base + 2;
        vm.data_stack.push(offset);
        code[0] = OpCodes::StoreIndStk as u8;
        code[1..3].clone_from_slice(&base.to_ne_bytes());
        assert!(vm.load(&code));
        vm.cycle_once();
        assert_eq!(vm.pc, 3, "Failed to increment program counter.");
        assert!(vm.data_stack.empty(), "Data stack not empty after store.");
        let mut chk_val_arr = [0u8; 4];
        chk_val_arr[0..4]
            .clone_from_slice(&vm.ram[target_addr as usize..target_addr as usize + 4]);
        let chk_val = i32::from_ne_bytes(chk_val_arr);
        assert_eq!(chk_val, test_val_fp, "Value at address not value to be stored!");
    }
}


