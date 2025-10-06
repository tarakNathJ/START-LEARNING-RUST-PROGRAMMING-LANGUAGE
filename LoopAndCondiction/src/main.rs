


fn main() {
    

    // try for loop
    for i in 1..5{
        println!("hello sir {}",i);
    }

    let  mut count :u8 = 0;


    // and try while loop 
    while count < 10 {
        println!("what  are you {}" , count);
        count += 1 ;
    }

    let mut  number  = [10,20 ,30 ,40 ,50 ,60 ,70];
    number[1]=100;

    println!("print full array : {:?} " ,number );

    println!("your  array is :- {}",number[1]);

}
