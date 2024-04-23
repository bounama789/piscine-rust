// pub fn delete_and_backspace(s: &mut String) {
//     let  q: &mut String = s;
//     q.char_indices().for_each(|(idx, c)| {
//         if c == '-' {
//             q.replace_range(idx-1..=idx, "");
//         }
//     });
    // {
    // Some((i, ch)) => {
    //     if *ch == '-' {
    //         println!("range: {}..{}",i-1,i);
    //         s.replace_range(i - 1..=*i, "");
    //     }
    // }
    // None => todo!(),
// }

// pub fn do_operations(v: &mut Vec<String>) {}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_delete_and_backspace() {
//         let mut a = String::from("bpp--o+er+++sskroi-++lcw");
//         delete_and_backspace(&mut a);
//         assert_eq!(a, "borrow");
//     }

//     #[test]
//     fn test_do_operations() {
//         let mut b: Vec<String> = vec![
//             "2+2".to_string(),
//             "3+2".to_string(),
//             "10-3".to_string(),
//             "5+5".to_string(),
//         ];
//         do_operations(&mut b);
//         assert_eq!(b, ["4", "5", "7", "10"]);
//     }
// }
