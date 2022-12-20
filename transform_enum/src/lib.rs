use derive_getters::Getters;
use derive_new::new;
use serde::Deserialize;
use serde::Serialize;
use strum_macros::EnumString;
use strum_macros::ToString;

#[cfg(test)]
mod tests {
    use crate::SortDirection;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn test_transform_enum() {
        let desc: SortDirection = "Desc".parse().unwrap();
        assert_eq!(SortDirection::Desc, desc);
    }
}


#[derive(Clone, Debug, EnumString, PartialEq)]
pub enum SortDirection {
    /// 昇順
    Asc,
    /// 降順
    Desc,
}
