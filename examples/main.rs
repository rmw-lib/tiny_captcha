use std::{env::current_exe, fs};
use tiny_captcha::{captcha, gifsize, makegif};

fn main() -> anyhow::Result<()> {
    let (word, img) = unsafe { captcha() };
    let word = unsafe { std::str::from_utf8_unchecked(&word) };
    println!("{}", word);
    let mut gif: [u8; gifsize as usize] = [0; gifsize as usize];
    unsafe {
        makegif(&img as *const u8 as *mut u8, &mut gif as *mut u8);
    }
    let exe = current_exe()?;
    let gif_path = exe.parent().unwrap().join("c.gif");
    println!("{}", gif_path.display());
    fs::write(gif_path, &gif)?;
    Ok(())
}
