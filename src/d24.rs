use std::fs;

use ndarray::prelude::*;
use ndarray_linalg::Solve;

pub fn solve() {
    let file_path = "input/day24.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let hail: Vec<((f64, f64, f64), (f64, f64, f64))> = contents
        .split("\n")
        .filter(|e| e.len() > 0)
        .map(|e| e.replace(" ", ""))
        .map(|e| {
            let mut e = e.split("@");
            let p = e.next().unwrap().split(",").collect::<Vec<&str>>();
            let v = e.next().unwrap().split(",").collect::<Vec<&str>>();
            (
                (
                    p[0].parse().unwrap(),
                    p[1].parse().unwrap(),
                    p[2].parse().unwrap(),
                ),
                (
                    v[0].parse().unwrap(),
                    v[1].parse().unwrap(),
                    v[2].parse().unwrap(),
                ),
            )
        })
        .collect();

    let lower_bound = 200000000000000.0;
    let upper_bound = 400000000000000.0;
    let n = hail.len();

    let mut res1 = 0;
    for i in 0..n {
        for j in 0..i {
            let a = hail[i];
            let b = hail[j];

            let a1 = (a.0 .0, a.0 .1);
            let a2 = (a.0 .0 + a.1 .0, a.0 .1 + a.1 .1);
            let dax = a2.0 - a1.0;
            let day = a2.1 - a1.1;
            let am = day / dax;
            let aq = a1.1 - am * a1.0;

            let b1 = (b.0 .0, b.0 .1);
            let b2 = (b.0 .0 + b.1 .0, b.0 .1 + b.1 .1);
            let dbx = b2.0 - b1.0;
            let dby = b2.1 - b1.1;
            let bm = dby / dbx;
            let bq = b1.1 - bm * b1.0;

            let x0 = (aq - bq) / (bm - am);
            let y0 = am * x0 + aq;

            if x0 > lower_bound && x0 < upper_bound && y0 > lower_bound && y0 < upper_bound {
                if (x0 - a1.0).signum() == a.1 .0.signum()
                    && (y0 - a1.1).signum() == a.1 .1.signum()
                    && (x0 - b1.0).signum() == b.1 .0.signum()
                    && (y0 - b1.1).signum() == b.1 .1.signum()
                {
                    res1 += 1;
                }
            }
        }
    }

    let res2 = 0;

    println!("Day 24:\nTask 1:{res1:16}\nTask 2:{res2:16}");
}

fn normalize(vec: &(f64, f64, f64)) -> (f64, f64, f64) {
    let length = (vec.0.powi(2) + vec.1.powi(2) + vec.2.powi(2)).sqrt();
    (vec.0 / length, vec.1 / length, vec.2 / length)
}

fn parallel(a: &(f64, f64, f64), b: &(f64, f64, f64)) -> bool {
    let epsilon = 0.01;
    let sa = a.0.signum();
    let a = (a.0 * sa, a.1 * sa, a.2 * sa);
    let sb = b.0.signum();
    let b = (b.0 * sb, b.1 * sb, b.2 * sb);
    (a.0 - b.0).abs() < epsilon && (a.1 - b.1).abs() < epsilon && (a.2 - b.2).abs() < epsilon
}

fn product(a: &(f64, f64, f64), b: &(f64, f64, f64)) -> f64 {
    a.0 * b.0 + a.1 * b.1 + a.2 * b.2
}

fn cross_product(a: &(f64, f64, f64), b: &(f64, f64, f64)) -> (f64, f64, f64) {
    (
        a.1 * b.2 - a.2 * b.1,
        a.2 * b.0 - a.0 * b.2,
        a.0 * b.1 - a.1 * b.0,
    )
}

/*

   let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
   let hail: Vec<((i64, i64, i64), (i64, i64, i64))> = contents
       .split("\n")
       .filter(|e| e.len() > 0)
       .map(|e| e.replace(" ", ""))
       .map(|e| {
           let mut e = e.split("@");
           let p = e.next().unwrap().split(",").collect::<Vec<&str>>();
           let v = e.next().unwrap().split(",").collect::<Vec<&str>>();
           (
               (
                   p[0].parse().unwrap(),
                   p[1].parse().unwrap(),
                   p[2].parse().unwrap(),
               ),
               (
                   v[0].parse().unwrap(),
                   v[1].parse().unwrap(),
                   v[2].parse().unwrap(),
               ),
           )
       })
       .collect();

   /*
   let hail: Vec<Vec<(i64, i64, i64)>> = hail
   .iter()
   .map(|(p, v)| {
       let mut trail = Vec::new();
       for i in 1..=n {
           let i = i as i64;
           trail.push((p.0 + i * v.0, p.1 + i * v.1, p.2 + i * v.2))
       }
       trail
   })
   .collect();
   */

   let scale = 2;
   let offset = 3;
   let mut res2 = 0;
   for offset in 0..100 {
       for scale in 1..100 {
           for i in 0..n {
               let (h0p, h0v) = hail[i];
               let p0 = (
                   h0p.0 + (offset + scale * 0) * h0v.0,
                   h0p.1 + (offset + scale * 0) * h0v.1,
                   h0p.2 + (offset + scale * 0) * h0v.2,
               );
               for j in 0..n {
                   if i == j {
                       continue;
                   }
                   let (h1p, h1v) = hail[j];
                   let p1 = (
                       h1p.0 + (offset + scale * 1) * h1v.0,
                       h1p.1 + (offset + scale * 1) * h1v.1,
                       h1p.2 + (offset + scale * 1) * h1v.2,
                   );
                   let d = (
                       (p1.0 - p0.0) / scale,
                       (p1.1 - p0.1) / scale,
                       (p1.2 - p0.2) / scale,
                   );
                   for k in 0..n {
                       for l in 1..scale {
                           if i == k || j == k {
                               continue;
                           }
                           let p2_guess = (p1.0 + d.0 * l, p1.1 + d.1 * l, p1.2 + d.2 * l);
                           let (h2p, h2v) = hail[k];
                           let p2 = (
                               h2p.0 + (offset + scale * 1 + l) * h2v.0,
                               h2p.1 + (offset + scale * 1 + l) * h2v.1,
                               h2p.2 + (offset + scale * 1 + l) * h2v.2,
                           );
                           if p2.eq(&p2_guess) {
                               let collision_time = offset + scale * 1;
                               let res = (
                                   p0.0 - d.0 * collision_time,
                                   p0.1 - d.1 * collision_time,
                                   p0.2 - d.2 * collision_time,
                               );
                               res2 = res.0 + res.1 + res.2;
                               println!(online system of equation sover
                                   "{p0:?} {d:?} collision time: {collision_time} {res:?} {res2}"
                               );
                           }
                       }
                   }
               }
           }
       }
       println!("{offset}");
   }
*/
