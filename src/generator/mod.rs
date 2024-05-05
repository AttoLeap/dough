pub mod text;

use std::io;

pub trait Generator {
    fn generate(&self, out: impl io::Write);
}
