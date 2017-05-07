/// A macro to safely extract data out of results.
/// If an error is encountered, Err(e) will be returned,
/// otherwise, the data is returned (if you want it to be!).
/// This macro can be used with any function that returns a
/// result.
/// # Examples
/// ```
/// # #[macro_use] extern crate check;
/// use std::fs::File;
/// use std::io::prelude::*;
/// use std::io::Error;
///
/// fn write_to_file(data: &[u8]) -> Result<(), Error> {
///    // Where the file, on sucess, will be stored
///    let mut file;
///    // Try to open the file. If it fails, the write_to_file will return
///    // with an Err(some_error)
///    check!(File::open("foo.txt"), file);
///
///    // Try to write to the file
///    // Since the result of write_all is (), we can ignore the result.
///    // the function will still return if there is an error!
///    check!(file.write_all(data));
///
///    Ok(())
/// }
/// # fn main() {}
/// ```
#[macro_export]
macro_rules! check {
    ( $e:expr ) => (
    match $e {
        Ok(_) => {},
        Err(e) => return Err(e)
        }
    );
    ( $e:expr, $v:ident) => (
        match $e {
            Ok(r) => $v = r,
            Err(e) => return Err(e)
        }
    )
}


#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::prelude::*;
    use std::io::Error;

    #[test]
    #[should_panic]
    fn it_works() {
        fn open_bad_file() -> Result<File, Error> {
            let mut file;
            check!(File::open("this_file_shouldn't_exist_or_this_test_will_fail!"), file);
            Ok(file)
        }

        open_bad_file().unwrap();
    }
}
