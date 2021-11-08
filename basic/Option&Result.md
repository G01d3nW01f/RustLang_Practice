# OPTION and RESULT

## option and result is important type in RustLang. Both these are so similar, 
## but it is not a same.
## so we have to switch the use depending on the case


## Option<T> types are enum type of Values that may not be retrieved
    None is not value
    Some(T) is got the something to value 
    None is not a error

```rust
  pub enum Opton<T>{
    None,
    Some(T),
  }
 ```
    
    if you want to error appear,
    then you should use the Result<T,E> types.

    
    
    
# Sample
'''rust
        fn get_value_failed(value: bool, result: &mut usize) -> bool{
        if value {
            *result = 1000;
            true
            }else{
                false
            }
        }

        fn get_value_success(value: bool) -> Option<usize>{
        if value {
        Some(1000)
            }else{
                None
            }
        }


        fn main(){
    
        match get_value_success(true){
        Some(result) => { 
        println!("Success: {}",result);
        
        },
        None => println!("Failed"),
            
            }
        }
```
    
