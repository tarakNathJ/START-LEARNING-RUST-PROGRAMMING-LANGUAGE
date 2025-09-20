

fn main() {

    // chack mut and unmut variable and different( i and u)
    let mut _day_2_first_variable: i8 = 20;

    // day_2_first_variable = 160; i8 renge -128..=127
    _day_2_first_variable = 110;

    let _day_2_first_variable = _day_2_first_variable ;

    println!("this is your first number is  :-  {_day_2_first_variable}");


    // first  time send value on function 
    let _send_value = _first_function(12) ;
    println!("function send   data   {}" ,_send_value);

    // first time call void function 
     _first_void_function("hello mather cho".to_string());
}


fn _first_void_function(values : String){

   println!( "your data is  ---  {}" ,values);

}


fn _first_function (number :i32) ->i32{
    let value :i32 = 10 ;
    println!("under the function {}",value);
    let   value2 = value + number;
    value2

}