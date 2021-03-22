#[cfg(test)]

const FP_SHIFT_AMT : i32 = 16;
const FP_ONE : i32 = 1 << 16;
const FP_LSB : f32 = 1.0 / ((1 << 16) as f32);

fn float_to_fix(a : f32) -> i32 {
    (a / FP_LSB) as i32
}

fn fix_to_float(a : i32) -> f32 {
    a as f32 * FP_LSB
}

fn fp_mul(a : i32, b : i32) -> i32 {
    let mul_result = a as i64 * b as i64;
    let result = mul_result >> 16;
    result as i32
}

mod test {
    use super::*;
    const EPSILON : f32 = FP_LSB;
    const VAL_A : f32 = 1234.5678;
    const VAL_A_FP : i32 = (VAL_A * (1 << 16) as f32) as i32;
    const VAL_B : f32 = -8675.309;
    const VAL_B_FP : i32 = (VAL_B * (1 << 16) as f32) as i32;

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
        let a_fp : i32 = 0;
        let b_fp : i32 = 0;
        assert_eq!(fp_mul(a_fp, b_fp), 0);
        let a_fp : i32 = FP_ONE;
        let b_fp : i32 = 0;
        assert_eq!(fp_mul(a_fp, b_fp), 0);
        let a_fp : i32 = float_to_fix(1.0);
        let b_fp : i32 = float_to_fix(1.0);
        assert_eq!(fp_mul(a_fp, b_fp), FP_ONE);
    }

    #[test]
    fn multiply_advanced() {
        let expected = VAL_A * VAL_B;
        let a_fp : i32 = float_to_fix(VAL_A);
        let b_fp : i32 = float_to_fix(VAL_B);
        let result = fp_mul(a_fp, b_fp);
        let result_float = fix_to_float(result);
        println!("{:?} vs {:?}", result_float, expected);
        assert!((result_float - expected).abs() < EPSILON);
    }
}
