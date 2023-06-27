use rise::*;

fn main() {
    println!("Creating rise core");

    let mut core = Rise::new();
    core.load("/bin/ls");
    core.test();
}