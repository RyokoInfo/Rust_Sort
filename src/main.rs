mod sort;

fn main() {
    //array setting
    let mut array = [0,2,3,8,7,99,62,0,5,5,32]; 
   
    
    sort::heap_sort(&mut array[..]);

    //display
    for elem in &array[..] {
        println!("answer = {}",*elem );
    }
   
}
