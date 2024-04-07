use std::path::PathBuf;
use coren::*;

fn main() {
  let p: PathBuf = PathBuf::from("./");
  dbg!(packs::iter_packs(&p).unwrap());
}
