pub fn roman_to_int(s: &str) -> Result<Vec<u32>, &'static str> {
    let mut iter = s.chars().peekable();
    let mut nums = Vec::new();

    while let Some(c) = iter.next() {
        nums.push(match c {
            'I' => {
                if iter.next_if_eq(&'I').is_some() {
                    if iter.next_if_eq(&'I').is_some() { 3 }
                    else { 2 }
                }
                else if iter.next_if_eq(&'V').is_some() { 4 }
                else if iter.next_if_eq(&'X').is_some() { 9 }
                else { 1 }
            },

            'V' => {
                if iter.next_if_eq(&'I').is_some() {
                    if iter.next_if_eq(&'I').is_some() {
                        if iter.next_if_eq(&'I').is_some() { 8 }
                        else { 7 }
                    } else { 6 }
                } else { 5 }
            },
            
            'X' => {
                if iter.next_if_eq(&'L').is_some() { 40 }
                else if iter.next_if_eq(&'C').is_some() { 90 }
                else { 10 }
            },

            'C' => {
                if iter.next_if_eq(&'D').is_some() { 400 }
                else if iter.next_if_eq(&'M').is_some() { 900 }
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
