include!(concat!(env!("OUT_DIR"),"/bindings.rs"));

fn test1() {

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        unsafe {
        }
        assert_eq!(4, 4);
    }
}
