use std::env;

pub struct Args {
    pub file_path: String,
}

impl Args {
    pub fn parse(file: &str) -> Result<Self, String> {
        let mut args = env::args().skip(1);

        let file_path = args
            .next()
            .ok_or(file)?;

        Ok(Self { file_path })
    }
}
