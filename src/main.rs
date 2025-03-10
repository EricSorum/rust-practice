
#[derive(Debug)]
struct Rectangle {
  width: u32,
  height: u32,
}

fn main() {

  let rect1 = Rectangle {
    width: 30,
    height: 50
  };

  // use debug for rect1
  println!("Debug the struct: {:?}", rect1);

  fn rectangle_function(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
  }

  println!("The area of the rectangle is {} square pixels.", rectangle_function(&rect1));





  let boo = false;
  if boo {
    println!("Hello, world!");
    // second_function();
    // params(5, 'h');
    looping();
    for_loop();
  }
}


// Rectangle program


// fn second_function() {
//   let x = "Some string";
//   println!("{}", x);
// }

// fn params(value: u32, unit_label: char) {
//   println!("The measurement is: {value}{unit_label}");
// }

fn looping() {
    let mut num = 3;
    while num != 0 {
      println!("{num}!");
      num -= 1;
    }

    println!("LIFTOFF!");
}

fn for_loop() {
  let arr = [10, 20, 30, 40, 50];
  // let mut i = 0;

  // while i < 5 {
  //   println!("The value is: {}", arr[i]);

  //   i += 1;
  // }
  for element in arr {
    println!("The value is: {}", element);
 }
}