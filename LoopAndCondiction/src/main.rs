fn main() {
    // try for loop
    for i in 1..5 {
        println!("hello sir {}", i);
    }

    let mut count: u8 = 0;

    // and try while loop
    while count < 10 {
        println!("what  are you {}", count);
        count += 1;
    }

    let mut number = [10, 20, 30, 40, 50, 60, 70];
    number[1] = 100;

    println!("print full array : {:?} ", number);

    println!("your  array is :- {}", number[1]);
    try_how_to_add_object_under_the_array();
}

fn try_how_to_add_object_under_the_array() {
    struct Parson {
        name: String,
        age: u32,
    }

    let people: [Parson; 3] = [
        Parson {
            name: String::from("tarak"),
            age: 89,
        },
        Parson {
            name: String::from("jamon"),
            age: 45,
        },
        Parson {
            name: String::from("data"),
            age: 45,
        },
    ];

    for i in 0..3 {
        println!(
            "this is your name {}  and your age is {} ",
            people[i].name, people[i].age
        );
    }

    let datas: Parson = Parson {
        name: String::from("dj"),
        age: 12,
    };
    let  datam: Parson = Parson {
        name: String::from("tarhf"),
        age: 129,
    };

    println!("your first data is {} , and your second data is  {}",datas.name,datam.age);
}
