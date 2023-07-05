use minifb::{Key, Window, WindowOptions}
use std::path::Path;
use image::DynamicImage;

fn main() {
	// Get image:
	let image_path = Path::new("test_rustacean.png");
	let img = image::open(&image_path).expect("Oh noes! Couldn't open image.")
	let image.to_rgb8(); // TODO: look up image type options!

	let mut window = Window::new(
		"Rob's Image Viewer!",
		img.width() as usize,
		img.height() as usize,
		WindowOptions::defualt(),  // TODO: look up options, ensure I need this final comma...
	)
	.expect("Oh noes! Couldn't create GUI window.");

	// Buffer for the image data
	// I'm sure I could use this for an interactive video by creating two buffers, display and update and swap. Classic.
	// "vec!" is a provided macro to "create a vetor and hold the values we provide". (Bing search.)
	let mut buffer = vec![0u32; img.width as usize * img.height as usize];
	// The buffer should now have a byte for each pixel
	for (i, pixel) in img.pixels().enumerate(){
		let rgb = pixel.2;
		// Fill the buffer with a=1 and rgb = what we got from the pixel ^^
		let r = rgb[0] as u32;
		let g = rgb[1] as u32;
		let b = rgb[2] as u32;
		let a = 0xFF;
		buffer[i] = ( a << 24 ) | ( r << 16) | ( g << 8 ) | ( b );
	}	

	// Main Loop!
	while window.is_open() && !window.is_key_down(Key::Escape){ // the "is_key_down" is good to know. TODO: look up mouse events.
		window.update_with_buffer(&buffer).unwrap();
	}
}
