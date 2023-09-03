pub mod helpers {
    pub fn is_line_winner(vector: &Vec<char>) -> bool {
        let first_element = vector.get(0).unwrap();

        for item in vector.iter() {
            if ( *item != *first_element ) || *item == ' ' {
                return false;
            }
        }

        true
    }

    // pub fn has_element<'a, T, U>(vector: Vec<T>, element: U) -> Option<bool>
    // where T: PartialEq<U> {
    //     let first_element = vector.get(0);
    //     if first_element.is_none() {
    //         return None;
    //     }

    //     for item in vector.iter() {
    //         if *item.clone() == element {
    //             return Some(true)
    //         }
    //     }

    //     Some(false)
    // }
}