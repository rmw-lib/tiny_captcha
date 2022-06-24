use anyhow::Result;
use std::{env::current_exe, fs::File};
use tiny_captcha::{captcha, HEIGHT, WIDTH};

fn main() -> Result<()> {
    for i in 1..=10 {
        let (word, mut img) = unsafe { captcha() };
        let word = unsafe { std::str::from_utf8_unchecked(&word) };
        println!("{}", word);
        let exe = current_exe()?;
        let gif_path = exe.parent().unwrap().join(format!("{}.gif", i));
        println!("{}", gif_path.display());
        let mut out = File::create(gif_path)?;

        let mut encoder = gif::Encoder::new(&mut out, WIDTH, HEIGHT, &[0, 0, 0, 0xFF, 0xFF, 0xFF])?;

        for i in img.iter_mut() {
            if *i == 255 {
                *i = 1;
            }
        }

        let frame = gif::Frame::from_indexed_pixels(WIDTH, HEIGHT, &img, None);

        encoder.write_frame(&frame)?;

        Ok(())
    }
}
