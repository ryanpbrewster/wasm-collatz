#[macro_use]
extern crate cfg_if;
extern crate wasm_bindgen;
extern crate web_sys;

use wasm_bindgen::prelude::*;

cfg_if! {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function to get better error messages if we ever panic.
    if #[cfg(feature = "console_error_panic_hook")] {
        extern crate console_error_panic_hook;
        use console_error_panic_hook::set_once as set_panic_hook;
    } else {
        #[inline]
        fn set_panic_hook() {}
    }
}

cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

#[wasm_bindgen]
pub fn collatz_len_wasm(n: u32) -> usize {
    let mut n = n as u64;
    let mut count = 0;
    while n > 1 {
        n = if n % 2 == 0 { n / 2 } else { 3 * n + 1 };
        count += 1;
    }
    count
}

#[wasm_bindgen]
pub fn collatz_max_wasm(hi: u32) -> u32 {
  (1..=hi).max_by_key(|&n| collatz_len_wasm(n)).unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn len_smoke() {
        assert_eq!(collatz_len(1), 0);
        assert_eq!(collatz_len(2), 1);
        assert_eq!(collatz_len(3), 7);
        assert_eq!(collatz_len(4), 2);
        assert_eq!(collatz_len(5), 5);
    }

    #[test]
    fn max_smoke() {
        assert_eq!(collatz_max_wasm(1_000), 871);
        assert_eq!(collatz_max_wasm(1_000_000), 837799);
    }
}
