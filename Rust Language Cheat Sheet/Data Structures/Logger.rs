union Logger {}
    impl Logger {
        fn log(message: &str){
            println!("Log : {}",message);
        }
    }

    fn main(){
                Logger::log("This is the message");
    }
