/// Module with the central vm structures.
mod opcodes;
mod stack_op_impl;

use crate::fp;
use crate::stk::Stack;
use opcodes::*;

const RAM_SIZE: usize = 1 << 15;
const NUM_INTERRUPTS: usize = 8;
const NUM_PORTS: usize = NUM_INTERRUPTS;
const INVALID_INTERRUPT: i16 = -1;

pub struct Vm {
    ram: Box<[u8]>,
    pc: usize,
    data_stack: Box<Stack>,
    call_stack: Box<Stack>,
    interrupts: Box<[i16]>,
    ports: Box<[Stack]>,
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
            ram: ram.into_boxed_slice(),
            pc: 0,
            data_stack,
            call_stack,
            interrupts: interrupts.into_boxed_slice(),
            ports: ports.into_boxed_slice(),
        })
    }

    pub fn load(&mut self, code_in: &[u8]) -> bool {
        if code_in.len() > self.ram.len() {
            return false;
        }
        self.ram[0..code_in.len()].clone_from_slice(code_in);
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
        let fam: OpFamily = OpFamily::from(next_inst);
        match fam {
            OpFamily::StackOp => {
                stack_op_impl::cycle_op(self, next_inst);
            }
            _ => {}
        }
    }
}

// Extracts the adress needed for an op given the adressing mode. Increments the program counter.
fn get_addr(vm: &mut Vm, addr_mode: &OpAddrMode) -> Option<isize> {
    match addr_mode {
        OpAddrMode::Immediate => {
            if vm.pc + 4 >= RAM_SIZE {
                return None;
            }
            let mut val_arr: [u8; 4] = [0; 4];
            val_arr[0..4].clone_from_slice(&vm.ram[vm.pc as usize..vm.pc as usize + 4]);
            let val = (i32::from_ne_bytes(val_arr) >> 16) as isize;
            vm.pc += 4;
            Some(val)
        }
        OpAddrMode::IndexStack => {
            if vm.pc + 2 >= RAM_SIZE {
                return None;
            }
            let mut arg: Option<isize> = None;
            let mut base_arr: [u8; 2] = [0; 2];
            base_arr[0..2].clone_from_slice(&vm.ram[vm.pc as usize..vm.pc as usize + 2]);
            let base = i16::from_ne_bytes(base_arr) as isize;
            if let Some(offset) = vm.data_stack.pop() {
                let off = offset >> 16;
                let val_addr: isize = base + off as isize;
                if val_addr > 0 && val_addr < RAM_SIZE as isize {
                    arg = Some(val_addr);
                }
            }
            vm.pc += 2;
            arg
        }
        OpAddrMode::IndexImmediate => {
            if vm.pc + 2 >= RAM_SIZE {
                return None;
            }
            let mut arg: Option<isize> = None;
            let mut offset_arr: [u8; 2] = [0; 2];
            offset_arr[0..2].clone_from_slice(&vm.ram[vm.pc as usize..vm.pc as usize + 2]);
            let offset = i16::from_ne_bytes(offset_arr) as isize;
            if let Some(base) = vm.data_stack.pop() {
                let base = base >> 16;
                let val_addr: isize = base as isize + offset;
                if val_addr > 0 && val_addr < RAM_SIZE as isize {
                    arg = Some(val_addr);
                }
            }
            vm.pc += 2;
            arg
        }
        OpAddrMode::Stack => {
            let mut arg: Option<isize> = None;
            if let Some(addr) = vm.data_stack.pop() {
                arg = Some((addr >> 16) as isize);
            }
            arg
        }
        _ => None,
    }
}

// Extracts the value in memory or on the stack based on the addressing mode. Increments the program counter if necessary and get_addr does not (or isn't used).
// Used in *_op_impl modules.
fn get_addr_val(vm: &mut Vm, addr_mode: &OpAddrMode) -> Option<i32> {
    match addr_mode {
        OpAddrMode::Immediate => {
            if vm.pc + 4 >= RAM_SIZE {
                return None;
            }
            let mut val_arr: [u8; 4] = [0; 4];
            val_arr[0..4].clone_from_slice(&vm.ram[vm.pc as usize..vm.pc as usize + 4]);
            let val = i32::from_ne_bytes(val_arr);
            vm.pc += 4;
            Some(val)
        }
        OpAddrMode::IndexStack | OpAddrMode::IndexImmediate => {
            if vm.pc + 2 >= RAM_SIZE {
                return None;
            }
            let mut arg: Option<i32> = None;
            if let Some(addr) = get_addr(vm, addr_mode) {
                let mut val_arr: [u8; 4] = [0; 4];
                val_arr[0..4].clone_from_slice(&vm.ram[addr as usize..addr as usize + 4]);
                arg = Some(i32::from_ne_bytes(val_arr));
            }
            arg
        }
        OpAddrMode::Stack => vm.data_stack.pop(),
        _ => None,
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::vm::Vm;
    use crate::vm::INVALID_INTERRUPT;
    use crate::vm::RAM_SIZE;

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
        const TEST_VAL: u8 = 66;
        let mut vm = init_vm();
        let code: [u8; RAM_SIZE] = [TEST_VAL; RAM_SIZE];
        assert!(vm.load(&code));
        for b in vm.ram.iter() {
            assert_eq!(*b, TEST_VAL);
        }
        let code: [u8; RAM_SIZE - 66] = [TEST_VAL; RAM_SIZE - 66];
        assert!(vm.load(&code));
        for b in vm.ram.iter() {
            assert_eq!(*b, TEST_VAL);
        }
        let code: [u8; RAM_SIZE + 10] = [TEST_VAL; RAM_SIZE + 10];
        assert!(!vm.load(&code));
    }
}
