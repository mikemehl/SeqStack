/// Module of fixed point arithmetic methods for the virtual machine.
/// SeqStk uses two's complement 16.16 bit fixed point arithmetic.

const FP_ONE: i32 = 1 << 16;
const FP_LSB: f32 = 1.0 / ((1 << 16) as f32);

pub fn float_to_fix(a: f32) -> i32 {
    (a / FP_LSB) as i32
}

pub fn fix_to_float(a: i32) -> f32 {
    a as f32 * FP_LSB
}

pub fn fp_mul(a: i32, b: i32) -> i32 {
    ((i64::from(a) * i64::from(b)) >> 16) as i32
}

pub fn fp_div(a: i32, b: i32) -> i32 {
    if b == 0 {
        0
    } else {
        ((i64::from(a) << 16) / i64::from(b)) as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;
    const EPSILON: f32 = 11.0 * FP_LSB; // TODO: Better precision.
    const VAL_A: f32 = 1234.5678;
    const VAL_A_FP: i32 = (VAL_A * (1 << 16) as f32) as i32;
    const VAL_B: f32 = -8675.309;
    const VAL_B_FP: i32 = (VAL_B * (1 << 16) as f32) as i32;

    #[test]
    fn zero() {
        assert_eq!(float_to_fix(0.0), 0);
    }

    #[test]
    fn one() {
        assert_eq!(float_to_fix(1.0), FP_ONE);
    }

    #[test]
    fn a() {
        assert_eq!(float_to_fix(VAL_A), VAL_A_FP);
    }

    #[test]
    fn b() {
        assert_eq!(float_to_fix(VAL_B), VAL_B_FP);
    }

    #[test]
    fn back_and_forth() {
        let a_fp = float_to_fix(VAL_A);
        let b_fp = float_to_fix(VAL_B);
        assert_eq!(fix_to_float(a_fp), VAL_A);
        assert_eq!(fix_to_float(b_fp), VAL_B);
    }

    #[test]
    fn multiply_basic() {
        let a_fp: i32 = 0;
        let b_fp: i32 = 0;
        assert_eq!(fp_mul(a_fp, b_fp), 0);
        let a_fp: i32 = FP_ONE;
        let b_fp: i32 = 0;
        assert_eq!(fp_mul(a_fp, b_fp), 0);
        let a_fp: i32 = float_to_fix(1.0);
        let b_fp: i32 = float_to_fix(1.0);
        assert_eq!(fp_mul(a_fp, b_fp), FP_ONE);
        let a_fp: i32 = float_to_fix(5.0);
        let b_fp: i32 = float_to_fix(5.0);
        assert_eq!(fix_to_float(fp_mul(a_fp, b_fp)), 25.0);
        let a_fp: i32 = float_to_fix(5.0);
        let b_fp: i32 = float_to_fix(-5.0);
        assert_eq!(fix_to_float(fp_mul(a_fp, b_fp)), -25.0);
    }

    #[test]
    fn multiply_advanced() {
        let eps_float = EPSILON;
        let eps_fp = float_to_fix(eps_float);
        let a_fp: i32 = float_to_fix(12.56);
        let b_fp: i32 = float_to_fix(34.23);
        let expected_float = 12.56 * 34.23;
        let expected_fp = float_to_fix(expected_float);
        let result_fp = fp_mul(a_fp, b_fp);
        let result_float = fix_to_float(result_fp);
        let diff_fp = (result_fp - expected_fp).abs();
        let diff_float = (result_float - expected_float).abs();
        assert!(diff_fp < eps_fp);
        assert!(diff_float < eps_float);
        let eps_float = EPSILON;
        let eps_fp = float_to_fix(eps_float);
        let a_fp: i32 = float_to_fix(12.56);
        let b_fp: i32 = float_to_fix(-34.23);
        let expected_float = 12.56 * -34.23;
        let expected_fp = float_to_fix(expected_float);
        let result_fp = fp_mul(a_fp, b_fp);
        let result_float = fix_to_float(result_fp);
        let diff_fp = (result_fp - expected_fp).abs();
        let diff_float = (result_float - expected_float).abs();
        assert!(diff_fp < eps_fp);
        assert!(diff_float < eps_float);
    }

    #[test]
    fn divide_basic() {
        let a_fp: i32 = FP_ONE;
        let b_fp: i32 = 0;
        assert_eq!(fp_div(a_fp, b_fp), 0); // Divide by zero should be....???
        let a_fp: i32 = float_to_fix(1.0);
        let b_fp: i32 = float_to_fix(1.0);
        assert_eq!(fp_div(a_fp, b_fp), FP_ONE);
        let a_fp: i32 = float_to_fix(4.0);
        let b_fp: i32 = float_to_fix(2.0);
        assert_eq!(fix_to_float(fp_div(a_fp, b_fp)), 2.0);
        let a_fp: i32 = float_to_fix(6.0);
        let b_fp: i32 = float_to_fix(-2.0);
        assert_eq!(fix_to_float(fp_div(a_fp, b_fp)), -3.0);
    }

    #[test]
    fn divide_advanced() {
        let eps_float = EPSILON;
        let eps_fp = float_to_fix(eps_float);
        let a_fp: i32 = float_to_fix(31.23);
        let b_fp: i32 = float_to_fix(12.56);
        let expected_float = 31.23 / 12.56;
        let expected_fp: i32 = float_to_fix(expected_float);
        let result_fp = fp_div(a_fp, b_fp);
        let result_float = fix_to_float(result_fp);
        let diff_fp = (result_fp - expected_fp).abs();
        let diff_float = (result_float - expected_float).abs();
        assert!(diff_fp < eps_fp);
        assert!(diff_float < eps_float);
        let a_fp: i32 = float_to_fix(-31.23);
        let b_fp: i32 = float_to_fix(12.56);
        let expected_float = -31.23 / 12.56;
        let expected_fp: i32 = float_to_fix(expected_float);
        let result_fp = fp_div(a_fp, b_fp);
        let result_float = fix_to_float(result_fp);
        let diff_fp = (result_fp - expected_fp).abs();
        let diff_float = (result_float - expected_float).abs();
        assert!(diff_fp < eps_fp);
        assert!(diff_float < eps_float);
    }
}
