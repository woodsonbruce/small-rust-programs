// recursive fib
fn fib(n: u32) -> u32 {
    if n == 0 || n == 1 {
        n
    } else {
        fib(n - 1) + fib(n - 2)
    }
}

// selection sort
fn select(mut n: Vec<i32>) -> Vec<i32> {
    for j in 0..(n.len() - 1) {
        let mut lowest: usize = j;
        for i in (j + 1)..n.len() {
            if n[i] < n[lowest] {
                lowest = i;
            }
        }
        if lowest != j {
            let temp = n[j];
            n[j] = n[lowest];
            n[lowest] = temp;
        }
    }
    n
}

// insertion sort
fn insert(mut n: Vec<i32>) -> Vec<i32> {
    for i in 1..n.len() {
        for j in 0..i {
            if n[j] > n[i] {
                let temp = n[j];
                n[j] = n[i];
                n[i] = temp;
            }
        }
    }
    n
}

fn main() {
    println!("fib(9) is {}", fib(9));
    println!(
        "[8,2,3,1,5,4,6,7] sorted is {:?}",
        select(vec![8, 2, 3, 1, 5, 4, 6, 7])
    );
    println!(
        "[8,2,3,1,5,4,6,7] sorted is {:?}",
        insert(vec![8, 2, 3, 1, 5, 4, 6, 7])
    );
}
