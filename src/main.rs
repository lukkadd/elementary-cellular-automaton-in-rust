use std::fs::File;
use std::io::Write;

fn main() {
    //image creation test
    create_ppm("./examples/image-test.ppm", 3, 1, &vec!(0xFF00FF, 0x00FFFF, 0xFFFF00));
    
    //rule 30 test
    println!("{}",rule30(true, false, true));

    //cellular automaton test (data only)

    //Defining ECA configuration
    const NUMBER_OF_CELLS: usize = 64;
	const ITERATIONS: usize = 20;
	
    //Defining ECA initial state (first row)
	let mut initial_state: Vec<bool> = vec![false; NUMBER_OF_CELLS];
	initial_state[(NUMBER_OF_CELLS / 2)] = true;

    //Generating ECA data over multiple generations 
    let eca_data = eca(&initial_state, rule30, ITERATIONS);
    println!("{:?}", eca_data);

    //boolean to color conversion test
    let color_data = bool_to_color(&eca_data);
    println!("{:?}", color_data);
}

// Create PPM image file
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

// Convert bool vector to i32 vector where false means white and true means black 
fn bool_to_color (bool_data: &Vec<bool>) -> Vec<i32> {
	let mut color_data: Vec<i32> = Vec::new();
	for i in 0..bool_data.len() {
		color_data.push(if bool_data[i] { 0x0 } else {0xFFFFFF});
	}
	return color_data;
}

// ECA rule 30 defined in a boolean function
fn rule30 (p: bool, q:bool, r: bool) -> bool{
	(!p && r) || (!p && q) || (p && !q && !r)
}

// Generate ECA data based on a given initial state, rule and number of iterations
fn eca (initial_state: &Vec<bool>, rule: fn(bool, bool, bool) -> bool, iterations: usize) -> Vec<bool> {
	let mut result: Vec<bool> = Vec::new();
	let number_of_cells = initial_state.len();

	result.append(&mut initial_state.clone());

	for i in 0..iterations+1 {
		result.append(&mut eca_step(&result[(i*number_of_cells)..((i+1)*number_of_cells)], rule));
	}

	return result;
}

// Generate a single iteration of the ECA
fn eca_step (current_state: &[bool], rule: fn(bool, bool, bool)->bool) -> Vec<bool> {
	if current_state.len() < 3 {panic!("length {}", current_state.len())} //Must have at least 3 cells for the rule to apply
	
	let mut result: Vec<bool> = Vec::new();

	// Iterate over every cell
	for i in 0..current_state.len() {
		if i == 0 {
			// First cell, loop back
			result.push(rule(current_state[current_state.len() - 1], current_state[i], current_state[i+1]));
		}else if i == current_state.len() - 1 {
			// Last cell, loop forward
			result.push(rule(current_state[i-1], current_state[i], current_state[0]));
		}else {
			result.push(rule(current_state[i-1], current_state[i], current_state[i+1]));
		}
    }

	return result;
}