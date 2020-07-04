 //use of method///
/*struct Class{
    teacher:String,
    no_of_students:i32,
    CR:String,
    teacher_attendence:bool
}
impl Class{
fn look(&self)->Class{
    Class{
        teacher:String::from("Miss Mehwish"),
        no_of_students:30,
        CR:String::from("Ali"),
        teacher_attendence:true
    }
  }
}
    fn main(){
        let inst=Class{teacher:String::from("Miss Mehwish"),
        no_of_students:30,
        CR:String::from("Ali"),
        teacher_attendence:true
        };
        inst.look();
    }
    */

    //////////////////////////////////////////////////////////////////////
 
    /*
     #[derive(Debug)]
    struct Class{
        teacher:String,
        no_of_students:i32,
        CR:String,
        teacher_attendence:bool
    }                                       ////problem 01
    fn check(pase:Class)->(String,String){
        format!("{},{}",pase.teacher,pase.CR)
    }
fn main(){
    let inst=Class{teacher:String::from("Miss Mehwish"),
    no_of_students:30,
    CR:String::from("Ali"),
    teacher_attendence:true
};
check(inst);
    
}
*/
// assignment:02  q_01
/*#[derive(Debug)]
struct Book{
    author:String,
    name:String
}
//impl Book{
    //fn label(){

    //}

trait BookInformation{
    fn info(&self)->String;
}                     //no need of implementation block
impl BookInformation for Book{
    fn info(&self)->String{
        format!("{},{}",self.author,self.name)
    }
}
fn main(){
    let res=Book{
        name:String::from("anonymus"),
        author:String::from("anoy")
    };
    println!("{}",res.info() );
}
*/
///////////////
/*
 #[derive(Debug)]
struct Book{
    author:String,
    name:String
}
impl Book{
    fn info_1( author:String,name:String)->Book{
        Book{
            name,
            author, 
        }
    }
}
trait BookInformation{
    fn info(&self)->String;
}
impl BookInformation for Book{
    fn info(&self)->String{
        format!("{},{}",self.author,self.name)

    }

}
fn main(){
let res=Book{name:String::from("anonymus"),
author:String::from("annoy") 
};


println!("{}",res.info() );
}
*/
/*
use std::fmt::Display;
#[derive(Debug)]
struct  Pair<T>{
    x:T,
    y:T,
}
impl <T> Pair<T>{
    fn new(x:T, y:T)->Pair<T>{
     Pair{
         x,
         y,
     }
    }
}    
impl <T:Display + PartialOrd> Pair<T>{
    fn compare(&self){
        if self.x>self.y{
            println!("X is greater then y");
        }
            else{
                println!("y is greater than x");
            }
        
    }collections::HashMap;
fn main(){
    let x=Pair::new(6,8);
    x.compare();
let mut h1=HashMap::new();
h1.insert("blue",10);
let mut h2=HashMap::new("pink"20);
h2.insert("pink",30);
let h3=Pair::new(h1,h2);
h3.compare();
*/
/*fn main(){
    fn  average(v:&i32)->i32{
let v=[3,7,5,13,20,23,40,23,40,23,14,12,56,23,29];
let count=v.len();
let mut sum=0;
for i in 0..count{
    sum+=v[i];
}
    }
println!("{}",sum/count );
}

//fn new(acc:i32)->i32{
    //let account=v[0];
    //for i in 0..v.len(){
        //account=item 
       // sum += v[i];
*/
/*
use std::ops::Add;
use std::ops::Div;
fn average<T>(list: &[T])->T
where T:Add<Output=T> + Copy + Clone + From<i32>{
    let mut sum =list[0];
    let count= T::from(list.len() as i32);
    for x in 1..list.len(){
        sum=sum+ list[x];
    }
    sum/count
    http://tiny.cc/IOTQ2S1
}
fn main(){
    let number_list=vec![34,50,25,100,65];
    let result =average(&number_list);
    println!("The sum of total number is {}",result);
    let number_list_02=vec![2.0,4,0,6.3,8.3,10.4];
    let result=average(&number_list_02);
    println!("The sum of total number is {}",result);
}
fn main(){
    let return=|m,n|{
        n+m
    };
    println!("{}",return(5,6));
}*/

//The closures capture the values from the scope i which they are defined.
/*use std::thread;
use std::time::Duration;
fn main(){
    fn workout(intensity:u32)->u32{
        println!("do work out for 2 seconds");
        thread::sleep(Duration::from_secs(2));
         intensity
    }
}
*/
/*fn main(){
    let var=|x|{
        println!("i'm here {}",x);
    };
    var(50);
}
*/
/*use std::thread;
use std::time::Duration;
fn main(){
    workout(lim,pit);
    let lim=20;
    let pit=30;

}
fn calculation(intensity:u32)->u32{
    println!("it will take 2 minutes");
    thread::sleep(Duration::from_secs(3));
    intensity
}
fn workout(intensity:u32,randomnum:u32)->u32{
    if intensity <25 {
        println!("do {} push ups",calculation(intensity));
    }
    else{
        println!(" {}go and take rest!",calculation(intensity));
    }
        }
    */

    //QUESTION #01
    /*fn main(){
        let Q1=|p,q|{
            println!("The sum of p and q is {}",(p+q));
        };
        Q1(2,3);
    }
    */ 
    //question #02
/*use primes::PrimeSet;
fn main(){

let mut pset = PrimeSet::new();

for (ix, n) in pset.iter().enumerate().take(10) {
    println!("Prime {}: {}", ix, n);
}
}
*/
/*important
fn main(){
let prime=|x|{
    for i in 2..x{
        if x%i==0{
            return false;
        }
    }    
            true  
};
println!("{}",prime(9));
}
*/

/*fn main() {
    use std::thread;
    use std::time::Duration;
    
    fn generate_workout(intensity: u32, random_number: u32) {
        let expensive_closure = |num| {
            println!("calculating slowly...");
            thread::sleep(Duration::from_secs(2));
            num
        };
    
        if intensity < 25 {
            println!(
                "Today, do {} pushups!",
                expensive_closure(intensity)
            );
            println!(
                "Next, do {} situps!",
                expensive_closure(intensity)
            );
        } 
        else {
            if random_number == 3 {
                println!("Take a break today! Remember to stay hydrated!");
            } else {
                println!(
                    "Today, run for {} minutes!",
                    expensive_closure(intensity)
            
                );
            }
        }
    }
    generate_workout
} 
*/

/*use std::thread;
use std::time::Duration;
fn main(){
    let var=|num:u8|->u8{
        println!("calculating....");
        thread::sleep(Duration::from_secs(9));
        num

    };
    var(8);
}
  closure is a function that can be stored in a variable.
  closure  environment sy value get krskta hy  function nai
  closure statement hy , closure k andr closure ko call krengy tou expression bn jata hy
  statement: terminator
  expression: no termiator
  synthetic sugar (i+=1)
  a program in a runing state is called process
  PID process ID
  CACHE IS ON OUR MOTHER BOARD cache is nearer as compare to memory

  subset of a process is called thread they are of two types 
  android OS is a 
  SOB single board comp    
  comp multi processing hota hy  
  hr closure fn trait ko accept krta hy
  java mai first time threads introduce hey thy

  */

  /*fn main(){
      println!("{}",add_two_num(&mut 2,&mut 3));
  }
  fn add_two_num<'a,'b>(x:&'a mut  i32,y:&'b i32)->&'a i32{
     *x= *x+y;
     x
  }
  */
  /*fn main(){
  
  let pi=3.14;
  let areacircle=|radius: f64|->f64{
      pi*(radius*radius)
  };
  fn area(radius:f64)->f64{
      3.14*(radius*radius)
  } 
  println!("{}",areacircle(3.0));

  println!("{}",area(3.0) );
}
*/

/*use std::thread;
use std::time::Duration;
struct Cacher<T>
    where T:Fn(u32)->u32{
    calculation: T,
    value:Option<u32>

}
impl Cacher<T>Cacher<T>
where T:Fn(u32)->u32{
    fn new(calculation:T)->Self{
        Self{
            calculation,
            value:None,
        }
    }
    fn the_value (&mut self,x:u32)->u32{
        match self.value{
            Some(v)=>v,
            None=>{
                let v=(self.calculation) (x);
                self.value=Some(v);
                v

            }
        }
    }
}

fn generate_workout(intensity:u32,random_number:u32){
    let mut expensive_cal=Cacher::new(|num:u32|{ 
        println!("wait....");
        thread::sleep(Duration::from_secs(3));
         num
    });
    println!("{:?}",expensive_cal.value);
*/
   /* if intensity<25{
        println!("do {} pushups",expensive_cal.(intensity));
        println!("do {} situps",expensive_cal(intensity)); 
    }
    else{
        if random_number==5{
            println!("take rest");
        }
        else{
        println!("run {} km",expensive_cal(intensity));}
    }

}
   fn main(){ 
    generate_workout(20,5);

}
*/
//hhtp://www.tiny.cc/FBFQ2

///////////////to ask/////////
/*fn main() {
    
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}
}
*
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}
*/

     /////to ask/////
/*fn main(){
    let u="i lovecup cakes";
    let v="i love chocolates";
    let r=longest(u,v);
    println!("{}",r);
}
fn longest<'a>(x:&'a str,y:&'a str)->&'a str{
    if x.len()>y.len(){
        x
    }
    else{
        y
    }
    
}
*/
/*#[derive(Debug)]
struct Kind<'a>{
    part:&'a str,
}
fn main(){
    let v=String::from("I am a human.Let me do what I want to do");
    let p=v.split(".").next().expect("an error occured!");
    let i=Kind{
        part:p,
    };
    println!("{:#?}",i );
}
*/ 

/*
use std::time::Duration;
use std::thread;
struct Cache<T>
where T:Fn(u32)->u32{
    calculation:T,
    value:Option<u32>,
}
impl <T>Cache<T> 
    where T:Fn(u32)->u32{
        fn new(calculation:T)->Cache<T>{
            Cache{
                calculation,
                value:None
            }
        }
        fn value(&mut self,arg:u32)->u32{
         match self.value{
             Some(v)=>v,
             None=>{
                 let v=(self.calculation)(arg);
                 self.value=Some(v);
                 v
             },
         }
        }
}
//fn simulated(intensity:u32)->u32{
    //println!("Have some patience!");
    //thread::sleep(Duration::from_secs(3));
    //intensity

fn main(){
fn generate_workout(endurance:u32,random_no:u32){
    let mut closure=Cache::new(|endurance|{
        println!("Have some patience!");
    thread::sleep(Duration::from_secs(3));
    endurance
    });
    if endurance > 10{
        println!("do {} squats",closure.value(endurance));
        println!("Start doing!!");
        //thread::sleep(Duration::from_secs(2));
        println!("Okay now\nStart {} crunches!",closure.value(endurance));
        //thread::sleep(Duration::from_secs(2));
        println!("Now just inhale and exhale......!!");
        
    }
    else{
        if random_no==30{
        println!("just do {} min plank",closure.value(endurance));
        }
        else{
            println!("Take rest...!!");
        }
    }
}

  generate_workout(50,30);
  
}
//Once the code is executed it sotes the value in its Some variant when the value 
//is called for the next time there'll be no need to execute closure again.

//We can not use this implementation of Cache in  a different context. 

//All closures must follow atleast one of the Fn or FnOnce or FnMut trait.

*/

/*fn main(){
    let vect=[12,13,14,15];
    let closure=move|x| x==vect;
        println!("{:?}",vect);
        
    closure(vect);
    let p=vect;
    println!("{:?}",p);
    */
    /*let test=thread::spawn(move||{
        for i  in 1..10{
            println!("It is spawned thread {}",i);
            thread::sleep(Duration::from_secs(1));

        }
    });
    //handle.join().unwrap();
    for i  in 11..20{
        println!("It is main thread {}",i);
        thread::sleep(Duration::from_secs(1));

    }*/

/////////////////
/*fn main() {
    let x = std::f64::consts::PI;
    let r = 8.0;
    println!("the area of the circle is {}", x * r * r);
}

fn main(){
let v=|x:i32,y:u8|->i32{
   x+y
};
let p=v(3,4);
println!("{}",p);
}
struct Example<'a>{
    x:& 'a i32,
}
impl<'a> Example <'a>{
    fn display(&self){
        println!("{}",a );
    }
}
fn main(){
    let y=&90;
    let b=Example{
        x:y
    };
    b.display(  )
}
*/  
fn main(){
    let a:i8=5;
    let b:i8=8;
    // & operation
    let AND=a*b;
    println!("{}",AND );
    // | operation
    let OR=a+b;
    println!("{}",OR );
    //  ^ operation
    let q=a^b;
    println!("{}",q);

    // left shift 5 by two places.
    //binary of 5 is : 0101
                       //10100  

}
