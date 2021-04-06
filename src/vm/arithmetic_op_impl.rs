use super::opcodes::*;
use crate::fp;
use crate::stk::Stack;

pub(super) fn cycle_op(vm: &mut super::Vm, inst: u8) {
    let op_type = ArithmeticOpTypes::from(inst);
    match op_type {
        ArithmeticOpTypes::Add => {
            if let Some(a) = vm.data_stack.pop() {
                if let Some(b) = vm.data_stack.pop() {
                    vm.data_stack.push(a + b);
                } else {
                    vm.data_stack.push(a);
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
    fn test_add_op() {
        fn run_test(a: f32, b: f32) -> () {
            let mut vm = init_vm();
            let c = a + b;
            let a_fp = fp::float_to_fix(a);
            let b_fp = fp::float_to_fix(b);
            let c_fp = fp::float_to_fix(c);
            vm.data_stack.push(a_fp);
            vm.data_stack.push(b_fp);
            let mut code = [0u8; RAM_SIZE];
            code[0] = OpCodes::Add as u8;
            vm.load(&code);
            vm.cycle_once();
            assert_eq!(vm.pc, 1, "Failed to increment program counter.");
            let r = vm.data_stack.pop();
            assert!(!r.is_none(), "Data stack empty after add.");
            assert_eq!(r.unwrap(), c_fp, "Wrong value after add.");
            assert!(
                vm.data_stack.pop().is_none(),
                "Extraneous data left on stack after add."
            );
        }
        run_test(5.0, 12.0);
        run_test(-25.0, 24.0);
        run_test(i16::MAX as f32, 0.0);
        run_test(-1234.0, 1234.0);
    }

    #[test]
    fn test_sub_op() {
        fn run_test(a: f32, b: f32) -> () {
            let mut vm = init_vm();
            let c = a + b;
            let a_fp = fp::float_to_fix(a);
            let b_fp = fp::float_to_fix(b);
            let c_fp = fp::float_to_fix(c);
            vm.data_stack.push(a_fp);
            vm.data_stack.push(b_fp);
            let mut code = [0u8; RAM_SIZE];
            code[0] = OpCodes::Sub as u8;
            vm.load(&code);
            vm.cycle_once();
            assert_eq!(vm.pc, 1, "Failed to increment program counter.");
            let r = vm.data_stack.pop();
            assert!(!r.is_none(), "Data stack empty after add.");
            assert_eq!(r.unwrap(), c_fp, "Wrong value after add.");
            assert!(
                vm.data_stack.pop().is_none(),
                "Extraneous data left on stack after add."
            );
        }
        run_test(5.0, 12.0);
        run_test(-25.0, 24.0);
        run_test(i16::MAX as f32, 0.0);
        run_test(-1234.0, 1234.0);
    }
}
