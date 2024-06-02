pub fn add(left: usize, right: usize, add_param: usize) -> usize {
    left + right + add_param
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2, 3);
        assert_eq!(result, 7);
    }
}
