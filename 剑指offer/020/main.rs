impl Solution {
    pub fn is_number(s: String) -> bool {
        let s = s.trim().as_bytes();
        //println!("{:?}",s);
        let mut state = 0;
        let mut i = 0;
        while i < s.len() {
            match state {
                0 => {
                    if s[i] == b'+' || s[i] == b'-' {
                        state = 1;
                    }else if s[i] == b'.' {
                        state = 3;
                    }else if s[i] <= b'9' && s[i] >= b'0' {
                        state = 2;
                        if i == s.len() - 1 {return true;}
                    }else {
                        return false;
                    }
                },
                1 => {
                    if s[i] <= b'9' && s[i] >= b'0' {
                        state = 2;
                        if i == s.len() - 1 {return true;}
                    }else if s[i] == b'.' {
                        state = 3;
                    }else {
                        return false;
                    }
                },
                2 => {
                    
                    if s[i] <= b'9' && s[i] >= b'0' {
                        state = 2;
                        if i == s.len() - 1 {return true;}
                    }else if s[i] == b'.'{
                        state = 4;
                        if i == s.len() - 1 {return true;}
                    }else if s[i] == b'e' || s[i] == b'E' {
                        state = 5;
                    }else {
                        return false;
                    }
                },
                3 => {
                    if s[i] <= b'9' && s[i] >= b'0' {
                        state = 4;
                        if i == s.len() - 1 {return true;}
                    }else {
                        return false;
                    }
                },
                4 => {
                    
                    if s[i] <= b'9' && s[i] >= b'0' {
                        state = 4;
                        if i == s.len() - 1 {return true;}
                    }else if s[i] == b'e' || s[i] == b'E' {
                        state = 5;
                    }else {
                        return false;
                    }
                },
                5 => {
                    if s[i] == b'+' || s[i] == b'-' {
                        state = 6;
                    }else if s[i] <= b'9' && s[i] >= b'0' {
                        state = 7;
                        if i == s.len() - 1 {return true;}
                    }else {
                        return false;
                    }
                },
                6 => {
                    if s[i] <= b'9' && s[i] >= b'0' {
                        state = 7;
                        if i == s.len() - 1 {return true;}
                    }else {
                        return false;
                    }
                },
                7 => {
                    
                    if  s[i] <= b'9' && s[i] >= b'0' {
                        state = 7;
                        if i == s.len() - 1 {return true;}
                    }else {
                        return false;
                    }
                },
                _ => {
                    return false;
                }
            }
            i += 1;
        }
        return false;
    }
}