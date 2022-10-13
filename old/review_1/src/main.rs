use std::fs;

#[allow(dead_code)]
#[derive(Debug)]
enum FileType {
    MUSIC,
    VIDEO,
    DOCUMENT,
    TEXT,
    SPREADSHEET
}

#[allow(dead_code)]
#[derive(Debug)]
struct File {
    path: String,
    name: String,
    extension: String,
    filetype: FileType
}

impl File {
    fn new(path:String, name: String, extension: String, filetype: FileType) -> File {
        return File{
            path,
            name,
            extension,
            filetype
        };
    }
    fn give_love(count: usize) {
        println!("Here is love for you: {:♥<1$}", "", count);
        println!("Here is love for you: {:♥<1$}", "", count);
    }
}

fn main() {
    let file = File::new(
        String::from("/windows/c"),
        String::from("lol"),
        String::from("lol"),
        FileType::TEXT
    );
    File::give_love(4);

    // // println!("{:?}", file);
    // for file in fs::read_dir("../").unwrap() {
    //     let file = file.unwrap();
    //     println!("{:?}", file.path().is_dir());
    // }
}
