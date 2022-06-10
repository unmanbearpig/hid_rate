use std::env;
use std::fs::File;
use std::io::{self, Read};
use std::time;

fn parse_dev_path_from_args(mut args: env::Args) -> io::Result<String> {
    let path = match args.next() {
        Some(path) => path,
        None => return  Err(io::Error::new(
            io::ErrorKind::Other,
            "Please specify dev file path in args".to_string())),
    };

    if args.next().is_some() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "extra argument specified".to_string()));
    }

    Ok(path.to_string())
}

fn main() -> io::Result<()> {
    let mut args = env::args();
    args.next(); // skip executable name
    let dev_path = parse_dev_path_from_args(args)?;
    println!("dev_path = {}", dev_path);

    let mut file = File::open(dev_path)?;
    let mut buf = [0u8; 128];

    let mut last_time;
    loop {
        last_time = time::Instant::now();
        let bytes_read = file.read(&mut buf)?;
        let elapsed = last_time.elapsed();
        let secs = elapsed.as_secs_f64();
        let hz = 1.0 / secs;

        println!("{:14} ns, {:>14.3} us, {:10.5} hz, {:4} bytes read",
                 elapsed.as_nanos(), secs * 1000_000.0, hz, bytes_read);
    }
}
