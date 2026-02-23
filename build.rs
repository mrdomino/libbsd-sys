fn main() {
    pkg_config::Config::new()
        .atleast_version("0.8")
        .probe("libbsd")
        .expect("libbsd not found; install libbsd-dev");
}
