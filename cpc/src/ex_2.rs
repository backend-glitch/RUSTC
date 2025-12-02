
fn another2(){

    let number = 3;

    if number < 5 {
        println!("True");
    }else{
        println!("False");
    }

    // if in a let
     let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");
}

// loop {} => for infinte loop

//counter
  let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break ;
        }
    };

    println!("The result is {result}");


    // iterating the array
      let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }

    //short way
      let a1 = [10, 20, 30, 40, 50];

    for element in a1 {
        println!("the value is: {element}");
    }
    // .rev() => to reverse a range