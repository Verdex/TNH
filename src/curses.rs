

extern {
    fn jabber( x : i32 ) -> i32;
}

pub fn wocky( x : i32 ) -> i32 {
    unsafe {
        jabber( x )
    }
}
