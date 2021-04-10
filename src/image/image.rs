use std::fs::File;
use std::io::prelude::*;

pub trait Image<T> {
    fn render(&self) -> Vec<u8>;
    fn render_to_file(&self, filename: &str) -> std::io::Result<()> {
        let mut file = File::create(filename)?;
        file.write_all(&self.render())?;
        Ok(())
    }
}
