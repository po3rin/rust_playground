// contents of client.rs
mod client;

mod network;

pub fn test_string() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn strint() {
        test_string()
    }
}
