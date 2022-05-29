use std::fs::File;
use std::io::Write;

fn main() {
    //image creation test
    create_ppm("./examples/image-test.ppm", 3, 1, &vec!(0xFF00FF, 0x00FFFF, 0xFFFF00));
    
    //rule 30 test
    println!("{}",rule30(true, false, true));
}

/** Create PPM image file */
fn create_ppm(path: &str, width: usize, height: usize, pixel_data: &[i32]) {
	// Creating the file
	let mut image_file = File::create(path).expect("Unable to create file");

	//Writing PPM header (rules 1 through 8) 
	write!(image_file, "P6\n{} {}\n255\n", width, height).expect("Unable to write to file");

	//Writing pixel data 
	for pixel in pixel_data {
		let color: [u8; 3] = [(pixel >> 16) as u8, (pixel >> 8) as u8, (pixel >> 0) as u8];
		image_file.write_all(&color).expect("Unable to write to file");
	}
}

fn rule30 (p: bool, q:bool, r: bool) -> bool{
	(!p && r) || (!p && q) || (p && !q && !r)
}