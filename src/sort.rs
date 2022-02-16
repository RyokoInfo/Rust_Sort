
pub fn heap_sort(array: &mut[i32]){
    let array_len = array.len() - 1;
    let mut i;
  
    i = array_len/2;
    while i>=1 {
      maketree(array,i ,array_len);
      i -= 1;
    }
  
  
    i = array_len;
    while i>1{    
  
      array.swap(1,i); 
      maketree(array,1,i - 1); 
        
      i -= 1;
    }
  
  }
  
  fn maketree(array: &mut[i32],first:usize,last:usize){
    let mut parent = first;
    let mut child = 2*parent;
    while child<=last{
      if (child<last) && (array[child]<array[child+1]){
        child += 1;
      }
      if array[child]<=array[parent] {
        break;
      }
      array.swap(child,parent);
      parent = child;
      child = 2*parent;
    }
  }