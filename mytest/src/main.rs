use std::iter;

fn solution(N: i32, M: i32, data: Vec<i32>) -> i32 {
  // Edit your code here
  let mut sum=data[..M as usize].iter().sum::<i32>();
  (M..N).into_iter().for_each(|x|{
    let tmp =sum+data[x as usize]-data[(x-1) as usize];
    if sum<=tmp{ 
      sum=tmp;
    }
  });
  -1
}

fn main() {
  // Add your test cases here
  println!("{}", solution(5, 1, vec![1, 3, -9, 2, 4]) == 6);
  println!("{}", solution(5, 3, vec![1, 3, -9, 2, 4]) == 11);
}
