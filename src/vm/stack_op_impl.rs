use crate::stk::Stack;
use crate::fp;
use super::opcodes::*;

pub (super) fn cycle_op(vm : &mut super::Vm, inst : u8) {
    let op_type = StackOpTypes::from(inst);
    let addr_mode = OpAddrMode::from(inst);
    match op_type {
        StackOpTypes::Push => op_push(vm, addr_mode),
        StackOpTypes::Store => {},
        _ => {} 
    }
}

pub fn op_push(vm : &mut super::Vm, addr_mode : OpAddrMode) {
    let arg : Option<i32> = super::get_addr_val(vm, &addr_mode);
    if let Some(p_val) = arg {
        if  let OpAddrMode::Stack = addr_mode {
            let addr = p_val >> 16 as isize;
            if addr >= 0 {
                let mut val_arr : [u8; 4] = [0; 4];
                for i in 0..val_arr.len() {
                    val_arr[i] = vm.ram[addr as usize + i];
                }
                vm.data_stack.push(i32::from_ne_bytes(val_arr));
            }
        }
        else {
            vm.data_stack.push(p_val);
        }
    }
}
