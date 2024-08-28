
pub mod some_lib_functions;
// pub mod more_lib_functions;

pub fn whats_up() {
    println!("What's up?");
}


pub fn add(left: usize, right: usize) -> usize {
    left + right
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
