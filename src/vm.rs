/// Module with the central vm structures.



#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn test_init() {
        let vm = Vm::new();
        assert!(vm.pc == 0);
        assert!(vm.data_stack.empty());
        assert!(vm.call_stack.empty());
        for p in &vm.ports {
            assert!(*p.empty());
        }
        for i in &vm.interrupts {
            assert_eq!(*i, INVALID_INTERRUPT);
        }
    }
}
