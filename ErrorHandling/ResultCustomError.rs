use std::sync::mpsc::Receiver;

#[derive(Debug)]

enum MyError {
    DivisionByZero,
    NegativeInput,
}

fn compute(val: i32 ) -> Result<i32 ,  MyError> {
    if val < 0 {
        return  Err(MyError::NegativeInput);
    }

    if val ==0 {
        return Err(MyError::DivisionByZero);
    }

    Ok(100/ val)
}

fn CustomError() {
    match Compute(0) {

        Ok(v) => println!("Value {}" , v),
        Err(e) => println!("Error {:?}",e),
        
    }
}
