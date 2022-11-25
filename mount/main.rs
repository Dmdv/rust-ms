extern crate iron;
extern crate staticfile;
extern crate mount;

fn main() {
    let mut mount = mount::Mount::new();

    mount.mount("/", staticfile::Static::new(std::path::Path::new("public/")));

    iron::Iron::new(mount).http("127.0.0.1:3000").unwrap();
}
