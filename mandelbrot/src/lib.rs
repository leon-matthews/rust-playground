
use image::ColorType;
use image::png::PNGEncoder;
use num::Complex;
use std::fs::File;
use std::str::FromStr;


/**
Determine if `c` is in the Mandelbrot set, using at most `limit` iterations.

Returns the number of iterations needed to escape, if the point was able to do so:

    let point = num::Complex { re: 1.0, im: 1.0 };
    assert_eq!(mandelbrot::escape_time(point, 100), Some(2));

Otherwise `None`, if the point had still not escaped after `limit` iterations:

    let point = num::Complex { re: 0.0, im: 0.0 };
    assert_eq!(mandelbrot::escape_time(point, 100), None);
*/
pub fn escape_time(c: Complex<f64>, limit: usize) -> Option<usize> {
    let mut z = Complex { re: 0.0, im: 0.0 };
    for i in 0..limit {
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
        z = z * z + c;
    }
    None
}


/// Parse complex number from a pair of separated by a comma.
pub fn parse_complex(s: &str) -> Option<Complex<f64>> {
    match parse_pair(s, ',') {
        Some((re, im)) => Some(Complex { re, im }),
        None => None,
    }
}

#[test]
pub fn test_parse_complex() {
    assert_eq!(parse_complex("0,0"), Some(Complex { re: 0.0, im: 0.0 }));
    assert_eq!(
        parse_complex("1.25,-0.0625"),
        Some(Complex {
            re: 1.25,
            im: -0.0625
        })
    );
    assert_eq!(parse_complex(",-0.0625"), None);
}

/**
Generic function to parse string into a pair of values.

Type `T` must implement FromStr trait, `separator` must be ASCII.
*/
pub fn parse_pair<T: FromStr>(s: &str, separator: char) -> Option<(T, T)> {
    match s.find(separator) {
        None => None,
        Some(index) => match (T::from_str(&s[..index]), T::from_str(&s[index + 1..])) {
            (Ok(l), Ok(r)) => Some((l, r)),
            _ => None,
        },
    }
}

#[test]
fn test_parse_pair() {
    assert_eq!(parse_pair::<i32>("", ','), None);
    assert_eq!(parse_pair::<i32>("10,", ','), None);
    assert_eq!(parse_pair::<i32>(",10", ','), None);
    assert_eq!(parse_pair::<i32>("10,20", ','), Some((10, 20)));
    assert_eq!(parse_pair::<i32>("10,20xy", ','), None);
    assert_eq!(parse_pair::<f64>("0.5x", 'x'), None);
    assert_eq!(parse_pair::<f64>("0.5x1.5", 'x'), Some((0.5, 1.5)));
}

/**
Convert from pixel coordinates to point of the complex plain.

`bounds` is a pair giving the width and height of the image in pixels.
`pixel` is a (column, row) pair for a particular pixel, starting at top-left.
`top_left` and `bottom_right` parameters are points on the complex
plane designating the area our image covers.
*/
fn pixel_to_point(
    bounds: (usize, usize),
    pixel: (usize, usize),
    top_left: Complex<f64>,
    bottom_right: Complex<f64>,
) -> Complex<f64> {
    let (width, height) = (
        bottom_right.re - top_left.re,
        top_left.im - bottom_right.im,
    );

    let re = top_left.re + pixel.0 as f64 * width / bounds.0 as f64;
    let im = top_left.im - pixel.1 as f64 * height / bounds.1 as f64;
    Complex { re, im }
}

/**
As per `render()`, but split problem over multiple CPU threads.

TODO: Some bands are easier to calculate than others, but we have to wait
      for the slowest to finish. Better to create many more bands and
      feed them into a thread-pool.
*/
pub fn render_multicore(
    pixels: &mut [u8],
    bounds: (usize, usize),
    top_left: Complex<f64>,
    bottom_right: Complex<f64>,
    threads: usize,
) {
    let rows_per_band = bounds.1 / threads + 1;

    // Break pixel buffer into bands
    let bands: Vec<&mut [u8]> = pixels.chunks_mut(rows_per_band * bounds.0).collect();

    // Wait for all threads to finish execution
    crossbeam::scope(|spawner| {
        // Grant exclusive ownership to each band
        for (i, band) in bands.into_iter().enumerate() {
            // Calculate bounding box
            let top = rows_per_band * i;
            let height = band.len() / bounds.0;
            let band_bounds = (bounds.0, height);
            let band_top_left = pixel_to_point(
                bounds, (0, top), top_left, bottom_right);
            let band_bottom_right = pixel_to_point(
                bounds, (bounds.0, top + height), top_left, bottom_right);

            // Create thread, giving it ownership of band
            spawner.spawn(move |_| {
                render(band, band_bounds, band_top_left, band_bottom_right);
            });
        }
    }).unwrap();
}

/**
Render a rectangle of the Mandelbrot set into a buffer of pixels.

`bounds` gives the width and height of the `pixels` buffer, which holds one
greyscale pixel per byte. The `top_left` and `bottom_right` arguments
specify points on the complex plane being rendered.
*/
fn render(
    pixels: &mut [u8],
    bounds: (usize, usize),
    top_left: Complex<f64>,
    bottom_right: Complex<f64>,
) {
    assert!(pixels.len() == bounds.0 * bounds.1);

    for row in 0..bounds.1 {
        for column in 0..bounds.0 {
            let point = pixel_to_point(bounds, (column, row), top_left, bottom_right);
            let index = row * bounds.0 + column;
            pixels[index] = match escape_time(point, 255) {
                None => 0,
                Some(count) => 255 - count as u8,
            };
        }
    }
}


/// Create a greyscale PNG image from the pixel buffer.
pub fn write_image(
    filename: &str, pixels: &[u8], bounds: (usize, usize),
) -> Result<(), std::io::Error> {
    let output = File::create(filename)?;
    let encoder = PNGEncoder::new(output);
    encoder.encode(&pixels, bounds.0 as u32, bounds.1 as u32, ColorType::Gray(8))?;
    Ok(())
}


#[test]
fn test_pixel_to_point() {
    let bounds = (100, 200);

    // Complex plane from (-1, 1) to (1, -1)
    let top_left = Complex { re: -1.0, im: 1.0 };
    let bottom_right = Complex { re: 1.0, im: -1.0 };

    // Top-left (first pixel!)
    assert_eq!(
        pixel_to_point(bounds, (0, 0), top_left, bottom_right),
        Complex { re: -1.0, im: 1.0 },
        "Top-left pixel is incorrect",
    );

    // Top-right
    assert_eq!(
        pixel_to_point(bounds, (0, 200), top_left, bottom_right),
        Complex { re: -1.0, im: -1.0 },
        "Top-right pixel is incorrect",
    );

    // Bottom-left
    assert_eq!(
        pixel_to_point(bounds, (100, 0), top_left, bottom_right),
        Complex { re: 1.0, im: 1.0 },
        "Bottom-left pixel is incorrect",
    );

    // Bottom-right (last pixel)
    assert_eq!(
        pixel_to_point(bounds, (100, 200), top_left, bottom_right),
        Complex { re: 1.0, im: -1.0 },
        "Bottom-right pixel is incorrect",
    );

    // Somewhere in the middle
    assert_eq!(
        pixel_to_point(bounds, (25, 175), top_left, bottom_right),
        Complex { re: -0.5, im: -0.75 },
    );
}
