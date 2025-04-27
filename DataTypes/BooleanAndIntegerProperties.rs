fn main() {
      let IsHandsome:bool = true;
      let IsSilly:bool = false;

      println!("Handsome : {IsHandsome}. Silly: {IsSilly}");

      let age:i32 = 21;
      let is_young:bool = age<35;

      println!("{is_young}");
      println!("{}{}" ,age.is_positive(),age.is_negative());
}
