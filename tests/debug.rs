mod test_debug {
    //use libaki_unbody::*;
    #[test]
    fn size_of() {
        #[cfg(target_pointer_width = "64")]
        {
            assert_eq!(std::mem::size_of::<&String>(), 8);
            assert_eq!(std::mem::size_of::<Box<String>>(), 8);
            assert_eq!(std::mem::size_of::<String>(), 24);
            assert_eq!(std::mem::size_of::<Vec<String>>(), 24);
        }
        #[cfg(target_pointer_width = "32")]
        {
            assert_eq!(std::mem::size_of::<&String>(), 4);
            assert_eq!(std::mem::size_of::<Box<String>>(), 4);
            assert_eq!(std::mem::size_of::<String>(), 12);
            assert_eq!(std::mem::size_of::<Vec<String>>(), 12);
        }
    }
}
