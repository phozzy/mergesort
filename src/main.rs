fn sort(unsorted: Vec<i32>) -> Vec<i32> {
    let length = unsorted.len();
    if length == 1 {
        unsorted
    } else {
        let (first_unsorted, second_unsorted) = unsorted.split_at(length / 2);
        merge(sort(first_unsorted.to_vec()), sort(second_unsorted.to_vec()))
    }
}
fn merge(first_sorted: Vec<i32>, second_sorted: Vec<i32>) -> Vec<i32> {
    let mut output: Vec<i32> = Vec::new();
    let mut first = first_sorted.iter();
    let mut second = second_sorted.iter();
    let mut f;
    let mut s;
    let mut fi = true;
    let mut si = true;
    match first.next() {
        Some(val) => f = val,
        None => panic!("Empy array!"),
    };
    match second.next() {
        Some(val) => s = val,
        None => panic!("Empy array!"),
    };
    loop {
        if f < s && fi || !si {
            output.push(*f);
            match first.next() {
                Some(val) => {
                    f = val;
                },
                None => {
                    fi = false;
                },
            };
        } else {
            output.push(*s);
            match second.next() {
                Some(val) => {
                    s = val;
                },
                None => {
                    si = false;
                },
            };
        };
        if !(fi || si) {
            break output;
        }
    }
}
fn main() {
    let second_sorted = vec![3, 5, 9];
    let first_sorted = vec![2, 7];
    let output = merge(first_sorted, second_sorted);
    for it in output {
        println!("{}", it);
    };
    println!("sorted");
    let unsorted = vec![8,2,3,1,5];
    for it in sort(unsorted) {
        println!("{}", it);
    };
}
