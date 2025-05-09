pub fn duplicate_string(string: String) -> String {

    let mut duplicate = String::new();
    for (_,value) in string.chars().enumerate(){
        duplicate += &(value.to_string() + &value.to_string());
    }
    duplicate
    
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_char_duplicate_string() {
        
        let s1 = String::from("Hello");
        let result = duplicate_string(s1);
        assert_eq!(result,"HHeelllloo");

        let s1 = String::from("世界");
        let result = duplicate_string(s1);
        assert_eq!(result,"世世界界");

    }
}
