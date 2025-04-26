fn main() {
    println!("{}", "coke" == "pepsi");
    println!("{}", "coke" == "coke");
    println!("{}", "coke" == "pepsi" && "coke" == "coke");
    println!("{}", "coke" == "pepsi" || "coke" == "coke");
    println!("{}", "coke" == "pepsi" && ("coke" == "coke" || "coke" == "pepsi"));
    println!("{}", "coke" == "pepsi" && ("coke" == "coke" || "coke" == "pepsi") && ("coke" == "pepsi" || "coke" == "pepsi"));
    println!("{}", "coke" == "pepsi" && ("coke" == "coke" || "coke" == "pepsi") && ("coke" == "pepsi" || "coke" == "pepsi") && ("coke" == "pepsi" || "coke" == "pepsi"));
    println!("{}" , "Coke" != "pespi");
    println!("{}", "Coke" != "pepsi");
    println!("{}", "Coke" != "pepsi" && "Coke" != "pepsi");
}
