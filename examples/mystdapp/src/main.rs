use important::*;

fn main() {
    let obj = MyImportantObj::new(0, "abc");
    let obj_hash = obj.hash();
    let obj_json_string = obj.to_json();
    println!("{:?}", obj_hash);
    println!("{:?}", obj_json_string);
}
