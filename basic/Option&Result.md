# Option\<T\> types

## option and result is important type in RustLang. Both these are so similar,
but it is not a same.
so we have to switch the use depending on the case


## Option<T> types are enum type of Values that may not be retrieved
    None is not value
    Some(T) is got the something to value 
    None is not a error

## OverView
    
```rust
  pub enum Opton<T>{
    None,
    Some(T),
  }
 ```
    
if you want to error appear,
then you should use the Result<T,E> types.

    
    
    
# Sample
```rust
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
    
        let mut result = 0;
        if get_value_failed(true, &mut result) {
            println!("success: {}", result);
        } else {
            println!("failure");
        }
    
        match get_value_success(true){
        Some(result) => { 
        println!("Success: {}",result);
        
        },
        None => println!("Failed"),
            
            }
        }
```
    
# Result \<T,E\> types
        Result<T,E> is an enumerated type 
        that represents the result of an operation that may fail.

        Result is given special treatment by the rust compiler,
        and ignoring it will result in a warinig.

        Result is the standard error handling method in trust,
        with no exceptions.
        If there is a possibility of an error, you should use Result for the result.
    
## Overview
    
 ```rust
    
    pub fn Result<T,E>{
        Ok(T),
        Err(E),
    }
 ```

## Sample
    
```rust
    
    fn get_value_failed(value: bool, result: &mut usize) -> usize{
        if value{
            *result = 1000;
            0
        }else{
            1
        }
    }

    fn get_value_success(value: bool) -> Result<usize,&'static str>{
        if value{
            Ok(1000)
        }else{
            Err("error message")
        }
    }

    fn main(){
        let mut result = 0;
        if get_value_failed(true, &mut result) == 0{
            println!("Success: {}",result);
        }else{
            println!("Failed!!!!");
        }
    
        match get_value_success(true){
            Ok(result) => println!("Success: {}",result),
            Err(msg) => println!("Failer: {}",msg),
        }
    }
```
    
