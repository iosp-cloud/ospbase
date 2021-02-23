use std::collections::HashMap;
use std::rc::Weak;
//use std::rc::Rc;
use crate::ent::ospbase::datatypes::ColumnValue;
//use crate::ent::ospbase::metadata::DOMetadata;

/**
*
*/
#[derive(Debug,Copy,Clone)]
pub struct EFDataset<'dataset> {
    // 数据集名称
    pub table_name: &'dataset String,
    // 行数据集合
    pub rowsets: Option<&'dataset Vec<&'dataset EFRowset<'dataset>>>,
    // 以主键为索引的HashMap
    pub rowmaps: Option<&'dataset HashMap<&'dataset String,&'dataset EFRowset<'dataset>>>,
    // 主键数组
    pub keysets: Option<&'dataset Vec<&'dataset String>>,
}
/**
*
*/
#[derive(Debug,Copy, Clone)]
pub struct EFRowset<'dataset> {
    // 每行的数据放在Vec中
    pub rowdata: Option<&'dataset Vec<ColumnValue<'dataset>>>,
    //pub dataset，可以为None
    pub dataset: Option<&'dataset Weak<EFDataset<'dataset>>>,
    // 下级可以有多个DataSet
    pub subdatasets: Option<&'dataset Box<HashMap<&'dataset String ,&'dataset EFDataset<'dataset>>>>,
}
