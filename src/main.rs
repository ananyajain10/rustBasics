fn main() {
    println!("Hello, ananya!");
    let x = add(1,2);
    println!("{:?}",x);
    let number = 8;

    match number{
        1=> println!("its 1"),
        2=> println!("its 2"),
        3=> println!("its 3"),
        _=> println!("its something else!!")
    }
    let mut y=5;
    while y>0{
        println!("{}",y);
        y=y-1;

    }
    println!("Done!");

    let mut c=1;

    loop{
        println!("{}",c);
        if c>=4{
            break;
        }
        c=c+1;
    }

    // enum
    enum Direction{
        Up,
        Down
    }
    let go=Direction::Up;
    match go{
        Direction::Up => println!("up"),
        Direction::Down => println!("down")

    };

   
   let go = color::blue;
   colors(go);
}

fn add(a: i32, b: i32) -> i32{
    return a+b;
}
enum color{
    blue,
    pink,
    red,
}

fn colors(go:color) {
    
    match go{
    color::blue => println!("blue"),
    color::pink => println!("pink"),
    color::red => println!("red")
    }
}
