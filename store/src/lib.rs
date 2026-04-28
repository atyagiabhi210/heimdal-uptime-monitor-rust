


// this api will depend on this store 
// rust has futures => promises in js

// event loop call back queue in js runtie 
// rust does not have an async runtime 

// multi threadning and async  is different 
// async vs multi threading


// here the store begins
pub struct Store {

}
 impl Store {
    pub fn create_user(&self) -> String {
        format!("creating user")
    }
    pub fn create_website(&self) -> String {
        format!("creating website")
    }
 }
