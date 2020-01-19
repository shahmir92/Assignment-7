pub mod shell{
    pub mod innershell{
        pub fn printer(){
            println!("Hello I'm in inner shell");
        }
    }
}


fn main() {
    shell::innershell::printer();
}
