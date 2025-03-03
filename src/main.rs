fn main() {
    println!("Hello, world!");
    // second_function();
    // params(5, 'h');
    looping();
    for_loop();
}

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
  let mut i = 0;

  while i < 5 {
    println!("The value is: {}", arr[i]);

    i += 1;
  }
}