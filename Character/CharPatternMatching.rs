fn main()
{
    let Grade:char = 'B';

    let Message = match Grade {
        'A' => "Excellent!",
        'B' => "Good Job",
        'C' => "Average",
        'D' => "Below Average",
        'F' => "Failed",
            _=> "Invalid Result",
    };

    println!("Grade {}means grade{} " , Grade,Message);

    CheckCharType('5');
    CheckCharType('x');
    CheckCharType(' ');
    CheckCharType('$');
}

    fn CheckCharType(Character:char){
        match Character{
            '0'..='9' => println!("{} is a digit ", Character),
            'a'..='z' | 'A' ..='Z' => println!("{} is a letter ", Character),
            ' ' | '\t'| '\n' => println!("{:?} is whitespace" , Character ),
            _=> println!("{} is a special character", Character),


        }
    }
