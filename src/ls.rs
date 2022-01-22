use std::path::Path;
use std::fs;


pub fn work(){
    working(Path::new("."))
}

fn working(path: &Path){
    let mut total_file  = 0;
    let mut total_size = 0;
    let mut is_dir:&str;
    if path.is_dir(){
        for file in fs::read_dir(path).unwrap(){
            total_file+=1;
            let file_path = file.unwrap().path();
            let file_size = fs::metadata(&file_path).unwrap().len();
            if fs::metadata(&file_path).unwrap().is_dir(){is_dir = "<DIR>";}else{is_dir = "<FILE>"}
            let last_modified = fs::metadata(&file_path).unwrap().modified().unwrap().elapsed().unwrap().as_secs();
            let seconds = last_modified % 60;
            let minutes = seconds % 60;
            let hours = minutes / 60;
            total_size += file_size;
            println!("File:{file_name} {dir} Size:{file_size}bytes LastModified:{hr}h:{min}m:{sec}s",
            file_name=file_path.display(), file_size=file_size, hr=hours, min=minutes, sec=seconds, dir=is_dir);
            println!("")
        };
        println!("{} File(s) {}Bytes", total_file, total_size);
    }
}