use crate::stk::Stack;
use crate::fp;
use super::opcodes::*;

pub (super) fn cycle_op(vm : &mut super::Vm, inst : u8) {
    let op_type = StackOpTypes::from(inst);
    let addr_mode = OpAddrMode::from(inst);
    match op_type {
        StackOpTypes::Push => op_push(vm, addr_mode),
        StackOpTypes::Store => op_store(vm, addr_mode),
        StackOpTypes::Pop => { let _ = vm.data_stack.pop(); },
        StackOpTypes::Dup => { 
            if let Some(val) = vm.data_stack.peek() {
                vm.data_stack.push(val);
            }
        },
        _ => {} 
    }
}

pub fn op_push(vm : &mut super::Vm, addr_mode : OpAddrMode) {
    let arg : Option<i32> = super::get_addr_val(vm, &addr_mode);
    if let Some(p_val) = arg {
        if  let OpAddrMode::Stack = addr_mode {
            let addr = p_val >> 16_isize;
            if addr >= 0 {
                let mut val_arr : [u8; 4] = [0; 4];
                val_arr[0..4].clone_from_slice(&vm.ram[addr as usize..addr as usize + 4]);
                vm.data_stack.push(i32::from_ne_bytes(val_arr));
            }
        }
        else {
            vm.data_stack.push(p_val);
        }
    }
}

pub fn op_store(vm : &mut super::Vm, addr_mode : OpAddrMode) {
    if let Some(addr) = super::get_addr(vm, &addr_mode) {
        if let Some(data) = vm.data_stack.pop() {
            vm.ram[addr as usize..(addr as usize+4)].clone_from_slice(&data.to_ne_bytes());
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::vm::RAM_SIZE;
    use crate::vm::INVALID_INTERRUPT;
    use crate::vm::Vm;
    
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
    #[test]
    fn test_store_idx_imm_op() {
        let mut vm = init_vm();
        let test_val = 666.0;
        let test_val_fp = fp::float_to_fix(test_val);
        vm.data_stack.push(test_val_fp);
        let mut code : [u8; RAM_SIZE] = [0; RAM_SIZE];
        let base = fp::float_to_fix(123.0);
        let offset = 2u16;
        let target_addr = 123 + 2;
        vm.data_stack.push(base);
        code[0] = OpCodes::StoreIndStk as u8;
        code[1..3].clone_from_slice(&offset.to_ne_bytes());
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
    
    #[test]
    fn test_store_stk_op() {
        let mut vm = init_vm();
        let test_val = 666.0;
        let test_val_fp = fp::float_to_fix(test_val);
        vm.data_stack.push(test_val_fp);
        let mut code : [u8; RAM_SIZE] = [0; RAM_SIZE];
        let target_addr = 123u16;
        let target_addr_fp = fp::float_to_fix(target_addr as f32);
        vm.data_stack.push(target_addr_fp);
        code[0] = OpCodes::StoreStk as u8;
        assert!(vm.load(&code));
        vm.cycle_once();
        assert_eq!(vm.pc, 1, "Failed to increment program counter.");
        assert!(vm.data_stack.empty(), "Data stack not empty after store.");
        let mut chk_val_arr = [0u8; 4];
        chk_val_arr[0..4]
            .clone_from_slice(&vm.ram[target_addr as usize..target_addr as usize + 4]);
        let chk_val = i32::from_ne_bytes(chk_val_arr);
        assert_eq!(chk_val, test_val_fp, "Value at address not value to be stored!");
    }

    #[test]
    fn test_pop_op() {
        let mut vm = init_vm();
        let test_val = 666.0;
        let test_val_fp = fp::float_to_fix(test_val);
        vm.data_stack.push(test_val_fp);
        let mut code = [0u8; RAM_SIZE];
        code[0] = OpCodes::Pop as u8;
        code[1] = OpCodes::Pop as u8;
        assert!(vm.load(&code));
        vm.cycle_once();
        assert_eq!(vm.pc, 1, "Failed to increment program counter.");
        assert!(vm.data_stack.empty(), "Data stack not empty after pop.");
        // Cycle again, make sure things don't fall apart.
        vm.cycle_once();
        assert_eq!(vm.pc, 2, "Failed to increment program counter.");
        assert!(vm.data_stack.empty(), "Data stack not empty after pop.");
    }

    #[test]
    fn test_dup_op() {
        let mut vm = init_vm();
        let test_val = 666.0;
        let test_val_fp = fp::float_to_fix(test_val);
        vm.data_stack.push(test_val_fp);
        let mut code = [0u8; RAM_SIZE];
        code[0] = OpCodes::Dup as u8;
        assert!(vm.load(&code));
        vm.cycle_once();
        assert_eq!(vm.pc, 1, "Failed to increment program counter.");
        assert!(!vm.data_stack.empty(), "Data stack empty after dup.");
        let top = vm.data_stack.pop();
        assert!(!top.is_none(), "Pop failed after dup!");
        let next = vm.data_stack.pop();
        assert!(!next.is_none(), "Second pop failed after dup!");
        assert_eq!(top.unwrap(), next.unwrap(), "Dup didn't duplicate!");
    }
    
    #[test]
    fn test_rot_op() {
        // a b c -> b c a
        let mut vm = init_vm();
        let a = 666.0;
        let a_fp = fp::float_to_fix(a);
        let b = 111.0;
        let b_fp = fp::float_to_fix(b);
        let c = 111.0;
        let c_fp = fp::float_to_fix(c);
        vm.data_stack.push(c_fp);
        vm.data_stack.push(b_fp);
        vm.data_stack.push(a_fp);
        let code = [OpCodes::Rot as u8; 1];
        assert!(vm.load(&code));
        vm.cycle_once();
        assert_eq!(vm.pc, 1, "Failed to increment program counter.");
        let top = vm.data_stack.pop();
        assert!(!top.is_none(), "Top empty after rot!");
        assert_eq!(top.unwrap(), b_fp, "Unexpected value in stack!");
        let top = vm.data_stack.pop();
        assert!(!top.is_none(), "Top empty after rot!");
        assert_eq!(top.unwrap(), c_fp, "Unexpected value in stack!");
        let top = vm.data_stack.pop();
        assert!(!top.is_none(), "Top empty after rot!");
        assert_eq!(top.unwrap(), c_fp, "Unexpected value in stack!");
    }
}


