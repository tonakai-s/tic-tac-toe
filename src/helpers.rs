pub mod helpers {
    pub fn every_same_element<T, F>(vector: &Vec<T>, validator: F) -> Option<bool>
    where F: Fn(&T) -> bool {
        let first_element = vector.get(0);
        if first_element.is_none() {
            return None;
        }

        for item in vector.iter() {
            let matched = validator(item);
            if matched == false {
                return Some(false);
            }
        }

        Some(true)
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