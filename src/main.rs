use minifb::{Key, Window, WindowOptions, MouseMode};
use std::path::Path;
use std::thread;
//use image::DynamicImage;

/* 
fn update_buffer(buffer1, dim_amount, r, g, b){
    // The buffer should now have a byte for each pixel
		
}*/

fn main() {
	// Get image:
	let image_path = Path::new("test_rustacean.png");
	let img = image::open(&image_path).expect("Oh noes! Couldn't open image.");
	let img = img.to_rgb8(); // TODO: look up image type options!

	let mut window = Window::new(
		"Rob's Image Viewer!",
		img.width() as usize,
		img.height() as usize,
		WindowOptions::default(),  // TODO: look up options, ensure I need this final comma...
	)
	.expect("Oh noes! Couldn't create GUI window.");


    

	// Buffer for the image data
	// I'm sure I could use this for an interactive video by creating two buffers, display and update and swap. Classic.
	// "vec!" is a provided macro to "create a vetor and hold the values we provide". (Bing search.)
	let mut buffer1 = vec![0u32; img.width() as usize * img.height() as usize];
    let mut buffer2 = vec![0u32; img.width() as usize * img.height() as usize];
    let mut r = 0u32;
    let mut g = 0u32;
    let mut b = 0u32;
    let mut dim_amount = 0 as u32;

	

    
	// Main Loop!
    

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way

        
        let mouse_pos = window.get_mouse_pos(MouseMode::Clamp).unwrap_or((0.0, 0.0));
        let mouse_col = mouse_pos.0 as u32;
        let mouse_row = mouse_pos.1 as u32;
        //println!("Mouse cursor at: ({}, {})", mouse_row, mouse_col);

        //update_buffer(buffer1, dim_amount, r, g, b);
        for (i, pixel) in img.pixels().enumerate(){
            let rgb = pixel;
            // Fill the buffer with a=1 and rgb = what we got from the pixel 
            // For now,  just put on "shades" depending on mouse position:
            if i < (mouse_row * mouse_col * 2) as usize{  // Why do I need the 2?
                dim_amount = 25;
            } else {
                dim_amount = 0;
            }

            if rgb[0] as u32 > dim_amount{
                r = (rgb[0] as u32) - dim_amount;
            } else {
                r = 0;
            }
            if rgb[1] as u32 > dim_amount * 2{
                g = (rgb[1] as u32) - dim_amount * 2;
            } else {
                g = 0;
            }
            if rgb[2] as u32 > dim_amount * 3 {
                b = (rgb[2] as u32) - dim_amount * 3;
            } else {
                b = 0;
            }
            let a = 0xFF;
            buffer1[i] = ( a << 24 ) | ( r << 16) | ( g << 8 ) | ( b );
        }
        window.update_with_buffer(&buffer1, img.width() as usize, img.height() as usize).unwrap();



        let one_sixtieth = std::time::Duration::from_millis(16);
        let one_second = std::time::Duration::from_millis(1000);
        thread::sleep(one_sixtieth);  // Lock "frame rate" to 60fps
        //thread::sleep(one_second);  // Lock "frame rate" to 1fps

    
    }

}
