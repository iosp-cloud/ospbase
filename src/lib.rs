pub mod ent;


// #[derive(Debug)]
// pub enum Value {
//     String(String),
//     Integer(i64),
//     Float(f64),
//     Boolean(bool),
//     Datetime(Datetime),
//     Array(Array),
//     Table(Table),
// }

#[cfg(test)]
mod tests {
    use std::{fs,env};

    #[test]
//    use crate::ent::ospbase::request::Request;

    fn it_works<'req>() {
        file_curdir();

    }
    fn file_curdir() {
        println!("------------>");
        println!("curdir: {}", env::current_dir().unwrap().display());
        println!("curexe: {}", env::current_exe().unwrap().display());
        println!("curexePath: {}", env::current_exe().unwrap().parent().unwrap().display());
        
        println!("cd to ../..\n");
        // env::current_exe().unwrap().as_path().display()
        env::set_current_dir(env::current_exe().unwrap().parent().unwrap()).unwrap();
        println!("ls: {}", env::current_dir().unwrap().display());
    
        // 注意：如果read_dir中指定的路径不存在，程序就会直接panic
        let paths = fs::read_dir("./").unwrap();
        for path in paths {
            let f = path.unwrap().path();
            println!("{} {}", if f.is_file() { "file " } else { "<dir>" }, f.display());
        }
    }
    
}
