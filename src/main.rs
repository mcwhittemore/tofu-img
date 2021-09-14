use structopt::StructOpt;
use image::{open, ImageBuffer};

use std::cmp::max;
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
    let mut imgs = vec![];
    let num_imgs = cli.files.len();
    for i in 0..num_imgs {
        let path = cli.files.get(i).unwrap().as_path();
        println!("{:?}", path);
        imgs.push(open(path).unwrap().into_rgb8());
    }

    let img_one = imgs.get(0).unwrap();

    let img_out = ImageBuffer::from_fn(img_one.width(), img_one.height(), |x, y| {
        let mut v = 0;
        for i in 0..num_imgs {
            let img = imgs.get(i).unwrap();
            if img.width() > x && img.height() > y {
              let pix = imgs.get(i).unwrap().get_pixel(x,y);
              v = max(max(max(v,pix[0]),pix[1]),pix[2]);
            }
        }
        image::Rgb([v,v,v])
    });

    img_out.save(Path::new("./output.png")).unwrap();
}
