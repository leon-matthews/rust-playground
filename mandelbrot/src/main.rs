
use std::env;

use mandelbrot;


fn main() {
    // Parse args
    let args: Vec<String> = env::args().collect();
    if args.len() != 5 {
        eprintln!("usage: {} FILE PIXELS TOP-LEFT BOTTOM-RIGHT", args[0]);
        eprintln!("eg. {} mandelbrot.png 1500x1000 -2.0,1.0 0.5,-1.0", args[0]);
        std::process::exit(1);
    }
    let bounds = mandelbrot::parse_pair::<usize>(&args[2], 'x')
        .expect("Error parsing image dimensions");
    let top_left = mandelbrot::parse_complex(&args[3])
        .expect("Error parsing top-left corner point");
    let bottom_right = mandelbrot::parse_complex(&args[4])
        .expect("Error parsing bottom-right corner point");

    // Render and save image
    let mut pixels = vec![0; bounds.0 * bounds.1];

    //~ render(&mut pixels, bounds, top_left, bottom_right);
    let cpus = num_cpus::get();
    mandelbrot::render_multicore(&mut pixels, bounds, top_left, bottom_right, cpus * 2);

    mandelbrot::write_image(&args[1], &pixels, bounds)
        .expect("Error writing PNG file");
}
