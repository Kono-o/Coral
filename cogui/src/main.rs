use std::path::PathBuf;
use coren::*;

fn main() {
  match packs::get_packs(&PathBuf::from("/home/kono/.minecraft/resourcepacks")) {
    Ok(..) => println!("worked"),
    Err(err) => println!("{:?}", err)
  }
  match packs::get_packs(&PathBuf::from("./res")) {
    Ok(..) => println!("worked"),
    Err(err) => println!("{:?}", err)
  }
}
