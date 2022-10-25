use std::fs::OpenOptions;
use std::io::Write;
use std::io;
use std::io::Read;
use std::fs;


fn adding_data()//adding data into database function
{
    let mut file = OpenOptions::new().append(true).open("data.txt").expect(
        "cannot open file");          
    let mut add_data_string = String::new();
    
    loop
        {
        println!("Please input flight date in format dd.mm.yy:");
        let mut s1 = String::new();
        io::stdin()
        .read_line(&mut s1)
        .expect("Failed to read line");

    if s1.chars().count() - 2 != 8 {println!("Incorrect input!\n");}
    else    { 
        add_data_string += &s1[0..(s1.chars().count() - 2)];
        add_data_string += " ";

        loop
                {
        println!("Please input flight time in format hh:mm :");
        let mut s2 = String::new();
        io::stdin()
        .read_line(&mut s2)
        .expect("Failed to read line");

        if s2.chars().count() - 2 != 5 {println!("Incorrect input!\n");}

        else        { 
        add_data_string += &s2[0..(s2.chars().count() - 2)];
        add_data_string += " ";
        println!("Please input way in format City_of_departure-City_of_destination :");
        let mut s3 = String::new();
        io::stdin()
        .read_line(&mut s3)
        .expect("Failed to read line");
        add_data_string += &s3[0..(s3.chars().count() - 2)];
        add_data_string += "\n";
        break;
                    }
                }
        file.write_all(add_data_string.as_bytes()).expect("write failed");
        println!("\nFlight added successful!");
        break;
        }
    }    
}

fn searching()//search quantity of matches into database
{  
    println!("\nPlease input your search parameter: ");
    let mut guess3 = String::new();
    io::stdin()
    .read_line(&mut guess3)
    .expect("Failed to read line");
    guess3.pop();
    guess3.pop();

    let mut file = std::fs::File::open("data.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let match1 = guess3;
    let c = contents.matches(&match1).count();
    println!("\n I found {:?} matches into database!", c);    
}

fn show()
{
    let mut file = std::fs::File::open("data.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("\n{} \nDatabase was shown above!", contents);
}

fn clear()//clean whole database
{
    println!("\nAre you sure? \n 1)Yes \n 2)No");
            let mut guess2 = String::new();
      io::stdin()
            .read_line(&mut guess2)
            .expect("Failed to read line");
            let guess2: u32 = match guess2.trim().parse() 
            {
                Ok(num) => num,
                Err(_) => return,
            };

            if guess2 == 1
            {
                fs::remove_file("data.txt").expect("could not remove file");
                std::fs::File::create("data.txt").expect("create failed");
                println!("\n Database was cleaned! ");
            }
            else{return;}

}

fn main() {
    println!("\nHi! I'm Airport-Database program.\n");
    loop 
    {
      println!("\nWhat do you want to do?
       1)Add flight to database
       2)Search flight
       3)Show whole database
       4)Clean database
       5)Exit");

      let mut guess = String::new();
      io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
            let guess: u32 = match guess.trim().parse() 
            {
                Ok(num) => num,
                Err(_) => continue,
            };
        if guess ==  1 
        {adding_data();}

        else if guess == 2 
        {searching();}

        else if guess == 3 
        {show();}

        else if guess == 4 
        {clear();}

        else if guess == 5 
        {break;}

        else {println!("\nWrong number!");}
    }
    
}