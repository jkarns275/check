# check
Check is a macro used save a few lines of code while checking results. On the case that there is an error,
the error will be returned. If desired, on success, the result will be stored
in a variable.

# Example
Without check:
`rust
#[macro_use] extern crate check;
use std::fs::File;
use std::io::prelude::*;
use std::io::Error;

fn write_to_file(data: &[u8]) -> Result<(), Error> {
   let mut file;
   // try to open the file, if there's an error, return it
   // otherwise, store it in file.
   match File::open("foo.txt") {
      Ok(res)  => file = res,
      Err(e)   => return Err(e)
   };

   // try to write to the file.
   // ignore the result, since it is ()
   match file.write_all(data) {
      Ok(())  => {},
      Err(e)   => return Err(e)
   };
   // we made it to here, both functions were successfull!
   Ok(())
}
`
With check:
`rust
#[macro_use] extern crate check;
use std::fs::File;
use std::io::prelude::*;
use std::io::Error;

fn write_to_file(data: &[u8]) -> Result<(), Error> {
   let mut file;
   // try to open the file, if there's an error, return it
   // otherwise, store it in file.
   check!(File::open("foo.txt"), file);

   // try to write to the file.
   // ignore the result, since it is ()
   check!(file.write_all(data));

   // we made it to here, both functions were successfull!
   Ok(())
}
` 
