pub fn count_set_bits(mut n: u128) -> u8 {
    let mut cnt: u8 = 0;
    while n > 0 {
        if (n & 1) == 1 {
            cnt += 1
        }
        n >>= 1
    }
    cnt
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = count_set_bits(999);
        assert_eq!(result, 8);
    }
}
