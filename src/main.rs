use minifb::{Key, Window, WindowOptions, MouseMode};
use std::path::Path;
use std::thread;
use image::imageops;
use std::fs::File;



fn load_sprite(sprite_path: &Path, sprite_size: u32) -> image::ImageBuffer<image::Rgb<u8>, Vec<u8>> {
    let sprite = image::open(sprite_path).expect("Oh noes! Couldn't open sprite.");
    let sprite_resized = sprite.resize_exact(sprite_size, sprite_size, imageops::FilterType::CatmullRom);
    let sprite_rgb = sprite_resized.into_rgb8();
    sprite_rgb  // Apparently, the last expression is returned by default...
}

fn sub(minuend: u32, subtruend: u32) -> u32{
    if minuend < subtruend{
        return 0;
    }
    minuend - subtruend
}

fn main() {
    // Attributes:
    let frame_rate: u32 = 60;
    let char_speed: u32 = 15;
    let mut moving_left = false;
    let char_size: u32 = 296;

	// Get image:
	let image_path = Path::new("bg_scene_1.png");
	let img = image::open(&image_path).expect("Oh noes! Couldn't open image.");
	let img = img.to_rgb8(); // TODO: look up image type options!

    // Load and resize sprite1:
    let sprite1_path = Path::new("test_rustacean_sprite_med_crab_mech_1.png");
    let sprite1_buf = load_sprite(sprite1_path, char_size);
    let mut sprite1_offset: (u32, u32) = (256, 520);


    // Load the cursor:
    let cursor_path = Path::new("cursor_2.png");
    let cursor_buf = load_sprite(cursor_path, 64);
    let mut cursor_offset: (u32, u32) = (0, 0);

	let mut window = Window::new(
		"Rob's Image Viewer!",
		img.width() as usize,
		img.height() as usize,
		WindowOptions::default(),  // TODO: look up options, ensure I need this final comma...
	)
	.expect("Oh noes! Couldn't create GUI window.");

    window.set_cursor_visibility(false);


    
	// Buffer for the image data
	// "vec!" is a provided macro to "create a vetor and hold the values we provide". (Bing search.)
	let mut buffer1 = vec![0u32; img.width() as usize * img.height() as usize];
    //let mut sprite_buffer1 = vec![0u32; sprite1.width() as usize * sprite1.height() as usize];
    let mut r = 0u32;
    let mut g = 0u32;
    let mut b = 0u32;

    
	// Main Loop!
    while window.is_open() && !window.is_key_down(Key::Escape) {
        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        let mouse_pos = window.get_mouse_pos(MouseMode::Clamp).unwrap_or((0.0, 0.0));
        let mouse_col = mouse_pos.0 as u32;
        let mouse_row = mouse_pos.1 as u32;
        //println!("Mouse cursor at: ({}, {})", mouse_row, mouse_col);

        
        // Changing offset from mouse position to keyboard input:
        if window.is_key_down(Key::Left) && sprite1_offset.0 > 0{
            sprite1_offset.0 = sub(sprite1_offset.0, char_speed);
            moving_left = true;
        }
        else if window.is_key_down(Key::Right) && sprite1_offset.0 < sub(img.width(), sprite1_buf.width()){
            sprite1_offset.0 += char_speed;
            moving_left = false;
        }

        /*
            ALL THE PIXELS!
         */
        for (i, pixel) in img.pixels().enumerate(){
            let col_num = (i as u32) % (img.width() as u32);  // This works since modulo gives column number
            let row_num = (i as u32) / (img.width() as u32);  // This works since we're using integer division
            let rgb = pixel;
            
            // Fill the buffer with a=1 and rgb = what we got from the pixel 
            r = (rgb[0] as u32);
            g = (rgb[1] as u32);
            b = (rgb[2] as u32);
            let a = 0xFF;

            //Layer sprite on top of image:
            // Adding offset from mouse position, use this for a mouse cursor still:
            // cursor_offset.0 = sub(mouse_col, cursor_buf.width()); 
            // cursor_offset.1 = sub(mouse_row, cursor_buf.height());
            cursor_offset.0 = mouse_col; 
            cursor_offset.1 = mouse_row;

           
            
            if  col_num < sprite1_offset.0 + sprite1_buf.width() 
                && col_num >= sprite1_offset.0
                && row_num < sprite1_offset.1 + sprite1_buf.height()
                && row_num >= sprite1_offset.1
                {
                
                // Add in a stipulation so that the sprite faces left/right when moving left/right:
                let mut sprite1_pixel = (col_num-sprite1_offset.0, row_num-sprite1_offset.1);
                if moving_left{
                    sprite1_pixel.0 = sprite1_buf.width() - sprite1_pixel.0 - 1;
                }
                let sprite_pixel = sprite1_buf.get_pixel(sprite1_pixel.0, sprite1_pixel.1);
                let sprite_rgb = sprite_pixel;

                // Knock out white background:
                let luma_key_low = 1 as u8;
                if sprite_rgb[0] > luma_key_low && sprite_rgb[1] > luma_key_low && sprite_rgb[2] > luma_key_low{
                    r = sprite_rgb[0] as u32;
                    g = sprite_rgb[1] as u32;
                    b = sprite_rgb[2] as u32;
                }
            }

            // Layer cursor on top of all:
            if  col_num < cursor_offset.0 + cursor_buf.width()
                && col_num >= cursor_offset.0
                && row_num < cursor_offset.1 + cursor_buf.height()
                && row_num >= cursor_offset.1
                {
                    let cursor_pixel = cursor_buf.get_pixel(col_num-cursor_offset.0, row_num-cursor_offset.1);
                    r += cursor_pixel[0] as u32;
                    g += cursor_pixel[1] as u32;
                    b += cursor_pixel[2] as u32;

                    if r > 255{
                        r = 255;
                    }
                    if g > 255{
                        g = 255;
                    }
                    if b > 255{
                        b = 255;
                    }
                }


            buffer1[i] = ( a << 24 ) | ( r << 16) | ( g << 8 ) | ( b );
        }
        window.update_with_buffer(&buffer1, img.width() as usize, img.height() as usize).unwrap();



        let delta_time = std::time::Duration::from_millis(1000 / frame_rate as u64);
        thread::sleep(delta_time);  // Lock "frame rate" 
    
    }

}
