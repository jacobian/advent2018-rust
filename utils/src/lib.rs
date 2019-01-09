use std::env;
use std::fs::File;
use std::path::Path;

// shortcut to read the filename specificed by argv[1] into a std::fs::File
pub fn open_argv1() -> File {
    let args: Vec<String> = env::args().collect();
    let path = Path::new(&args[1]);
    return File::open(&path).expect(&format!("can't open {:?}", path));
}

