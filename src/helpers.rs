pub mod helpers {
    pub fn is_vector_winner(vector: &Vec<char>) -> bool {
        let first_element = vector.get(0).unwrap();

        for item in vector.iter() {
            if ( *item != *first_element ) || *item == ' ' {
                return false;
            }
        }

        true
    }
}