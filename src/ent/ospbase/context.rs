use std::collections::HashMap;

#[derive(Debug,Clone)]
/**
*
*
*/
pub struct Context<'ctx> {
    // 
    _context_map: HashMap<&'ctx String, &'ctx String>,
}
/**
*
*
*/
impl <'ctx>Context<'ctx> {
    /**
    *
    *
    */
    pub fn new() -> Self { 
        let mut _map: HashMap<&'ctx String, &'ctx String> = HashMap::new();
        Self { _context_map:_map } 
    }

    
}