use crate::form;
use std::fs;

pub fn gen_mem() -> String
{
   let path: &str = "test.txt";
   let file: Vec<u8> = fs::read(path).unwrap();
   
   let mut text: String = String::new();
   
   for i in 0..file.len()
   {
      if i % 8 == 0 && i != 0 { text = format!("{}  ", text); }
      if i % 16 == 0
      {
         if i != 0 { text = format!("{}\n", text); }
         text = format!("{}{}: ", text, form::address(i));
      }
      text = format!("{}{} ", text, form::uint_to_mem(file[i]).1);
   }
   return text;
}
