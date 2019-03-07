extern crate num;
extern crate image;

use num::Complex;
use std::str::FromStr;

use image::ColorType;
use image::png::PNGEncoder;
use std::fs::File;
use std::io::Write;

#[allow(dead_code)]
fn complex_square_add_loop(c: Complex<f64>) {
    let mut z = Complex{re: 0.0, im: 0.0};
    loop {
        z = z * z + c
    }
}

fn parse_pair<T: FromStr>(s: &str, sep: char)->Option<(T,T)> {
    match s.find(sep) {
        None => None,
        Some(index) => {
            match (T::from_str(&s[..index]), T::from_str(&s[index+1..])) {
                (Ok(i), Ok(j)) => Some((i,j)),
                _ => None
            }
        }
    }
}

#[test]
fn test_parse_pair() {
    assert_eq!(parse_pair::<i32>("", ','), None);
    assert_eq!(parse_pair::<i32>("10,", ','), None);
    assert_eq!(parse_pair::<i32>("10,10", ','), Some((10,10)));
    assert_eq!(parse_pair::<i32>("10x10", 'x'), Some((10,10)));
    assert_eq!(parse_pair::<f32>("10x10", 'x'), Some((10.,10.)));
    assert_eq!(parse_pair::<f32>("1.05x1.05", 'x'), Some((1.05,1.05)));
}


/// Parse a complex value from a string using ',' as the separator
fn parse_complex(s: &str)->Option<Complex<f64>> {
    match parse_pair::<f64>(s, ',') {
        None => None,
        Some((l, r)) => Some(Complex{re: l, im: r})
    }
}

#[test]
fn test_parse_complex() {
    assert_eq!(parse_complex(""), None);
    assert_eq!(parse_complex("10,10"), Some(Complex{re: 10., im: 10.}));
    assert_eq!(parse_complex("21.1,-35.2"), Some(Complex{re: 21.1, im: -35.2}));
}

fn pixel_to_point(bounds: (usize, usize), 
               pixel: (usize, usize),
               upper_left: Complex<f64>,
               lower_right: Complex<f64>) -> Complex<f64> {
    let (width, height) = (lower_right.re - upper_left.re, upper_left.im - lower_right.im);
    Complex {
        re: upper_left.re + pixel.0 as f64 * width / bounds.0 as f64,
        im: upper_left.im - pixel.1 as f64 * height / bounds.1 as f64
    }
}

#[test]
fn test_pixel_to_point() {
    assert_eq!(pixel_to_point((100,100), (25, 75), Complex{re: -1.0, im: 1.0}, Complex{re: 1.0, im: -1.0}), Complex{re: -0.5, im: -0.5});
}

fn render(pixels: &mut [u8],
        bounds: (usize, usize),
        upper_left: Complex<f64>,
        lower_right: Complex<f64>) {
    assert!(pixels.len() == bounds.0 * bounds.1);

    for row in 0..bounds.1 {
        for column in 0..bounds.0 {
            let point = pixel_to_point(bounds, (column, row), upper_left, lower_right);
            pixels[row * bounds.0 + column] = match escape_time(point, 200) {
                None => 0,
                Some(count) => 255 - count as u8
            };
        } 
    }
}

fn escape_time(c: Complex<f64>, limit: u32) -> Option<u32> {
    let mut z = Complex{re: 0.0, im: 0.0};
    for i in 0..limit {
        z = z*z + c;
        if z.norm_sqr() > 4.0 {
            return Some(i)
        }
    }
    return None
}

fn write_image(filename: &str, pixels: &[u8], bounds: (usize, usize)) -> Result<(), std::io::Error> {
    let output = File::create(filename)?;
    let encoder = PNGEncoder::new(output);

    encoder.encode(&pixels, bounds.0 as u32, bounds.1 as u32, ColorType::Gray(8))?;
    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 5 {
        writeln!(std::io::stderr(), "Usage: mandelbrot FILE PIXELS UPPERLEFT LOWERRIGHT").unwrap();
        writeln!(std::io::stderr(), "Example: {} foo.png 1000x750 -1.20,0.35 -1.0,0.20", args[0]).unwrap();
        std::process::exit(1);
    }

    let bounds = parse_pair(&args[2], 'x').expect("error parsing image dimensions");
    let upper_left = parse_complex(&args[3]).expect("error parsing upper left");
    let lower_right = parse_complex(&args[4]).expect("error parsing lower right");

    let mut pixels = vec![0;bounds.0 * bounds.1];
    render(&mut pixels, bounds, upper_left, lower_right);
    write_image(&args[1], &pixels, bounds).expect("error writing png file");
}

