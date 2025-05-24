pub mod logs;
mod local;


#[cfg(test)]
mod tests {
    use super::*;
    use logs::LogError;
    #[test]
    fn it_works() {
        if let Some(file)=std::fs::read("./README.md").log_error("open fle error"){
            println!("{:?}",file);
        }
        if let Some(file)=std::fs::read("./README_cn.md").log_error("open file"){
            println!("{:?}",file);
        }
        let file=std::fs::read("./README_cn.md").log_error("open file error");
        println!("{:?}",file);
    }
}