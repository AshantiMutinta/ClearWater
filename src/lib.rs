#[macro_use]
extern crate lazy_static;

pub mod token_analysis;
pub mod syntax_analysis;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
