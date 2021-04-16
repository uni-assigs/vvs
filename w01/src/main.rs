// Repo: https://github.com/uni-assigs/vvs
// Author: Vinícius R. Miguel, 120.639 (https://github.com/vrmiguel)

#[cfg(test)]
mod test;

mod triangle;
use triangle::Triangle;

macro_rules! probe_triangle {
    ( $tr:expr, $id:expr ) => {
        {
            match $tr {
                Ok(triangle) => println!("{} = {:?}", $id, triangle),
                Err(_) => println!("{} is an invalid triangle!", $id)
            }
        }
    };
}

fn main() {
    let t1 = Triangle::new(2, 2, 4);
    let invalid_triplet = (1, 10, 12);
    let t2 = Triangle::from_triplet(invalid_triplet);

    probe_triangle!(t1, "T1");
    probe_triangle!(t2, "T2");

}