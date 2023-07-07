use clap::{App, Arg};
use plotchart::prelude::*;
use rand::Rng;
use utf8_slice;

fn main() {

    const WIDTH : &str = "100";

    let matches = App::new("Avatar image creator")
        .version("1.0")
        .author("Paddi")
        .arg(
            Arg::with_name("size")
                .short('s')
                .long("size")
                .value_name("SIZE")
                .help("Sets the width of the square image")
                .default_value(WIDTH),
        )
        .arg(
            Arg::with_name("name")
            .short('n').
            long("name").
            value_name("NAME").
            help("Sets the name of the person")
            .takes_value(true)
        )
        .arg(
            Arg::with_name("output")
            .short('o')
            .long("output")
            .value_name("OUTPUT")
            .help("Sets the output png file")
            .takes_value(true)
        )
        .get_matches();

    let width = matches
        .value_of("size")
        .unwrap_or(WIDTH)
        .parse::<u32>()
        .unwrap();

    let name: &str = matches
        .value_of("name")
        .expect("Name is required");

    let out_png: &str = matches
        .value_of("output")
        .expect("Output file is required");

    let name_len = name.chars().count();

    let i_name = if name_len > 2 {
        utf8_slice::slice(name, name_len-2, name_len)
    } else {
        name
    };

    draw_png(width, i_name, out_png).expect("Failed to draw png");

}

fn draw_png(width: u32, name: &str, out_png: &str) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new(out_png, (width, width)).into_drawing_area();
    root.fill(&get_rand_color())?;

    let font_size = (width as f64 * 0.45) as u32 as f64;
    let pos_x = (width as f64 * 0.16) as i32;
    let pos_y = (width as f64 * 0.3) as i32;

    root.draw(&Text::new(
        name,
        (pos_x, pos_y),
        ("微软雅黑", font_size).into_font().color(&WHITE),
    ))?;
    Ok(())
}

fn get_rand_color() -> RGBColor {
    let color_pool: [(u8, u8, u8); 6] = [
        (114, 101, 230),
        (255, 191, 0),
        (0, 162, 174),
        (245, 106, 0),
        (24, 144, 255),
        (96, 109, 128)
        ];
    let mut rng = rand::thread_rng();
    let idx = rng.gen_range(0..color_pool.len());
    RGBColor(color_pool[idx].0, color_pool[idx].1, color_pool[idx].2)
}