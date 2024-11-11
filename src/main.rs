
struct Rectangle{
    width:u32,
    height:u32
}

// We can use multiple implementation 
impl Rectangle {
    //function inside implementation is an internal function 
    fn print_area(&self) {
        println!("Area of Rectangle is {}",self.width*self.height);
    }

    fn is_squre(&self) ->bool{
         self.width==self.height
    }
}


fn main() {

    let myrect = Rectangle{width:20,height:40};

    myrect.print_area();
    println!("{}",myrect.is_squre());

    let mut _person = Person{
        name:String::from("Ravi"),
        email:String::from("ravi@gmail.com"),
        age:43,
        gender:Gender::NonBinary
    };

    println!("Struct {:?}",_person);
    let result =call(81);
    println!("{:?}",result);
    touplestruct();
    stackmemory();
    heapmemory();
    vestors();
}


#[derive(Debug)]
enum Gender{
    Binary,
    NonBinary,
    Asexual
}

#[derive(Debug)]
struct Person{
    name:String,
    email: String,
    age:u32,
    gender: Gender
}

//Option Enums - Default Enums provided by rust 

/*
enum Option<T>{
Some(T)
}
*/

fn call(no:i32)->Option<bool>{
    if no%2 ==0 {
        Some(true)
    }else{
        None
    }
}


fn touplestruct(){
    struct User(u8,f64,String);

    // let mut _user1 = User(60,80,"Hisgenberg".to_string());
    let mut _user1 = User(60,80.55,String::from("Hisgenberg"));

    println!("{} -- {} -- {}",_user1.0,_user1.1,_user1.2);
}

//Stack Memory 

fn stackmemory(){
    // Stack Memory are of fixed size 
    let x=6; 
    let y=8;

    let sum = add(x,y);
    println!("Sum of {} and {} is {}",x,y,sum);

    fn add(a:i32,b:i32) ->i32{
      a+b
    }

    // x,y, sum Uses Stack Memory 
}

fn heapmemory(){
    //Heap memory is a dynamically allocated memory 
    let mut v:Vec<i32> =Vec::new();

    v.push(1);
    v.push(7);
    v.push(-4);
    v.push(0);
    println!("{:?}",v);
}

 //Vestors 

 fn vestors(){
    let mut _a = vec![1,2,3,4,5,6];

    _a.push(74);
    println!("{:?}",_a);
    let mut _b = Vec::<u32>::new();// Declearing an empty Vector 
 }
