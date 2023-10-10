fn main() {
    let arr = vec![7, 1, 3, 4, 1, 7];
    let res = minimumDistances(&arr);
    println!("Result: {}", res);
}


use std::collections::HashMap;

/*
 * Complete the 'minimumDistances' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts INTEGER_ARRAY a as parameter.
 */

fn minimumDistances(a: &[i32]) -> i32 {
    let mut occ = HashMap::new();
    for (ind, el) in a.iter().enumerate() {
        println!{"el: {}", el};
        
        if let None = occ.get(el) {
            occ.insert(*el, Vec::<usize>::new());
        }
        
        occ.get_mut(el).unwrap().push(ind);
        
        
    }
    println!("occ: {:?}", occ);
    let mut min_dist_opt: Option<i32> = None;
    for (key, val) in &mut occ {
        if val.len() == 0 { continue };
        val.sort();
        
        for i in 1..val.len() {
            let current_dist: i32 = (val[i] - val[i-1]) as i32;                
            if let Some(min_dist) = min_dist_opt {
                if current_dist < min_dist { min_dist_opt = Some(current_dist) };
            } else {
               min_dist_opt = Some(current_dist); 
            }
        }
    }
    return min_dist_opt.unwrap_or(-1);
}