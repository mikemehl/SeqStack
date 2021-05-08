use super::opcodes::*;
use crate::stk::Stack;
use crate::vm::fp;

pub(super) fn cycle_op(vm: &mut super::Vm, inst: u8) {
    let op_type = BitOpTypes::from(inst);
    match op_type {
        BitOpTypes::Shl =>
        {
            if let Some(shift_amt) = vm.data_stack.pop() 
            {

               let shift_amt = fp::fix_to_float(shift_amt) as i32; 
               if (shift_amt >= 0)
               {
                  if let Some(val) = vm.data_stack.pop()
                  {
                      vm.data_stack.push(val << shift_amt);
                  }
               }
            }
        }
        _ => {}
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::vm::Vm;
    use crate::vm::INVALID_INTERRUPT;
    use crate::vm::RAM_SIZE;
    use crate::fp;

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
    fn test_shl()
    {
        let mut vm = init_vm();
        let a = 0xFFFFFFFFu32 as i32;
        let shift_amt = fp::float_to_fix(1.0);
        let expected = a << 1;
        vm.data_stack.push(a);
        vm.data_stack.push(shift_amt);
        let mut code = [0u8; RAM_SIZE];
        code[0] = OpCodes::Shl as u8;
        vm.load(&code);
        vm.cycle_once();
        assert_eq!(vm.pc, 1, "Shiftl failed to increment program counter!");
        let result = vm.data_stack.pop();
        assert!(!result.is_none(), "NONE on stack pop!");
        assert_eq!(result.unwrap(), expected, "Shiftl expected {:x} but found {:x}!", expected, a);
    }
}
