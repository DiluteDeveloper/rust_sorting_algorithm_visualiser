

pub fn bubble_sort(vector : &mut Vec<i32>) -> &mut Vec<i32> {
    for (i, mut x) in vector.iter_mut().enumerate() {

        let mut x1: &mut i32 = &mut vector[i + 1];
        // if next value is greater than current
        if x > x1 {
            let x_temp = x;
            x = x1;
            x1 = x_temp;
        }
    }

    vector
}