
fn main() {
    enum Direction {
        Left,
        Right,
        Top,
        Buttom
        }
    
    
    // let mut x: i64 =45;//by default is  i32
    // println!("the value of x is {}", x);

    // x=90;
    // println!("the value of x is {}", x);

    // let  n = 20;
    // if  n == 45 {
    //     println!("Rust!")

    // }else if  n > 50 {
    //     println!("it is greater than 50!")
    // } 
    // else{
    //     println!("it was not 45! and it was less than 50")

    // }

    // let mut y = 0;
    // loop{
    //     y+=1;

    //     if y == 7{
    //         continue;
    //     }
    //     if y > 10{
    //         break;
    //     }
    //     println!("The value of y is {}", y);
    // }

    //while loop multiple of 5
   let mut x=0;
   while x <= 50 {
    if x % 5==0 {
        
        println!("The number of x is {}", x);
       
    }
    x+=1;
       
   }




    //for loop 
    // let i = 0;
    // let  mut numbers = 10..40;
    // for i in numbers{
    //     println!("the num is {}", i);
    
    // }
    //for inn vectors

    let animals =vec!["cat", "lion", "zebra"];
    for (index, a) in animals.iter().enumerate(){
        println!("the index is {} and the name of the animal is {}", index, a);
    }
//enum
    let play_direction:Direction=Direction::Top;

    match play_direction {
        Direction::Top=> println!("this is my top!"),
        Direction::Buttom=> println!("this is my buttom!"),
        Direction::Right=> println!("this is my right!"),
        Direction::Left=> println!("this is my left!"),

    } 
    //constants

    // const  MAXNUM:u8 = 20;
   
    // for n in 1..MAXNUM{
    //     println!("{}", n)
    // }

    let tup1=(20,30,40,50);
    println!("{}", tup1.2);
        let tup2 = (29,"salman", 9.8, (389,678,90));
        println!("{}", (tup2.3).1);

        let tup3 = (23,45,"salman");
        let (a,b,c)=tup3;
        println!("a is  {}", a);
        println!("b is  {}", b);
        println!("c is  {}", c);
    
}
