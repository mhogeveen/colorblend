use clap::Parser;
use regex::Regex;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    hex1: String,
    hex2: String,
    #[arg(default_value_t = 1.0)]
    alpha: f32,
}

fn main() {
    let args = Args::parse();

    let is_hex1_valid = validate_hex(&args.hex1);
    let is_hex2_valid = validate_hex(&args.hex2);

    if is_hex1_valid && is_hex2_valid {
        let color_blend = blend(&args.hex1, &args.hex2, &args.alpha);

        println!("{}", color_blend);
    } else {
        println!("Please supply valid hex values");
    }
}

fn blend(hex1: &String, hex2: &String, alpha: &f32) -> String {
    let [r1, g1, b1] = hex_to_rgb(&hex1);
    let [r2, g2, b2] = hex_to_rgb(&hex2);

    let r = alpha * r1 + ((1.0 - alpha) * r2);
    let g = alpha * g1 + ((1.0 - alpha) * g2);
    let b = alpha * b1 + ((1.0 - alpha) * b2);

    rgb_to_hex([r, g, b])
}

fn rgb_to_hex(rgb: [f32; 3]) -> String {
    let h1 = format!("{:x}", rgb[0] as u8);
    let h2 = format!("{:x}", rgb[1] as u8);
    let h3 = format!("{:x}", rgb[2] as u8);
    
    format!("#{}{}{}", h1, h2, h3)
}

fn hex_to_rgb(hex: &String) -> [f32; 3] {
    let clean_hex = hex.replace("#", "");
    let mut rgb: [f32; 3] = [0.0; 3];

    if clean_hex.len() == 6 {
        for (i, j) in [(0, 0), (1, 2), (2, 4)] {
            let hex_slice = &clean_hex[j..=j + 1];
            let hex_slice = u8::from_str_radix(hex_slice, 16).unwrap();
            let hex_slice: f32 = hex_slice.into();
            rgb[i] = hex_slice;
        }
    } else {
        for i in 0..2 {
            let hex_slice = &clean_hex[i..=i + 1];
            let hex_slice = u8::from_str_radix(hex_slice, 16).unwrap();
            let hex_slice: f32 = hex_slice.into();
            rgb[i] = hex_slice;
        }
    }
    rgb
}


fn validate_hex(hex: &String) -> bool {
    let regex = Regex::new("^#?([A-Fa-f0-9]{6}|[A-Fa-f0-9]{3})$").unwrap();
    regex.is_match(hex)
}
