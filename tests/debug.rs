mod test_debug {
    //use libaki_unbody::*;
    #[test]
    fn size_of() {
        assert_eq!(std::mem::size_of::<&String>(), 8);
        assert_eq!(std::mem::size_of::<Box<String>>(), 8);
        assert_eq!(std::mem::size_of::<String>(), 24);
        assert_eq!(std::mem::size_of::<Vec<String>>(), 24);
    }
}
