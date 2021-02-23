use std::collections::HashMap;
use std::option::Option;

/**
 *
 *
 *
 */
#[derive(Debug)]
#[warn(unused_variables)]
pub struct Request<'re> {
    //
    _envs: HashMap<&'re String, &'re String>,
    //
    _params: HashMap<&'re String, &'re String>,
}
/**
 *
 *
 *
 */
impl<'re> Request<'re> {
    /**
     *
     *
     */
    pub fn new() -> Option<Request<'re>> {
        let mut _envs_hash: HashMap<&'re String, &'re String> = HashMap::new();
        let mut _params_hash: HashMap<&'re String, &'re String> = HashMap::new();
        //   let mut _envs        : Option<HashMap<&'re String,&'re String>> = Option::Some(_envs_hash);
        //   let mut _params      : Option<HashMap<&'re String,&'re String>> = Option::Some(_params_hash);
        return Option::Some(Request {
            _envs: _envs_hash,
            _params: _params_hash,
        });
    }
    /**
     *
     *
     *
     */
    pub fn get_param(&self, _key: &'re String, _def: &'re String) -> Option<&'re String> {
        if self._params.contains_key(_key) {
            return Option::Some(self._params.get(_key).unwrap());
        } else {
            return Option::Some(_def);
        }
    }
    /**
     *
     *
     *
     */
    pub fn set_param(&mut self, _key: &'re String, _value: &'re String) {
        self._params.insert(_key, _value);
    }
    /**
     *
     *
     *
     */
    pub fn get_evn(&self, _key: &'re String, _def: &'re String) -> Option<&'re String> {
        if self._envs.contains_key(_key) {
            return Option::Some(self._envs.get(_key).unwrap());
        } else {
            return Option::Some(_def);
        }
    }
    /**
     *
     *
     *
     */
    pub fn set_env(&mut self, _key: &'re String, _value: &'re String) {
        self._envs.insert(_key, _value);
    }
}
