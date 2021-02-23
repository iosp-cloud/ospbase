use std::collections::HashMap;

/**
 *
 *
 *
 */
#[derive(Debug)]
#[warn(unused_variables)]
pub struct Response<'re> {
    //
    _envs: HashMap<&'re String, &'re String>,
    //
    _params: HashMap<&'re String, &'re String>,
}