
/* 
We use "use" Keywords for importing any module and public functions from 
different file 
*/
mod lib;
use std::env;
use std::collections::HashMap;
use std::thread;
use std::time::Duration;

mod my_module{
    pub fn persoal(){
        println!("Printing inside the Module ");
    }
}

//Nested module 

mod my_mod2{
    pub mod movie{
        pub mod holywood{
            pub fn play(name:&str){
             println!("Movie name is - {}",name);
            }
        }
        pub mod bollywood{
            pub fn play_hindi(name:&str){
             println!("Movie name is - {}",name);
            }
        }
    }
}


use my_mod2::movie::holywood::play;
use my_mod2::movie::bollywood::play_hindi;

fn main() {
    
    my_module::persoal();
    lib::print();
    lib::print2();
    lib::print3();



let arg:Vec<String>= env::args().collect();
// println!("{}",arg[1]);

for argunments in arg.iter(){
    println!("{}",argunments);
}

    // Nested 
    play("Solc");
    play_hindi("Hash-Map");

    // Declearing a hashmap using new marks variable
    let mut marks: HashMap<&str ,i32> = HashMap::new();

    // Inserting into Hashmap 
    marks.insert("Rust",40);
    marks.insert("Go",60);
    marks.insert("Solidity",85);
    marks.insert("JavaScript",50);
    marks.insert("TypeScript",50);
    
    println!("{:?}",marks);
    //find the length
    println!("Total Number of Subjects - {}",marks.len());

    //matching the value
    match marks.get("Rust"){
        Some(mark) => println!("You got {} marks",mark),
        None => println!("Subject not matched")
    }

    //Removing a value
    marks.remove("TypeScript");
    println!("{:?}",marks);


    //Loops through HashMap 

    for(subject,mark) in &marks{ // key- subject, value - mark
        println!("For {} you got {} marks ",subject,mark )
    }

    //Check the values
    println!("Do we have C++ as subject {}",&marks.contains_key("C++"));


    // Creating a new thread to run parallely with main function 
    thread::spawn(||{
        for i in 1..100{
            println!("The numbers are from the spawned thread {}",i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // code for main Function 
    for i in 1..4{
        println!("The Numbers are from the Main Thread {}",i);
        thread::sleep(Duration::from_millis(200));
    }
}
