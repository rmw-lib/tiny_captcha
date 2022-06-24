use anyhow::Result;
use std::{env::current_exe, fs};
use tiny_captcha::{captcha, gifsize, makegif};

fn main() -> anyhow::Result<()> {
    let mut img: [u8; 70 * 200] = [0; 70 * 200];
    let mut word: [u8; 6] = [0; 6];
    unsafe {
        captcha(&mut img as *mut u8, &mut word as *mut u8);
    }
    let word = unsafe { std::str::from_utf8_unchecked(&word) };
    let mut gif: [u8; gifsize as usize] = [0; gifsize as usize];
    unsafe {
        makegif(&mut img as *mut u8, &mut gif as *mut u8);
    }
    let exe = current_exe()?;
    let gif_path = exe.parent().unwrap().join("c.gif");
    println!("{}", gif_path.display().to_string());
    fs::write(gif_path, &gif);
    Ok(())
}
