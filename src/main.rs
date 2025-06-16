use serde::{Deserialize, Serialize};

#[derive(Serialize,Deserialize,Debug,Clone)]
struct User{
    username:String,
    password:String
}

fn main() {

    //Serialization Converting struct to string
    // let s = User{
    //     username:String::from("Siddharth"),
    //     password:String::from("Siddharth"),
    // };
    // let serialized_string = serde_json::to_string(&s);
    // match serialized_string {
    //     Ok(s)=>print!("{}",s),
    //     Err(_e)=>print!("Errorrrr")
    // }
    // print!("{:?}",s);

    //Deserialization converting string to struct 
    let s = String::from("{\"username\":\"Siddharth\",\"password\":\"Siddharth\"}");
    let u:Result<User, serde_json::Error> = serde_json::from_str(&s);
    match u {
        Ok(u) =>println!("{:?}",u),
        Err(_e)=>println!("There was error")
    }


}
