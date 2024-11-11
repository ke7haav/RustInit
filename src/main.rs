// use std::string;

// use std::clone;

trait Pintable{
    fn print(&self){

    }
}
struct Rectangle{
    width:u32,
    height:u32
}

impl Pintable for Rectangle{
    fn print(&self){
      println!("Rectangle: {} x {}",self.width ,self.height)
    }
}

trait HasArea {
    fn area(&self) ->f64;
}

struct Square {
    side:f64
}

impl HasArea for Square {
    fn area(&self) ->f64 {
        self.side *self.side
    }
}

fn print_area <T:HasArea>(shape:T){
    println!("Area of Shape is {}", shape.area());
}
fn main() {

    let rect1 = Rectangle{width:43,height:7};
    rect1.print();

    let squre1 = Square{
        side:9.30
    };

    print_area(squre1);

    const USERS:u32=100;
    const USER_AVG:f64=42.4;

    println!("Number Of Users are {}",USERS);
    println!("Average of Users are {}",USER_AVG);
    let a= 45; // Variables are by-default immutable 
    let mut _b=66;
    println!("{a}");
    println!("{}",a);
    println!("{}",_b);

    _b= 90;
    println!("{}",_b);

    let c ="String";
    println!("{}",c);

    first_fn();
    function_with_params(8);
    multiple_params(99,"hochimin");
    ex();
    let mut _return_val = return_value();
    println!("returned Value is -{}",_return_val);

    data_type();
    conditions();
    loops();
    ownership();
    reference();
    structs();

    printcars(CarTypes::SUV);
    // println!("{:?}",_x);
}

fn first_fn(){
    println!("new Function")
}

fn function_with_params(x:i32){
  println!("The Value in the param {}",x);
}

fn multiple_params(x: i32, y: &str){

    println!("first Param {}",x);
    println!("Second Param {}",y);
}

//expression
fn ex(){
    let y ={
        let x= 10;
        x+6
    };
  println!("value inside expression- {}",y)
}

fn return_value() -> i32{
    98+57
}

fn data_type() {
    //scalar Types - types where we can store single value
    //Integer, boolean ,Floating,Char, Strings

    //Compound Types - types where we store multiple data at a time 
    // Array,tuples, dictionary


    //tuple
    let tup =(8,5,6);
    println!("{:?}",tup);  // :? is default formatter 
    println!("{}",tup.2);

    let mut _tup2:(i32,u8,f64) = (4,6,7.3);
    // Here we can change the value in tuple as it is mutable but we can't add the value in the tuple

    println!("{:?}",_tup2);
    _tup2.2 = 8.9;
    println!("{:?}",_tup2);

    // Array - is a collection of multiple values
    let mut _arr =[7,3,40,0,1];
    println!("This is the whole Array - {:?}",_arr);
    println!("Fourth Element of Array {}",_arr[3]);

}

fn conditions(){
    let no = 7;
    if no<7{
        println!("Less Than 7")
    }else if no==7{
        println!("Equal to 7")
    }else {
        println!("Greater Than 7")
    }

    // ShortEnd If Condition
    let _condition = false;
    let number = if _condition {5} else{0};
    println!("Let's see the number - {}", number)
}

fn loops(){
//Types of Loops 
//loops , for, while
let mut _x =10;
loop {
    println!("Print Till 10- {}",_x);

    if _x == 11 || _x==10{
      break;
    }
}

while _x!=0{
    println!(" Number-{}",_x);
    _x-=1;
}

let mut _arr=[2,3,5,7,1];
let mut _index = 0;

while _index<3 {
    println!("Value of Index is {}",_arr[_index]);
    _index+=1;
}


for _y in 1..11{  // y=0;y<11;y++ print y expect 5 
    if _y==5{
        continue;
    }
    println!("y is ={}",_y);
}
    
}

fn ownership(){
    let x=90;
    println!("value of x - {}",x);
    let y=x;
    println!("value of Y - {}",y);
  
    //------------------------------

    let a =String::from("Koder");
    // let b:&str="KODER";
    let b= a.clone();// clone funciton only works with string and Arrays

    println!("{}-----{}",a,b);
}

fn reference(){
    let mut _x=10;
    let z = &mut _x;
    *z +=1;
    println!("value of z is - {}",z);
    println!("value of x is - {}",_x);

}

struct User{
    name :String,
    company: String,
    age:u8
}

fn structs(){

    let mut _u1 = User{
        name:String::from("Asish"),
        company:String::from("WazirX"),
        age:26
    };

    println!("Name {} Company {} Age {}", _u1.name,_u1.company, _u1.age);
    _u1.age=27;
    println!("Age Updated {}", _u1.age);

}

enum CarTypes {
    Hatchback,
    Sedan,
    SUV,
    MUV
}

fn printcars(car:CarTypes){
match car {
    CarTypes::Hatchback=>{
        println!("Small car Segment")
    }
    CarTypes::MUV=>{
        println!("MultiPurpose Car")
    }
    CarTypes::SUV=>{
        println!("Sports Utility ")
    }
    CarTypes::Sedan=>{
        println!("Luxury Cars")
    }
    
}
}
