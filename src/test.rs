#[cfg(test)] // 当 test 配置项存在时，才运行下面的代码，而 cargo test 在运行时，就会将 test 这个配置项传入进来。
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}