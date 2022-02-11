//use std::env;
//mod module_mult;
mod mod_bable;

fn main() {
    let mut array = vec![2,3,8,7,99,62,0,5,5,32];
    //let mut i = 8;
   
    //ここで所有権が移動する
    mod_bable::bable_sort(&mut array);

    //表示
    for elem in &array[..] {
        println!("answer = {}",*elem );
    }
    //println!("answer i = {}",i);

}
