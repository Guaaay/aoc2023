use std::{collections::HashSet, ops::Sub, cmp::{max, min}};

#[derive(PartialEq, Debug, Clone)]
struct Segment{
    id: i64,
    p0: (f64,f64),
    p1: (f64,f64),
}

// fn ccw(p0: (f64,f64), p1: (f64,f64), s2.p0: (f64,f64)) -> bool{
//     return (p1.1 - p0.1) * (s2.p0.0 - p0.0) > (s2.p0.1 - p0.1) * (p1.0 - p0.0);
// }
// fn check_segment_intersect(s1: Segment, s2: Segment) -> bool{
//     return ccw(s1.p0,s2.p0,s2.p1) != ccw(s1.p1,s2.p0,s2.p1) && ccw(s1.p0,s1.p1,s2.p0) != ccw(s1.p0,s2.p1,s2.p1);
// }

fn on_segment(p: (f64,f64), q: (f64,f64), r:(f64,f64)) -> bool
{ 
    if q.0 <= f64::max(p.0, r.0) && q.0 >= f64::min(p.0, r.0) && 
        q.1 <= f64::max(p.1, r.1) && q.1 >= f64::min(p.1, r.1) {
            return true;
        }
  
    return false; 
} 
  
// To find orientation of ordered triplet (p, q, r). 
// The function returns following values 
// 0 --> p, q and r are collinear 
// 1 --> Clockwise 
// 2 --> Counterclockwise 
fn orientation(p: (f64,f64), q: (f64,f64), r:(f64,f64)) -> i32 
{ 
    // See https://www.geeksforgeeks.org/orientation-3-ordered-points/ 
    // for details of below formula. 
    let val = (q.1 - p.1) * (r.0 - q.0) - 
              (q.0 - p.0) * (r.1 - q.1); 
  
    if val == 0.0 {return 0};  // collinear 
  
    if val > 0.0{
        return 1; // clock wise 
    }
    else{
        return 2; // counterclock wise 
    }
} 
  
// The main function that returns true if line segment 'p1s1.p1' 
// and 's2.p0s2.p1' intersect. 
fn do_intersect(s1: Segment, s2: Segment) -> bool
{ 
    // Find the four orientations needed for general and 
    // special cases 
    let o1 = orientation(s1.p0, s1.p1, s2.p0); 
    let o2 = orientation(s1.p0, s1.p1, s2.p1); 
    let o3 = orientation(s2.p0, s2.p1, s1.p0); 
    let o4 = orientation(s2.p0, s2.p1, s1.p1); 
  
    // General case 
    if (o1 != o2 && o3 != o4) {
        return true; }
  
    // Special Cases 
    // s1.p0, s1.p1 and s2.p0 are collinear and s2.p0 lies on segment s1.p0s1.p1 
    if (o1 == 0 && on_segment(s1.p0, s2.p0, s1.p1)) {return true}; 
  
    // s1.p0, s1.p1 and s2.p1 are collinear and s2.p1 lies on segment s1.p0s1.p1 
    if (o2 == 0 && on_segment(s1.p0, s2.p1, s1.p1)) {return true}; 
  
    // s2.p0, s2.p1 and s1.p0 are collinear and s1.p0 lies on segment s2.p0s2.p1 
    if (o3 == 0 && on_segment(s2.p0, s1.p0, s2.p1)) {return true}; 
  
     // s2.p0, s2.p1 and s1.p1 are collinear and s1.p1 lies on segment s2.p0s2.p1 
    if (o4 == 0 && on_segment(s2.p0, s1.p1, s2.p1)) {return true}; 
  
    return false; // Doesn't fall in any of the above cases 
} 


fn get_segments(i: i64,point: (f64,f64), dir: (i64,i64))->Segment{
    //Get the point where it intersects the box (200000000000000,200000000000000), (200000000000000,400000000000000), (400000000000000,400000000000000), (400000000000000,200000000000000)
    let mut p0 = point;
    let mut p1 = (0.0,0.0);
    let mut poss1: (f64, f64) = point;

    let mut poss2 = (0.0,0.0);
    let mut intersections = 0;

    //let lower_bound = 7.0;
    //let upper_bound = 27.0;

    let lower_bound = 200000000000000.0;
    let upper_bound = 400000000000000.0;
    //Get line equation
    let m = dir.1 as f64 / dir.0 as f64;
    let c = point.1 as f64 - m * point.0 as f64;
    //Check left bound
    let y = m * lower_bound + c;
    //println!("y: {}", y);
    if y >= lower_bound && y <= upper_bound {
        if intersections == 0{
            poss1 = (lower_bound, y);
        }else{
            poss2 = (lower_bound, y);
        }
        intersections += 1;
    }
    //Check right bound
    let y = m * upper_bound + c;
    if y >= lower_bound && y <= upper_bound {
        if intersections == 0{
            poss1 = (upper_bound, y);
        }else{
            poss2 = (upper_bound, y);
        }
        intersections += 1;
    }
    //Check top bound
    let x = (lower_bound - c) / m;
    if x >= lower_bound && x <= upper_bound {
        if intersections == 0{
            poss1 = (x, lower_bound);
        }else{
            poss2 = (x, lower_bound);
        }
        intersections += 1;
    }
    //Check bottom bound
    let x = (upper_bound - c) / m;
    if x >= lower_bound && x <= upper_bound {
        if intersections == 0{
            poss1 = (x, upper_bound);
            
        }else{
            poss2 = (x, upper_bound);
        }
        intersections += 1;
    }
    if  dir.0 < 0 {
        if poss1.0< point.0 && poss2.0 < point.0{
            if poss1.0> poss2.0{
                p0 = poss1;
                p1 = poss2;
            }
            else{
                p0 = poss2;
                p1 = poss1;
            }
        }
        else if poss1.0< point.0{
            p1 = poss1;
            p0 = point;
        }
        else if poss2.0 < point.0{
            p1 = poss2;
            p0 = point;
        }
    }
    else {
        if poss1.0> point.0 && poss2.0 > point.0{
            if poss1.0> poss2.0{
                p0 = poss1;
                p1 = poss2;
            }
            else{
                p0 = poss2;
                p1 = poss1;
            }
        }
        else if poss1.0> point.0{
            p1 = poss1;
            p0 = point;
        }
        else if poss2.0 > point.0{
            p1 = poss2;
            p0 = point;
        }
    }
    


    Segment{id: i, p0: p0, p1: p1}
}


fn part1(lines: &Vec<&str>) {
    let mut segments: Vec<Segment> = Vec::new();
    let mut i = 1;
    for line in lines {
        let parsed = line.split('@').collect::<Vec<&str>>();
        let point = parsed[0].split(',').collect::<Vec<&str>>();
        let dir = parsed[1].split(',').collect::<Vec<&str>>();
        let point = (point[0].trim().parse::<f64>().unwrap(), point[1].trim().parse::<f64>().unwrap());
        let dir = (dir[0].trim().parse::<i64>().unwrap(), dir[1].trim().parse::<i64>().unwrap());
        let segment = get_segments(i,point, dir);
        // if segment == Segment{p0: (0.0,0.0), p1: (0.0,0.0)}{
        //     println!("Error");
        // }
        //println!("point: {:?}", point);
        //println!("dir: {:?}", dir);
        //println!("line: {:?}", line);
        //println!("Segment: {:?}", segment);
        segments.push(segment);
        i += 1;
    }
    let mut sum = 0;
    let mut checked: HashSet<(i64,i64)> = HashSet::new();
    let mut i = 0;
    for s1 in &segments {
        i += 1;
        println!("i: {}", i);
        
        for s2 in &segments.clone() {
            
            if s1.id == s2.id{
                continue;
            }
            if checked.contains(&(s1.id,s2.id)) || checked.contains(&(s2.id,s1.id)){
                //println!("Already checked");
                continue;
            }
            checked.insert((s1.id,s2.id));
            if s1 != s2{
                if do_intersect(s1.clone(), s2.clone()){
                    //println!("Intersecting segments: {:?} {:?}", s1.id, s2.id);
                    //println!("Segment 1: {:?}", s1);
                    //println!("Segment 2: {:?}", s2);
                    sum += 1;
                }
            }
            //println!("--------------------------");
        }
    }
    println!("Intersections: {}", sum);

}

fn main() {
    let lines = include_str!("../input.txt").lines().collect::<Vec<&str>>();
    part1(&lines);
}
