fn fahrenheit_to_celsius(deg_f: f32) -> f32 {
  (deg_f - 32.0) / (5.0 / 9.0) 
}

fn celsius_to_fahrenheit(deg_c: f32) -> f32 {
  (deg_c * (9.0 / 5.0)) + 32.0
}

fn fibonacci(i: i32) -> i32 {
  if i < 2 {
    return 1;
  }
  return fibonacci(i - 1) + fibonacci(i - 2);
}

fn twelve_days_of_christmas() {
  let ordinals = [
    "first",
    "second",
    "third",
    "fourth",
    "fifth",
  ];

  let gifts = [
    "a partridge in a pear tree",
    "two turtledoves",
    "three french hens",
    "four calling birds",
    "five gold rings",
  ];

  for i in 0..5 {
    println!("On the {} day of Christmas, my true love sent me", ordinals[i]);
    for j in 0..=i {
      println!("{}", gifts[j]);
    }
  }
}

fn main() {
  let deg_c = 100.0;
  let deg_f = 32.0;
  let i = 20; 

  println!("{} deg Celsius equals {} deg Fahrenheit", deg_c, celsius_to_fahrenheit(deg_c));
  println!("{} deg Fahrenheit equals {} deg Celsius", deg_f, fahrenheit_to_celsius(deg_f));
  println!("The {}-th fibonacci number is {}", i, fibonacci(i));
  twelve_days_of_christmas();
}
