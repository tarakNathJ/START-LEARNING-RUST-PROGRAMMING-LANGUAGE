

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





    //  chack ownership
    let _owner_ship :String = "this is your data".to_string();
    let _return_owner_data =  _chack_owner_ship(_owner_ship);
    println!("{}",_return_owner_data);




    // chack referencing 

    let mut referencing_data = 124; 

    let return_data = _reference(& mut referencing_data);
    
    println!(" your data is ; {} this is your return data : {}",referencing_data ,return_data);



}



//  this is void function 
fn _first_void_function(values : String){

   println!( "your data is  ---  {}" ,values);

}

//  this is non void function
fn _first_function (number :i32) ->i32{
    let value :i32 = 10 ;
    println!("under the function {}",value);
    let   value2 = value + number;
    value2

}



// chack ownerShip

fn _chack_owner_ship (data :String)-> String{
    let data = data;
    let data2 = data;

    
    println!(" owner function data : {}",data2 ); // thay give me problem for owner ship
    data2
}


// referencing  

fn _reference (    data  :&mut i32)->i32{
    *data =*data - *data + 23;
    let return_data = *data +20;


    let chack_test= 12232;
    let test_2 = &chack_test;
    println!("your test data is : {} second test {}",&chack_test ,*test_2);


    return_data
    
}


