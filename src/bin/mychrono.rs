use chrono::{Duration, Utc, Days, Months};

fn main() {
    // 获取 世界统一时间的现在时间
    let now = Utc::now();
    // 获取当地时间的现在时间
    // let local=Local::now();
    println!("当前时间: {}", now);
    // checked_add_signed 添加指定的时间到
    let almost_three_weeks_from_now = now.checked_add_signed(Duration::weeks(2));
    // checked_add_days 添加指定的天数
    let after_one_day=now.checked_add_days(Days::new(1));
    // checked_sub_months 添加指定的月数
    let after_one_mouth=now.checked_sub_months(Months::new(1));
    // 计算时间差
    // let start_of_period = Utc.ymd(2020, 1, 1).and_hms(0, 0, 0);
    // let end_of_period = Utc.ymd(2021, 1, 1).and_hms(0, 0, 0);
    // let duration = end_of_period - start_of_period;
    // println!("num days = {}", duration.num_days());
    
    match almost_three_weeks_from_now {
        Some(x) => println!("两周后的时间: {}", x),
        None => eprintln!("时间格式不对"),
    }
    match after_one_day {
        Some(x) => println!("一天后的时间: {}", x),
        None => eprintln!("时间格式不对"),
    }
    match after_one_mouth {
        Some(x) => println!("一月后的时间: {}", x),
        None => eprintln!("时间格式不对"),
    }

}