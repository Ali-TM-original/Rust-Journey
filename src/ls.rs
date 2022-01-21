use std::path::Path;
use std::fs;


pub fn work(){
    working(Path::new("."))
}

fn working(path: &Path){
    let mut total_file  = 0;
    let mut total_size = 0;
    if path.is_dir(){
        for file in fs::read_dir(path).unwrap(){
            total_file+=1;
            let file_path = file.unwrap().path();
            let file_size = fs::metadata(&file_path).unwrap().len();
            let last_modified = fs::metadata(&file_path).unwrap().modified().unwrap().elapsed().unwrap().as_secs();
            let seconds = last_modified % 60;
            let minutes = seconds % 60;
            let hours = minutes / 60;
            total_size += file_size;
            println!("File:{} Size:{} bytes LastModified:{}h:{}m:{}s", file_path.display(), file_size, hours, minutes, seconds);
        };
        println!("{}File(s) {}Bytes", total_file, total_size);
    }
}