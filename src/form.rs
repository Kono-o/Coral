pub fn address(address: usize) -> String
{
   let mut address = format!("{:X}", address);
   if address.len() < 8
   {
      for _i in 0..(8 - address.len())
      {
         address = format!("{}{}", 0, address);
      }
   }
   return address;
}

pub fn uint_to_mem(byte: u8) -> (String, char)
{
   let mut c = byte as char;
   let mut hex = format!("{:X}", byte);
   
   if c == '\n' {c = 'n';}
   if hex.len() == 1 {hex = format!("{}{}", 0, hex);}
   return (hex,c);
}
