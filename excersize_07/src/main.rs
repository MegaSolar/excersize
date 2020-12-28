
// 직사각형 방의 면적
// 출력문과 계산 부분 분리할것
// 상수를 사용하여 변환 상수를 저장할것.
// 제곱피트에서 제곱 미터로 변환하는 공식  :: m*m = f*f* 0.09290304
use std::io;
use std::io::Write;
use std::ops::Mul;

fn main() {
    let mut length_str= String::new();
    let mut witdth_str = String::new();
    let sq_meter_num = 0.09290304;
    loop {
        print!("what is the length of the room in feet?");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut length_str).expect("test");
        if length_str.trim().is_empty() == true {println!("go to start");}

    }

    print!("what is the width of the room in feet?");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut witdth_str).expect("Can't get width");
    let res = cal_room(length_str.to_owned(),witdth_str.to_owned());
    let res_sq = res.mul(res) as f64;
    let res_meter = res_sq.mul(sq_meter_num);
    println!("You entered dimensions of {} feet by {} feet .",length_str.trim().to_string(),witdth_str.trim().to_string());
    println!("The area is ");
    println!("{} square feet ",res.to_string());
    println!("{} square meters",res_meter.to_string());
}
fn cal_room ( length: String  , width:String) ->i64 {
    let length_feet : i64 = length.trim().parse().unwrap();
    let width_feet : i64 = width.trim().parse().unwrap();
    return length_feet * width_feet;

}
