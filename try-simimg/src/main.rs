use simimgrs;
use image;

fn main() {
    let img1 = image::open("testdata/go1.jpg").unwrap();
    let img2 = image::open("testdata/go2.jpg").unwrap();

    let checker = simimgrs::SimilarChecker::new().threshold(10).compression_size(8, 8);

    println!("similar image: {}", checker.is_similar(img1, img2)) // true !
}