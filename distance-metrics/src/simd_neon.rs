#[cfg(target_feature = "neon")]
use std::arch::aarch64::*;

#[cfg(target_feature = "neon")]
pub(crate) unsafe fn euclid_distance_neon(v1: &[f32], v2: &[f32]) -> f32 {
    let n = v1.len();
    let m = n - (n % 16);
    let mut ptr1: *const f32 = v1.as_ptr();
    let mut ptr2: *const f32 = v2.as_ptr();
    let mut sum1 = vdupq_n_f32(0.);
    let mut sum2 = vdupq_n_f32(0.);
    let mut sum3 = vdupq_n_f32(0.);
    let mut sum4 = vdupq_n_f32(0.);

    let mut i: usize = 0;
    while i < m {
        let sub1 = vsubq_f32(vld1q_f32(ptr1), vld1q_f32(ptr2));
        sum1 = vfmaq_f32(sum1, sub1, sub1);

        let sub2 = vsubq_f32(vld1q_f32(ptr1.add(4)), vld1q_f32(ptr2.add(4)));
        sum2 = vfmaq_f32(sum2, sub2, sub2);

        let sub3 = vsubq_f32(vld1q_f32(ptr1.add(8)), vld1q_f32(ptr2.add(8)));
        sum3 = vfmaq_f32(sum3, sub3, sub3);

        let sub4 = vsubq_f32(vld1q_f32(ptr1.add(12)), vld1q_f32(ptr2.add(12)));
        sum4 = vfmaq_f32(sum4, sub4, sub4);

        ptr1 = ptr1.add(16);
        ptr2 = ptr2.add(16);
        i += 16;
    }
    let mut result = vaddvq_f32(sum1) + vaddvq_f32(sum2) + vaddvq_f32(sum3) + vaddvq_f32(sum4);
    for i in 0..n - m {
        result += (*ptr1.add(i) - *ptr2.add(i)).powi(2);
    }
    result.abs().sqrt()
}

#[cfg(target_feature = "neon")]
pub(crate) unsafe fn cosine_preprocess_neon(vector: &mut [f32]) {
    let n = vector.len();
    let m = n - (n % 16);
    let mut ptr: *const f32 = vector.as_ptr();
    let mut sum1 = vdupq_n_f32(0.);
    let mut sum2 = vdupq_n_f32(0.);
    let mut sum3 = vdupq_n_f32(0.);
    let mut sum4 = vdupq_n_f32(0.);

    let mut i: usize = 0;
    while i < m {
        let d1 = vld1q_f32(ptr);
        sum1 = vfmaq_f32(sum1, d1, d1);

        let d2 = vld1q_f32(ptr.add(4));
        sum2 = vfmaq_f32(sum2, d2, d2);

        let d3 = vld1q_f32(ptr.add(8));
        sum3 = vfmaq_f32(sum3, d3, d3);

        let d4 = vld1q_f32(ptr.add(12));
        sum4 = vfmaq_f32(sum4, d4, d4);

        ptr = ptr.add(16);
        i += 16;
    }
    let mut length = vaddvq_f32(sum1) + vaddvq_f32(sum2) + vaddvq_f32(sum3) + vaddvq_f32(sum4);
    for v in vector.iter().take(n).skip(m) {
        length += v.powi(2);
    }
    if length < f32::EPSILON {
        return;
    }
    let length = length.sqrt();
    for x in vector.iter_mut() {
        *x /= length;
    }
}

#[cfg(target_feature = "neon")]
pub(crate) unsafe fn dot_similarity_neon(v1: &[f32], v2: &[f32]) -> f32 {
    let n = v1.len();
    let m = n - (n % 16);
    let mut ptr1: *const f32 = v1.as_ptr();
    let mut ptr2: *const f32 = v2.as_ptr();
    let mut sum1 = vdupq_n_f32(0.);
    let mut sum2 = vdupq_n_f32(0.);
    let mut sum3 = vdupq_n_f32(0.);
    let mut sum4 = vdupq_n_f32(0.);

    let mut i: usize = 0;
    while i < m {
        sum1 = vfmaq_f32(sum1, vld1q_f32(ptr1), vld1q_f32(ptr2));
        sum2 = vfmaq_f32(sum2, vld1q_f32(ptr1.add(4)), vld1q_f32(ptr2.add(4)));
        sum3 = vfmaq_f32(sum3, vld1q_f32(ptr1.add(8)), vld1q_f32(ptr2.add(8)));
        sum4 = vfmaq_f32(sum4, vld1q_f32(ptr1.add(12)), vld1q_f32(ptr2.add(12)));
        ptr1 = ptr1.add(16);
        ptr2 = ptr2.add(16);
        i += 16;
    }
    let mut result = vaddvq_f32(sum1) + vaddvq_f32(sum2) + vaddvq_f32(sum3) + vaddvq_f32(sum4);
    for i in 0..n - m {
        result += (*ptr1.add(i)) * (*ptr2.add(i));
    }
    result
}

#[cfg(test)]
mod tests {
    #[cfg(target_feature = "neon")]
    #[test]
    fn test_spaces_neon() {
        use super::*;
        use crate::*;

        if std::arch::is_aarch64_feature_detected!("neon") {
            let v1: Vec<f32> = vec![
                10., 11., 12., 13., 14., 15., 16., 17., 18., 19., 20., 21., 22., 23., 24., 25.,
                26., 27., 28., 29., 30., 31.,
            ];
            let v2: Vec<f32> = vec![
                40., 41., 42., 43., 44., 45., 46., 47., 48., 49., 50., 51., 52., 53., 54., 55.,
                56., 57., 58., 59., 60., 61.,
            ];

            let euclid_simd = unsafe { euclid_distance_neon(&v1, &v2) };
            let euclid = euclid_distance(&v1, &v2);
            assert_eq!(euclid_simd, euclid);

            let dot_simd = unsafe { dot_similarity_neon(&v1, &v2) };
            let dot = dot_similarity(&v1, &v2);
            assert_eq!(dot_simd, dot);

            let mut v1 = v1;
            let mut v1_copy = v1.clone();
            unsafe { cosine_preprocess_neon(&mut v1) };
            cosine_preprocess(&mut v1_copy);
            assert_eq!(v1, v1_copy);
        } else {
            println!("neon test skipped");
        }
    }
}