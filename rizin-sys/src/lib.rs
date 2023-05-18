pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[repr(C)]
struct RzAnalysis;

extern "C" {
    fn rz_analysis_set_limits(analysis: *mut RzAnalysis, from: u64, to: u64);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        unsafe {
            rz_analysis_set_limits(&mut RzAnalysis, 0, 1);
        }
        assert_eq!(4, 4);
    }
}
