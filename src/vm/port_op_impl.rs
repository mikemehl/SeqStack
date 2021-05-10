
use super::opcodes::*;
use crate::stk::Stack;
use crate::vm::fp;

pub(super) fn cycle_op(vm: &mut super::Vm, inst: u8) {
    let op_type = PortOpTypes::from(inst);
    match op_type {
        PortOpTypes::Push => {
            if let Some(val) = vm.data_stack.pop() {
                let port_val = (inst & 0b00000111) as usize;
                vm.ports[port_val].push(val);
            }
        }
        _ => ()
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::fp;
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
    fn test_push() {
        let mut vm = init_vm();
        let val = 0xDEADC0EDu32 as i32;
        vm.data_stack.push(val);
        let mut code = [0u8; RAM_SIZE];
        code[0] = 0b100_00_000;
        vm.load(&code);
        vm.cycle_once();
        assert!(vm.data_stack.empty(), "PortPush failed to modify data stack!");
        assert_eq!(vm.pc, 1, "PortPush failed to increment program counter!");
        let port_val = vm.ports[0].pop();
        assert!(!port_val.is_none(), "PortPush failed to push to port!");
        assert_eq!(port_val.unwrap(), val, "PortPush pushed wrong value to port!");
    }
}
