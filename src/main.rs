use structopt::StructOpt;
use image::{open, ImageBuffer};

use std::path::PathBuf;
use std::path::Path;

#[derive(StructOpt)]
struct Cli {
    #[structopt(parse(from_os_str), required = true)]
    files: Vec<PathBuf>
}

fn main() {
    let cli = Cli::from_args();
    //println!("{:?}", cli.files[0]);
    let img_one = open(cli.files.get(0).unwrap().as_path()).unwrap().into_rgb8();

    let img_out = ImageBuffer::from_fn(img_one.width(), img_one.height(), |x, y| {
        let pix = img_one.get_pixel(x,y);
        let v = std::cmp::max(std::cmp::max(pix[0], pix[1]), pix[2]);
        image::Rgb([v,v,v])
    });

    img_out.save(Path::new("./output.png")).unwrap();
}
