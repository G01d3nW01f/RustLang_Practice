#TypeGet

this function is for check the types for variable or any objects and some values
you know I'm rustlang noob, so I use this for test and learn


```rust
pub fn get_type<T>(_: T) -> &'static str {
    std::any::type_name::<T>()
}
```

#usage
    
 ```rust
 fn main(){
    
 let variable: u32 = 24;
 let type_name = type_get(variable);
    
 println!("{}",type_name);
    
 }
    
 ```
