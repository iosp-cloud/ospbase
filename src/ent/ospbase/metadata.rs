use crate::ent::ospbase::context::Context;
use std::collections::HashMap;
//use std::rc::Rc;
//use std::rc::Weak;
use std::option::Option;
use crate::ent::ospbase::dataset::EFRowset;
use crate::ent::ospbase::dataset::EFDataset;

#[warn(unused_variables)]
/**
 *
 *  
 **/
 #[derive(Debug,Clone)]
pub struct DOMetadata<'meta> {
    // ID
    doname     : &'meta String,
    // 单行信息
    dorowset   : Option<Box<EFRowset<'meta>>>,
    // 列信息
    docolumns  : Option<Box<EFDataset<'meta>>>,
    // 主键与索引
    doindexs   : Option<Box<EFDataset<'meta>>>,
     // 引用DO
    refdometa  : Option<HashMap<&'meta String,Box<DOMetadata<'meta>>>>
}
/**
 *
 *  
 **/
impl<'meta> DOMetadata<'meta> {
    /**
    *
    *
    */
    pub fn get_doname(&self) -> &'meta String {
        return self.doname;
    }
    /**
     * 
     * 
     */
    pub fn get_dorowset(&self) -> Option<&Box<EFRowset<'meta>>> {
        return self.dorowset.as_ref();
    }
    /**
     * 
     * 
     */
    pub fn get_docolumns(&self) -> Option<&Box<EFDataset<'meta>>> {
        return self.docolumns.as_ref();
    }
    /**
     * 
     * 
     */
    pub fn get_doindexs(&self) -> Option<&Box<EFDataset<'meta>>> {
        return self.doindexs.as_ref();
    }
    /**
     * 
     * 
     */
    pub fn get_refdometa(&self) -> Option<&HashMap<&'meta String,Box<DOMetadata<'meta>>>> {
        //let _hmap = Box::new(&self.refdometa.unwrap());
        //let mut res = self.refdometa;
        return self.refdometa.as_ref();
    }
}

/**
 * 
 * 
 **/
 #[derive(Debug,Clone)]
 pub struct DCTMetadata<'meta> {
     // DCT Name
    dctname    : &'meta String,
     // MetaData
    dometadata : Option<Box<DOMetadata<'meta>>>,
     // 定义行
    dctrowset  : Option<Box<EFRowset<'meta>>>,
 }
/**
 *
 *
 */
impl<'meta> DCTMetadata<'meta> {
    /**
    *
    *
    */
    pub fn get_dctname(&self) -> &'meta String {
        return self.dctname;
    }
    /**
     * 
     * 
     */
    pub fn get_dometadata(&self) -> Option<&Box<DOMetadata<'meta>>> {
        return self.dometadata.as_ref();
    }
    /**
     * 
     * 
     */
     pub fn get_dctrowset(&self) -> Option<&Box<EFRowset<'meta>>> {
        return self.dctrowset.as_ref();
    }
} 

 /**
 * 
 * 
 **/
 #[derive(Debug,Clone)]
 pub struct FCTMetadata<'meta> {
    // FCT Name
    fctname    : &'meta String,
    // MetaData
    dometadata : Option<Box<DOMetadata<'meta>>>,
    // 定义行
    fctrowset  : Option<Box<EFRowset<'meta>>>,
}
 /**
 * 
 * 
 **/
impl<'meta> FCTMetadata<'meta> {
    /**
    *
    *
    */
    pub fn get_fctname(&self) -> &'meta String {
        return self.fctname;
    }
    /**
     * 
     * 
     */
    pub fn get_dometadata(&self) -> Option<&Box<DOMetadata<'meta>>> {
        return self.dometadata.as_ref();
    }
    /**
     * 
     * 
     */
    pub fn get_fctrowset(&self) -> Option<&Box<EFRowset<'meta>>> {
        return self.fctrowset.as_ref();
    }
} 

 /**
 * 
 * 
 **/
 #[derive(Debug,Clone)]
 pub struct BIZMetadata<'meta> {
     // BIZ Name
    bizname    : &'meta String,
     // 定义行
    bizrowset  : Option<Box<EFRowset<'meta>>>,
 }
 /**
 * 
 * 
 **/
impl<'meta> BIZMetadata<'meta> {
    /**
    *
    *
    */
    pub fn get_bizname(&self) -> &'meta String {
        return self.bizname;
    }
    /**
     * 
     * 
     */
    pub fn get_bizrowset(&self) -> Option<&Box<EFRowset<'meta>>> {
        return self.bizrowset.as_ref();
    }
}
 
/**
 * 
 * 
 **/
 #[derive(Debug,Clone)]
pub struct MetadataManager<'meta> {
    // 
    dometa_hashmap    : Option<&'meta HashMap<&'meta String,&'meta DOMetadata<'meta>>>,
    // 
    dctmeta_hashmap   : Option<&'meta HashMap<&'meta String,&'meta DCTMetadata<'meta>>>,
    // 
    fctmeta_hashmap   : Option<&'meta HashMap<&'meta String,&'meta FCTMetadata<'meta>>>,
    //
    bizmeta_hashmap   : Option<&'meta HashMap<&'meta String,&'meta BIZMetadata<'meta>>>,
}
/**
 * 
 * 
 **/
impl<'meta> MetadataManager<'meta> {
    /**
     * 
     * 
     **/
    pub fn  load_dometadata(_doname :  &'static str,_context:&'meta Context)  -> Option<&'meta DOMetadata<'meta>>  {
        return Option::None;
    }
    /**
     * 
     * 
     **/
    pub fn load_dctmetadata(_dctname : &'static str,_context:&'meta Context) -> Option<&'meta DCTMetadata<'meta>> {
        return Option::None;
    }
    /**
     * 
     * 
     **/
    pub fn load_fctmetadata(_fctname : &'static str,_context:&'meta Context) -> Option<&'meta FCTMetadata<'meta>> {
        return Option::None;
    }
    /**
     * 
     * 
     **/
    pub fn load_bizmetadata(_bizname : &'static str,_context:&'meta Context) -> Option<&'meta BIZMetadata<'meta>> {
        return Option::None;
    }
}


