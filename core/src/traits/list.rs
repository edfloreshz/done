pub trait List {
    fn id(&self) -> String;
    fn display_name(&self) -> String;
    fn is_owner(&self) -> bool;
    fn count(&self) -> i32;
    fn icon_name(&self) -> Option<String>;
    fn provider(&self) -> String;
    fn is_smart(&self) -> bool;
}