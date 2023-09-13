fn main() {
    assert_eq!(Solution::roman_to_int(String::from("III")), 3);
    assert_eq!(Solution::roman_to_int(String::from("LVIII")), 58);

    //MCMXCIV => M(1000) + CM(900) + XC(90) + IV(4) 
    assert_eq!(Solution::roman_to_int(String::from("MCMXCIX")), 1999);
}

struct Solution;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut res = 0;
        let s: Vec<char> = s.chars().collect();

        let mut i = 0;
        while i<s.len(){
            let s1 = value(s[i]);
            
            if i+1 < s.len(){
                let s2 = value(s[i+1]);

                if s1 >= s2{
                    res = res + s1;
                }else{
                    res = res + s2 - s1;
                    i +=1;
                }
            }else{
                res = res + s1;
            }
            i +=1;
        }
        res
    }
}

pub fn value(a: char) -> i32{
    match a {
        'I'  => 1,
        'V'  => 5,
        'X'  => 10,
        'L'  => 50,
        'C'  => 100,
        'D'  => 500,
        'M'  => 1000,
        _ => 0
    }
}

// I  => 1,
// IV => 4,
// V  => 5,
// IX => 9,
// X  => 10,
// XL => 40,
// L  => 50,
// XC => 90,
// C  => 100,
// CD => 400,
// D  => 500,
// CM => 900,
// M  => 1000

