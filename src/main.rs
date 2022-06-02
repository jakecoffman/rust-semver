use semver::{VersionReq};

fn main() {
    println!("Hello, world!");

    // panic: unexpected character '*' while parsing major version number
    let req = VersionReq::parse("> *").unwrap();

    println!("{}", req)
}
