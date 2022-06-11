/*
*
* Made by @Lemonetar Copyright ©️ @Lemonetar 2022 ononwards
* Password Manager
*
*/

// all the includes needed
use std::path::Path;
use std::io;
use std::io::Write;
use std::fs::*;

//entry point calles function
fn main() {

    get_password();

}

fn get_password() {

    //user input
    let action = input("What do you want to do? ")
        .expect("Error Handling input")
        .parse::<String>();

    let sp = "sp".parse::<String>();
    let show_password = "shp".parse::<String>();

    if action == sp {

        println!("Save Password");
        makefile().expect("Error");

    } else if action == show_password {
        println!("Show a password? ");
    }
}
fn makefile() -> std::io::Result<()>{

        //gets the password!
        let pw = input("Whats the password? ")
            .expect("Error ")
            .parse::<String>();

        //file name
        let pfilename = input("The Name? ")?;
        //Why The fuck did i do that lmao
        let pfilenamee:&Path = pfilename.as_ref();

        //creates the file
       let mut pfile = File::create(pfilenamee)?;
        write!(pfile, "{:?}", pw)?;
        Ok(())

    }

fn input(user_message: &str) -> io::Result<String> {
    print!("{}", user_message);
    io::stdout().flush()?;

    let mut buffer: String = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(buffer.trim_end().to_owned())
}
