
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

fn main() {
    green_world();
    handle_records();
}