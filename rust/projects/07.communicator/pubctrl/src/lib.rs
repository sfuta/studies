mod outermost {
    pub fn middle_fn() {}
    fn middle_secret_fn() {}

    mod inside {
        pub fn inner_fn() {}
        fn inner_secret_fn() {}
    }
}

fn call_me() {
    outermost::middle_fn();
    // NG. No public
    // outermost::middle_secret_fn();
    // outermost::inside::inner_fn();
    // outermost::inside::inner_secret_fn();
}
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
