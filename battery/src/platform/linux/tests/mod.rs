/// This macro generates the bunch of files representing the
/// `/sys/class/power_supply/{name}/*` directory contents.
macro_rules! sysfs_test_suite {
    ( $( $name:expr => $value:expr ),* ) => {{
        use ::std::io::Write as _;

        let root = tempfile::tempdir().unwrap();

        $(
            let mut file = ::std::fs::OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(root.path().join($name))
                .unwrap();
            file.write_fmt(format_args!("{}\n", $value)).unwrap();
        )*

        root
    }};
}

mod issue_28;
mod issue_30;
