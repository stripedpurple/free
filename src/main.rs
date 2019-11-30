extern crate sys_info;
extern crate getopts;

use getopts::Options;
use std::env;
use sys_info::os_type;
use sys_info::mem_info;

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optflag("b", "", "prints values in bytes");
    opts.optflag("k", "", "prints values in kilobytes (DEAFAULT)");
    opts.optflag("m", "", "prints values in megabytes");
    opts.optflag("g", "", "prints values in gigabytes");
    opts.optflag("h", "help", "print this help menu");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    let mut diviser = 1 as f64;
    let mut label = "KB";
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    };
    if matches.opt_present("b") {
        diviser = 0.001;
        label = "B";
    };
    if matches.opt_present("m") {
        diviser = 1024.0;
        label = "MB";
    };
    if matches.opt_present("g") {
        diviser = 1048576.0;
        label = "GB";
    };

    let os = os_type().unwrap();
    let mem = mem_info().unwrap();

    println!("{:<10} {:<14} {:<14} {:<14} {:<14} {:<14}",label,"total","free","avail","buffers","cached");
    print!("{name:<10} {:<14.2} {:<14.2} {:<14.2} ",
            mem.total as f64 / diviser, mem.free as f64 / diviser, mem.avail as f64 / diviser, name="mem:");
    if os == "Linux" {
        print!("{:<14.2} {:<14.2}",
                mem.buffers as f64 / diviser, mem.cached as f64 / diviser)
    }
    println!();
    println!("{name:<10} {:<14.2} {:<14.2}",
                mem.swap_total as f64 / diviser, mem.swap_free as f64 / diviser, name="swap:");
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}
