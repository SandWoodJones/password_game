pub fn roman_to_int(s: &str) -> Result<Vec<u32>, &'static str> {
    let mut iter = s.chars().peekable();
    let mut nums = Vec::new();

    while let Some(c) = iter.next() {
        nums.push(match c {
            'I' => {
                if let Some(_) = iter.next_if_eq(&'I') {
                    if let Some(_) = iter.next_if_eq(&'I') { 3 }
                    else { 2 }
                }
                else if let Some(_) = iter.next_if_eq(&'V') { 4 }
                else if let Some(_) = iter.next_if_eq(&'X') { 9 }
                else { 1 }
            },

            'V' => {
                if let Some(_) = iter.next_if_eq(&'I') {
                    if let Some(_) = iter.next_if_eq(&'I') {
                        if let Some(_) = iter.next_if_eq(&'I') { 8 }
                        else { 7 }
                    } else { 6 }
                } else { 5 }
            },
            
            'X' => {
                if let Some(_) = iter.next_if_eq(&'L') { 40 }
                else if let Some(_) = iter.next_if_eq(&'C') { 90 }
                else { 10 }
            },

            'C' => {
                if let Some(_) = iter.next_if_eq(&'D') { 400 }
                else if let Some(_) = iter.next_if_eq(&'M') { 900 }
                else { 100 }
            }

            'L' => 50,
            'D' => 500,
            'M' => 1000,

            _ => return Err("invalid roman numeral")
        });
    }

    Ok(nums)
}
