/* 
    LifeTimes in Rust

    - A tool to ensure Memory Safety 
    - Prevent From Dangling Reference 
    - A type of generic
*/

// fn main()
// {
//     let r ;

//     {
//         let x = 5;
//         r = &x;
//     }
//     println!("r: {r}");
// }

//Generic LifeTime Function
fn main(){
    let str1 = String::from("Siddharth");
    let str2 = String::from("Sid");

   let result = longest(&str1, &str2); 
    println!("{}",result);
}
 
fn longest<'a>(s1:&'a String , s2: &'a String)-> &'a String{

    if s1.len()>s2.len(){
        return &s1;
    }
    else {
        return &s2;
    }

}