fn main() {
    
    startscreen();
    println!("Welcome to schmillies economics Simulator! Press any key to countinue!");
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
//very useful, clears screen and moves variable back to location of 1;1, great for our purposes
fn clearscreen(){ 
    print!("\x1B[2J\x1B[1;1H");
}
