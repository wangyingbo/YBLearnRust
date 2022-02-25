
// 常量的声明
const MAX_OPTIONS: u32 = 1000_000;

fn green_world() {
    let southern_germany = "Grüß Gott!";
    let chinese = "世界，你好！";
    let english = "world,hello!";
    let regions = [southern_germany,chinese,english];
    for region in regions.iter() {
        println!("{}",&region);
    }
    println!("\n");
}

/**
 * https://course.rs/first-try/hello-world.html
*/
fn handle_records(){
    let penguin_data = "\
   common name,length (cm)
   Little penguin,33
   Yellow-eyed penguin,65
   Fiordland penguin,60
   Invalid,data
   ";
   let records = penguin_data.lines();
   
   for (i,record) in records.enumerate()  {
       if i==0 || record.trim().len() == 0 {
           continue;
       }
       let fields: Vec<_> = record
       .split(',')
       .map(|field| field.trim())
       .collect();
       if cfg!(debug_assertions) {//说明紧跟其后的输出打印只在debug模式下生效
           // 输出到标准错误输出
           eprintln!("debug: {:?} -> {:?}",record, fields);
        }
       let name = fields[0];
       if let Ok(length) = fields[1].parse::<f32>() {
           println!("{},{}cm",name,length)
       }
   }
   println!("\n");
}

fn basic_grammer_method() {
    let a = 0;
    let b: i32 = 10;
    let c = 30_i32;
    let mut d = 30_i32;
    println!("可变变量：{}",d);
    d = 40_i32;
    println!("a={},b={},c={},d={}",a,b,c,d);
    println!("a+b+c+d={}",add(add(a,b), add(c,d)));
    
    let (a, mut b):(bool,bool) = (true,false);
    // a = true,不可变；b = false，可变
    println!("a = {:?},b = {:?}",a,b);
    b = true;
    assert_eq!(a,b);

    println!("常量的值：{}",MAX_OPTIONS);
    
    let guess = "42".parse::<i32>().expect("not a number");
    // let guess: i32 = "42".parse().expect("not a number"); //和上面同样效果
    println!("类型推导与标注：{}",guess);
    
    println!("\n");
}

fn add(i:i32,j:i32)-> i32 {
    i+j //可省略return
}


fn greet(name:String) {
    println!("hello,{}",name);
}

fn test_string() {
    
}


fn main() {
    green_world();
    handle_records();
    basic_grammer_method();
    test_string();
}

