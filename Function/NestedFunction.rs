fn OuterFunction (x: i32 ) ->i32 {

    fn InnerFunction (y:i32) ->i32{
        y*y
    }

    InnerFunction(x) +10

}

fn main() {
    println!("Result : {}", OuterFunction(4));
}
