use paste::paste;
// use strum_macros::{Display, EnumString};
pub trait Method {
    fn name(&self) -> &str;
}



// macro_rules! as_item {
//     ($i:item) => {
//         $i
//     };
// }

macro_rules! method_enum {
    ($name:ident; $($body:tt)*) => {
        paste! {
            #[derive(PartialEq,Debug)]
            pub enum $name { $( $body )* }
        }
    };
}

method_enum!(Item; Get, Create, Delete, Update);
method_enum!(Action; Create, Delete, Get, Update);
method_enum!(Alert; Get);
method_enum!(ApiInfo; Version);

impl Method for Item {
    fn name(&self) -> &str {
        match &self {
            Item::Get => "item.get",
            Item::Create => "item.create",
            Item::Delete => "item.delete",
            Item::Update => "item.update",
        }
    }
}



// #[derive(Debug, Eq, PartialEq, EnumString, Display)]
// pub enum Item {
//     #[strum(to_string="item.create")]
//     Create,
//     #[strum(to_string="item.delete")]
//     Delete,
//     #[strum(to_string="item.get")]
//     Get,
//     #[strum(to_string="item.update")]
//     Update,
// }



mod tests {
    #[allow(unused_imports)]
    use super::Item;

    #[test]
    fn test_generated_enums() {
        // Enum Item
        assert_eq!(Item::Get, Item::Get);
        assert_eq!(Item::Create, Item::Create);
        assert_ne!(Item::Get, Item::Create);
    }
}
