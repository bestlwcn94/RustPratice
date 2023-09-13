fn main() {
    println!("Hello, world!");

    println!("{}",take(add,5,2));

    let mut counter = Counter::new();
  
    do_something(|x| counter.add(x));
  
    // println!("{}", counter.count);
}

fn add(a:i32,b:i32)->i32{
    a+b
}
fn take(f:fn(i32,i32)->i32,a:i32,b:i32)->i32{
    f(a,b)-a
}

struct Counter {
    count: i32
  }
  
  impl Counter {
    fn new() -> Counter {
       Counter { count: 0 } 
    }
  
    fn add(&mut self, x: i32) {
      self.count += x;
      println!("{}",self.count);
    }
  }
  
  fn do_something<F>(mut f: F) 
  where
    F: FnMut(i32)  
  {
    f(10);
    f(20);
  }
  