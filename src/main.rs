use std::io;

fn main() {
    
    let mut data_01 = String::new(); // make a mutable string variable
    startscreen();
    println!("Welcome to schmillies economics Simulator! Press enter to countinue!");
    io::stdin().read_line(&mut data_01); //to get input from the RustcEncodable
    clearscreen();
    println!("Program ran succesfully!"); 
}


//Aesthetic Functions, not of much use or intrest to us.
fn startscreen() {
    clearscreen();
    println!("~         ~~          __");
    println!("       _T      .,,.    ~--~ ^^");
    println!(" ^^   // \\                    ~");
    println!("      ][O]    ^^      ,-~ ~");
    println!("   /''-I_I         _II____");
    println!("__/_  /   \\ ______/ ''   /'\\_,__");
    println!("  | II--'''' \\,--:--..,_/,.-{{ }},");
    println!("; '/__\\,.--';|   |[] .-.| O{{ _ }}");
    println!(":' |  | []  -|   ''--:.;[,.'\\,/");
    println!("'  |[]|,.--'' '',   ''-,.    |");
    println!("  ..    ..-''    ;       ''. '");
    println!("Ask art stolen from the Ascii Art archive")
}
//very useful, clears screen and moves cursos back to location of 1;1, great for our purposes
fn clearscreen(){ 
    print!("\x1B[2J\x1B[1;1H");
}
