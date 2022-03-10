pub mod tree;
pub mod helper;

pub use tree::LazySegtree;
pub use helper::LazySegtreeHelper;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
