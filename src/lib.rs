use std::mem;

#[macro_export]
macro_rules! match_all {
   ($val:expr, IfNoMatch => $c:expr, $($($p:pat)|+ => $b:expr),+) => {{
        unsafe {
            let val = $val;
            let mut matched = false;
            let mut ret = mem::uninitialized();
            $(
                loop {
                    $(
                        if let $p = val {
                            matched = true;
                            ret = $b;
                            break
                        }
                    )+
                    break
                }
            )+
            if !matched {
                ret = $c;
            }
            ret
        }
   }};
   ($val:expr, $($($p:pat)|+ => $b:expr),+) => {{
        let val = $val;
        $(
            loop {
                $(
                    if let $p = val {
                        $b;
                        break
                    }
                )+
                break
            }
        )+
   }};
}
