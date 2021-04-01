use crate::stk::Stack;
use crate::fp;
use super::opcodes::*;

pub (super) fn cycle_op(vm : &mut super::Vm, inst : u8) {
    let op_type = StackOpTypes::from(inst);
    let addr_mode = OpAddrMode::from(inst);
    match op_type {
        StackOpTypes::Push => op_push(vm, addr_mode),
        StackOpTypes::Store => op_store(vm, addr_mode),
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
    if let Some(addr) = super::get_addr_val(vm, &addr_mode) {
        let addr = (addr >> 16) as usize;
        if let Some(data) = vm.data_stack.pop() {
            vm.ram[addr..(addr+4)].clone_from_slice(&data.to_ne_bytes());
        }
    }
}
