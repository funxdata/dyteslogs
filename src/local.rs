use chrono::Datelike;
use chrono::Local;
use std::env;
use std::fs;
use std::io::Write;

// create local error cache
fn create_local_log_cache() {
   let mut path_src = env::current_dir().unwrap();
   path_src.push(".cache");
    let res = fs::metadata(path_src.clone()).is_ok();
    // checl date log
    println!("{:?}", res);
    if !res {
        fs::create_dir(path_src).unwrap();
    }
}
// create log cache fil
fn create_local_log_fil() -> String {
    let current_date = chrono::Utc::now();
    let year = current_date.year();  //this is not working, it should output the current year with i32/usize type
    let month = current_date.month();
    let day = current_date.day();
    let  mut current_file = env::current_dir().unwrap();
    current_file.push(".cache/");
    let fil_name = format!("info_{:?}{:?}{:?}.log",year,month,day);
    current_file.push(fil_name);
    let res = fs::metadata(current_file.clone()).is_ok();
    if !res {
        fs::File::create(current_file.clone()).unwrap();
    }
    return current_file.to_str().unwrap().to_string();
}

// write log info
pub fn write_log_info(err_info:String) {
    create_local_log_cache();
    let fil = create_local_log_fil();
    let mut file = fs::OpenOptions::new()
                            .append(true)
                            .open(fil).unwrap();  
    let dt = Local::now();
    let info = format!("{},time:{}\n",err_info,dt.timestamp_millis());         
    file.write_all(info.as_bytes()).unwrap();
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn do_local_run() {
        println!("........");
        create_local_log_cache();
        create_local_log_fil();
    }
}