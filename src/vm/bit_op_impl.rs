use super::opcodes::*;
use crate::stk::Stack;
use crate::vm::fp;

pub(super) fn cycle_op(vm: &mut super::Vm, inst: u8) {
    let op_type = BitOpTypes::from(inst);
    match op_type {
        BitOpTypes::Shl => {
            if let Some(shift_amt) = vm.data_stack.pop() {
                let shift_amt = fp::fix_to_float(shift_amt) as i32;
                if shift_amt >= 0 {
                    if let Some(val) = vm.data_stack.pop() {
                        vm.data_stack.push(val << shift_amt);
                    }
                }
            }
        }
        BitOpTypes::Shr => {
            if let Some(shift_amt) = vm.data_stack.pop() {
                let shift_amt = fp::fix_to_float(shift_amt) as i32;
                if shift_amt >= 0 {
                    if let Some(val) = vm.data_stack.pop() {
                        vm.data_stack.push(val >> shift_amt);
                    }
                }
            }
        }
        BitOpTypes::Rotl => {
            if let Some(rot_amt) = vm.data_stack.pop() {
                let rot_amt = fp::fix_to_float(rot_amt) as i32;
                if rot_amt >= 0 {
                    if let Some(val) = vm.data_stack.pop() {
                        let mask = (0xFFFFFFFFu32 as i32) << (32 - rot_amt) as i32;
                        let masked_val: i32 = val & mask;
                        let rot_bits: i32 = (masked_val as u32 >> (32 - rot_amt)) as i32;
                        vm.data_stack.push((val << rot_amt) | rot_bits);
                    }
                }
            }
        }
        BitOpTypes::Rotr => {
            if let Some(rot_amt) = vm.data_stack.pop() {
                let rot_amt = fp::fix_to_float(rot_amt) as i32;
                if rot_amt >= 0 {
                    if let Some(val) = vm.data_stack.pop() {
                        let mask = (0xFFFFFFFFu32 as i32) >> (32 - rot_amt) as i32;
                        let masked_val: i32 = val & mask;
                        let rot_bits: i32 = ((masked_val as u32) << (32 - rot_amt)) as i32;
                        vm.data_stack.push((val >> rot_amt) | rot_bits);
                    }
                }
            }
        }
        BitOpTypes::And => {
            if let Some(a) = vm.data_stack.pop() {
                if let Some(b) = vm.data_stack.pop() {
                    vm.data_stack.push(a & b);
                }
            }
        }
        BitOpTypes::Or => {
            if let Some(a) = vm.data_stack.pop() {
                if let Some(b) = vm.data_stack.pop() {
                    let val = a | b;
                    vm.data_stack.push(val);
                }
            }
        }
        BitOpTypes::Xor => {
            if let Some(a) = vm.data_stack.pop() {
                if let Some(b) = vm.data_stack.pop() {
                    let val = a ^ b;
                    vm.data_stack.push(val);
                }
            }
        }
        BitOpTypes::Not => {
            if let Some(a) = vm.data_stack.pop() {
                vm.data_stack.push(!a);
            }
        }
        _ => {}
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
    fn test_shl() {
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
        assert_eq!(
            result.unwrap(),
            expected,
            "Shiftl expected {:x} but found {:x}!",
            expected,
            result.unwrap()
        );

        let mut vm = init_vm();
        let a = 0xFFFFFFFFu32 as i32;
        let shift_amt = fp::float_to_fix(2.0);
        let expected = a << 2;
        vm.data_stack.push(a);
        vm.data_stack.push(shift_amt);
        let mut code = [0u8; RAM_SIZE];
        code[0] = OpCodes::Shl as u8;
        vm.load(&code);
        vm.cycle_once();
        assert_eq!(vm.pc, 1, "Shiftl failed to increment program counter!");
        let result = vm.data_stack.pop();
        assert!(!result.is_none(), "NONE on stack pop!");
        assert_eq!(
            result.unwrap(),
            expected,
            "Shiftl expected {:x} but found {:x}!",
            expected,
            result.unwrap()
        );
    }

    #[test]
    fn test_shr() {
        let mut vm = init_vm();
        let a = 0xFFFFFFFFu32 as i32;
        let shift_amt = fp::float_to_fix(1.0);
        let expected = a >> 1;
        vm.data_stack.push(a);
        vm.data_stack.push(shift_amt);
        let mut code = [0u8; RAM_SIZE];
        code[0] = OpCodes::Shr as u8;
        vm.load(&code);
        vm.cycle_once();
        assert_eq!(vm.pc, 1, "Shiftr failed to increment program counter!");
        let result = vm.data_stack.pop();
        assert!(!result.is_none(), "NONE on stack pop!");
        assert_eq!(
            result.unwrap(),
            expected,
            "Shiftr expected {:x} but found {:x}!",
            expected,
            result.unwrap()
        );

        let mut vm = init_vm();
        let a = 0xFFFFFFFFu32 as i32;
        let shift_amt = fp::float_to_fix(2.0);
        let expected = a >> 2;
        vm.data_stack.push(a);
        vm.data_stack.push(shift_amt);
        let mut code = [0u8; RAM_SIZE];
        code[0] = OpCodes::Shr as u8;
        vm.load(&code);
        vm.cycle_once();
        assert_eq!(vm.pc, 1, "Shiftr failed to increment program counter!");
        let result = vm.data_stack.pop();
        assert!(!result.is_none(), "NONE on stack pop!");
        assert_eq!(
            result.unwrap(),
            expected,
            "Shiftr expected {:x} but found {:x}!",
            expected,
            result.unwrap()
        );
    }

    #[test]
    fn test_rotl() {
        let mut vm = init_vm();
        let a = 0x8FFFFFFFu32 as i32;
        let expected = 0x1FFFFFFFu32 as i32;
        let rot_amt = fp::float_to_fix(1.0);
        vm.data_stack.push(a);
        vm.data_stack.push(rot_amt);
        let mut code = [0u8; RAM_SIZE];
        code[0] = OpCodes::Rotl as u8;
        vm.load(&code);
        vm.cycle_once();
        assert_eq!(vm.pc, 1, "Rotl failed to increment program counter!");
        let result = vm.data_stack.pop();
        assert!(!result.is_none(), "NONE on stack pop!");
        assert_eq!(
            result.unwrap(),
            expected,
            "Rotl expected {:x} but found {:x}!",
            expected,
            result.unwrap()
        );

        let mut vm = init_vm();
        let a = 0x8FFFFFFFu32 as i32;
        let expected = 0x3FFFFFFEu32 as i32;
        let rot_amt = fp::float_to_fix(2.0);
        vm.data_stack.push(a);
        vm.data_stack.push(rot_amt);
        let mut code = [0u8; RAM_SIZE];
        code[0] = OpCodes::Rotl as u8;
        vm.load(&code);
        vm.cycle_once();
        assert_eq!(vm.pc, 1, "Rotl failed to increment program counter!");
        let result = vm.data_stack.pop();
        assert!(!result.is_none(), "NONE on stack pop!");
        assert_eq!(
            result.unwrap(),
            expected,
            "Rotl expected {:x} but found {:x}!",
            expected,
            result.unwrap()
        );
    }

    #[test]
    fn test_rotr() {
        let mut vm = init_vm();
        let a = 0xFFFFFFF1u32 as i32;
        let expected = 0xFFFFFFF8u32 as i32;
        let rot_amt = fp::float_to_fix(1.0);
        vm.data_stack.push(a);
        vm.data_stack.push(rot_amt);
        let mut code = [0u8; RAM_SIZE];
        code[0] = OpCodes::Rotr as u8;
        vm.load(&code);
        vm.cycle_once();
        assert_eq!(vm.pc, 1, "Rotr failed to increment program counter!");
        let result = vm.data_stack.pop();
        assert!(!result.is_none(), "NONE on stack pop!");
        assert_eq!(
            result.unwrap(),
            expected,
            "Rotr expected {:x} but found {:x}!",
            expected,
            result.unwrap()
        );

        let mut vm = init_vm();
        let a = 0xFFFFFFF1u32 as i32;
        let expected = 0xFFFFFFFCu32 as i32;
        let rot_amt = fp::float_to_fix(2.0);
        vm.data_stack.push(a);
        vm.data_stack.push(rot_amt);
        let mut code = [0u8; RAM_SIZE];
        code[0] = OpCodes::Rotr as u8;
        vm.load(&code);
        vm.cycle_once();
        assert_eq!(vm.pc, 1, "Rotr failed to increment program counter!");
        let result = vm.data_stack.pop();
        assert!(!result.is_none(), "NONE on stack pop!");
        assert_eq!(
            result.unwrap(),
            expected,
            "Rotr expected {:x} but found {:x}!",
            expected,
            result.unwrap()
        );
    }

    #[test]
    fn test_and() {
        let mut vm = init_vm();
        let a = 0xFFFF0000u32 as i32;
        let b = 0x000FFFFFu32 as i32;
        let expected = a & b;
        vm.data_stack.push(a);
        vm.data_stack.push(b);
        let mut code = [0u8; RAM_SIZE];
        code[0] = OpCodes::And as u8;
        vm.load(&code);
        vm.cycle_once();
        assert_eq!(vm.pc, 1, "And failed to increment program counter!");
        let result = vm.data_stack.pop();
        assert!(!result.is_none(), "NONE on stack pop!");
        assert_eq!(
            result.unwrap(),
            expected,
            "And expected {:x} but found {:x}!",
            expected,
            result.unwrap()
        );
    }

    #[test]
    fn test_or() {
        let mut vm = init_vm();
        let a = 0xF1FF0000u32 as i32;
        let b = 0x000F1F1Fu32 as i32;
        let expected = a | b;
        vm.data_stack.push(a);
        vm.data_stack.push(b);
        let mut code = [0u8; RAM_SIZE];
        code[0] = OpCodes::Or as u8;
        vm.load(&code);
        vm.cycle_once();
        assert_eq!(vm.pc, 1, "Or failed to increment program counter!");
        let result = vm.data_stack.pop();
        assert!(!result.is_none(), "NONE on stack pop!");
        assert_eq!(
            result.unwrap(),
            expected,
            "Or expected {:x} but found {:x}!",
            expected,
            result.unwrap()
        );
    }

    #[test]
    fn test_xor() {
        let mut vm = init_vm();
        let a = 0xF1FF0000u32 as i32;
        let b = 0x000F1F1Fu32 as i32;
        let expected = a ^ b;
        vm.data_stack.push(a);
        vm.data_stack.push(b);
        let mut code = [0u8; RAM_SIZE];
        code[0] = OpCodes::Xor as u8;
        vm.load(&code);
        vm.cycle_once();
        assert_eq!(vm.pc, 1, "Xor failed to increment program counter!");
        let result = vm.data_stack.pop();
        assert!(!result.is_none(), "NONE on stack pop!");
        assert_eq!(
            result.unwrap(),
            expected,
            "Xor expected {:x} but found {:x}!",
            expected,
            result.unwrap()
        );
    }

    #[test]
    fn test_not() {
        let mut vm = init_vm();
        let a = 0xF1FF0000u32 as i32;
        let expected = !a;
        vm.data_stack.push(a);
        let mut code = [0u8; RAM_SIZE];
        code[0] = OpCodes::Not as u8;
        vm.load(&code);
        vm.cycle_once();
        assert_eq!(vm.pc, 1, "Not failed to increment program counter!");
        let result = vm.data_stack.pop();
        assert!(!result.is_none(), "NONE on stack pop!");
        assert_eq!(
            result.unwrap(),
            expected,
            "Not expected {:x} but found {:x}!",
            expected,
            result.unwrap()
        );
    }
}
