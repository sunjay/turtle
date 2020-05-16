cfg_if::cfg_if! {
    if #[cfg(any(feature = "test", test))] {
        mod test;
        pub use test::*;

    } else if #[cfg(target_os = "macos")] {
        mod multiprocessed;
        pub use multiprocessed::*;

    } else {
        mod multithreaded;
        pub use multithreaded::*;
    }
}
