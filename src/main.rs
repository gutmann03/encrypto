pub mod caesar_ciepher{
    pub fn pseudo_shift_right(shift: u8, data: &String) -> String {
        let mut shifted = Vec::new();
        for ch in data.as_bytes() {
            shifted.push((ch + shift)%128);
        }

        String::from_utf8(shifted).expect("error")
    }

    pub fn pseudo_shift_left(shift: u8, data: &String) -> String {
        let mut shifted = Vec::new();
        for ch in data.as_bytes() {
            shifted.push((ch - shift)%128);
        }

        String::from_utf8(shifted).expect("error")
    }
}


fn main() {
    let s = String::from("hello");
    println!("s as bytes {:?}", &s.as_bytes());
    let mut sN: String;

    for i in 0..=128 {
        println!("shift: {}", i);
        sN = caesar_ciepher::pseudo_shift_right(i, &s);
        println!("s: {}", &s);
        println!("sN: {}", &sN);
        println!("sN as bytes {:?}", &sN.as_bytes());
        println!();
    }

    
}
