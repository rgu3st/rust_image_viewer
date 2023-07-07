use minifb::{Key, Window, WindowOptions, MouseMode};
use std::path::Path;
use std::thread;
use image::imageops;

/* 
fn update_buffer(buffer1, dim_amount, r, g, b){
    // The buffer should now have a byte for each pixel
		
}*/

fn load_sprite(sprite_path: &Path, sprite_size: u32) -> image::ImageBuffer<image::Rgb<u8>, Vec<u8>> {
    let sprite = image::open(sprite_path).expect("Oh noes! Couldn't open sprite.");
    let sprite_resized = sprite.resize_exact(sprite_size, sprite_size, imageops::FilterType::CatmullRom);
    let sprite_rgb = sprite_resized.into_rgb8();
    sprite_rgb  // Apparently, the last expression is returned by default...
}


fn main() {
	// Get image:
	let image_path = Path::new("test_rustacean.png");
	let img = image::open(&image_path).expect("Oh noes! Couldn't open image.");
	let img = img.to_rgb8(); // TODO: look up image type options!

    // Load and resize sprite1:
    let sprite1_path = Path::new("test_rustacean_sprite_med_crab_mech_1.png");
    let sprite1_buf = load_sprite(sprite1_path, 256);
    let mut sprite1_offset: (u32, u32) = (1280, 140);


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
    //let mut sprite_buffer1 = vec![0u32; sprite1.width() as usize * sprite1.height() as usize];
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
            let col_num = (i as u32) % (img.width() as u32);  // This works since modulo gives column number
            let row_num = (i as u32) / (img.width() as u32);  // This works since we're using integer division
            let rgb = pixel;
            
            // Fill the buffer with a=1 and rgb = what we got from the pixel 
            // For now,  just put on "shades" depending on mouse position:
            if i < (mouse_row * img.width() ) as usize{  
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


            //Layer sprite on top of image:
            // Adding offset from mouse position:
            sprite1_offset = (mouse_col, mouse_row);
            
            if  col_num < sprite1_offset.0 + sprite1_buf.width() 
                && col_num >= sprite1_offset.0
                && row_num < sprite1_offset.1 + sprite1_buf.height()
                && row_num >= sprite1_offset.1
                {
                let sprite_pixel = sprite1_buf.get_pixel(col_num-sprite1_offset.0, row_num-sprite1_offset.1);
                let sprite_rgb = sprite_pixel;

                // Knock out white background:
                let luma_key_low = 240 as u8;
                let luma_key_high = 255 as u8;
                if sprite_rgb[0] >= luma_key_low && sprite_rgb[1] >= luma_key_low && sprite_rgb[2] >= luma_key_low{
                    if sprite_rgb[0] >= luma_key_high && sprite_rgb[1] >= luma_key_high && sprite_rgb[2] >= luma_key_high{
                        r = sprite_rgb[0] as u32;
                        g = sprite_rgb[1] as u32;
                        b = sprite_rgb[2] as u32;
                }
                } else {
                    r = sprite_rgb[0] as u32;
                    g = sprite_rgb[1] as u32;
                    b = sprite_rgb[2] as u32;
                }

                //window.update_with_buffer(&buffer1, img.width() as usize, img.height() as usize).unwrap();
            }




            buffer1[i] = ( a << 24 ) | ( r << 16) | ( g << 8 ) | ( b );
        }
        window.update_with_buffer(&buffer1, img.width() as usize, img.height() as usize).unwrap();



        let one_sixtieth = std::time::Duration::from_millis(16);
        let one_120th = std::time::Duration::from_millis(8);
        let one_second = std::time::Duration::from_millis(1000);
        thread::sleep(one_120th);  // Lock "frame rate" to 120fps
        //thread::sleep(one_sixtieth);  // Lock "frame rate" to 60fps
        //thread::sleep(one_second);  // Lock "frame rate" to 1fps

        //println!("Mouse cursor at: ({}, {})", mouse_row, mouse_col);
    
    }

}
