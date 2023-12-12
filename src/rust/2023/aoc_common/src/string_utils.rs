pub trait StringManipulation{
    fn extract_numbers<'a>(&'a self) -> Box<dyn Iterator<Item=i64> + 'a>;
    fn split_in<const N: usize>(&self, pat: &str) -> [&str; N];

    fn right_of(&self, pat: &str) -> &str;

    fn left_of(&self, pat: &str) -> &str;
}

impl StringManipulation for &str {
    fn extract_numbers<'a>(&'a self) -> Box<dyn Iterator<Item=i64> + 'a> {
        Box::new(self.split_whitespace().filter_map(|s| s.parse::<i64>().ok()))
    }

    fn split_in<const N: usize>(&self, pat: &str) -> [&str; N] {
        self.split(pat).collect::<Vec<&str>>().try_into().unwrap_or_else(|v: Vec<&str>| panic!("Input cannot be split in exactly {} parts. Was split in {} parts", N, v.len()))
    }

    fn right_of(&self, pat: &str) -> &str {
        self.split_in::<2>(pat)[1]
    }

    fn left_of(&self, pat: &str) -> &str {
        self.split_in::<2>(pat)[0]
    }
}

impl StringManipulation for String {
    fn extract_numbers<'a>(&'a self) -> Box<dyn Iterator<Item=i64> + 'a> {
        Box::new(self.split_whitespace().filter_map(|s| s.parse::<i64>().ok()))
    }

    fn split_in<const N: usize>(&self, pat: &str) -> [&str; N] {
        self.split(pat).collect::<Vec<&str>>().try_into().unwrap_or_else(|v: Vec<&str>| panic!("Input cannot be split in exactly {} parts. Was split in {} parts", N, v.len()))
    }

    fn right_of(&self, pat: &str) -> &str {
        self.split_in::<2>(pat)[1]
    }

    fn left_of(&self, pat: &str) -> &str {
        self.split_in::<2>(pat)[0]
    }
}

pub fn find_matching_in_sorted<T: Ord>(to_find: &Vec<T>, in_this_list: &Vec<T>) -> usize{
    let mut j=0;
    let mut total=0;
    for x in to_find{
        while j < in_this_list.len(){
            match in_this_list.get(j) {
                None => {panic!("How ?")}
                Some(y) => {
                    if y == x{
                        total+=1;
                    }
                    if y >= x{
                        break;
                    }
                    j+=1;
                }
            }
        }
    }

    total
}