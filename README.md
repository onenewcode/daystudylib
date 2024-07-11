# 概述
本仓库是本人体验各种rust库的代码，会不定期跟新rust各种有趣的库。
# 生成随机值
rust中官方并没有像以他语言一样，rust并没有官方并没有提供生成随机数的工具，所以我们要借助rand包进行生成随机数。这里我们使用现在使用罪为广泛的rand包只需要引入以下依赖就能够使用。`rand = "0.8.5"`

## 生成随机数


在随机数生成器 rand::Rng 的帮助下，通过 rand::thread_rng 生成随机数。可以开启多个线程，每个线程都有一个初始化的生成器。整数在其类型范围内均匀分布，浮点数是从 0 均匀分布到 1，但不包括 1。

```rs
use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();

    let n1: u8 = rng.gen();
    let n2: u16 = rng.gen();
    println!("Random u8: {}", n1);
    println!("Random u16: {}", n2);
    // 改变类型
    println!("Random u32: {}", rng.gen::<u32>());
    println!("Random i32: {}", rng.gen::<i32>());
    println!("Random float: {}", rng.gen::<f64>());
}
```
**结果**
```shell
Random u8: 247
Random u16: 46458
Random u32: 2649532043
Random i32: 1393744920
Random float: 0.5923489382636902
```
## 生成范围内随机数

使用 Rng::gen_range，在半开放的 [0, 10) 范围内（不包括 10）生成一个随机值。

```rs
use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    println!("Integer: {}", rng.gen_range(0..10));
    println!("Float: {}", rng.gen_range(0.0..10.0));
}
```
**结果**
```shell
   let mut rng = rand::thread_rng();
    println!("Integer: {}", rng.gen_range(0..10));
    println!("Float: {}", rng.gen_range(0.0..10.0));
```

使用 Uniform 模块可以得到均匀分布的值。下述代码和上述代码具有相同的效果，但在相同范围内重复生成数字时，下述代码性能可能会更好。
```rs

use rand::distributions::{Distribution, Uniform};

fn main() {
    let mut rng = rand::thread_rng();
    let die = Uniform::from(1..7);

    loop {
        let throw = die.sample(&mut rng);
        println!("Roll the die: {}", throw);
        if throw == 6 {
            break;
        }
    }
}
```
**结果**
```shell
Roll the die: 1
Roll the die: 2
Roll the die: 6
```

## 生成自定义类型随机值


随机生成一个元组 (i32, bool, f64) 和用户定义类型为 Point 的变量。为 Standard 实现 Distribution trait，以允许随机生成。
```rs
use rand::Rng;
use rand::distributions::{Distribution, Standard};

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Distribution<Point> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Point {
        let (rand_x, rand_y) = rng.gen();
        Point {
            x: rand_x,
            y: rand_y,
        }
    }
}

fn main() {
    let mut rng = rand::thread_rng();
    let rand_tuple = rng.gen::<(i32, bool, f64)>();
    let rand_point: Point = rng.gen();
    println!("Random tuple: {:?}", rand_tuple);
    println!("Random Point: {:?}", rand_point);
}
```
**结果**
```shell
Random tuple: (590118681, false, 0.7548409339548463)
Random Point: Point { x: 914499268, y: 795986012 }
```
## 从一组字母数字字符创建随机密码

随机生成一个给定长度的 ASCII 字符串，范围为 A-Z，a-z，0-9，使用字母数字样本。

```rs
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

fn main() {
    let rand_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .map(char::from)
        .collect();

    println!("{}", rand_string);
}
```
**结果**
```shell
fwaZUzdIkK1p78fyNvh44Od5gcr3BL
```
## 从一组用户定义字符创建随机密码


使用用户自定义的字节字符串，使用 gen_range 函数，随机生成一个给定长度的 ASCII 字符串。
```rs
use rand::Rng;
fn main() {
   
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789)(*&^%$#@!~";
    const PASSWORD_LEN: usize = 30;
    let mut rng = rand::thread_rng();

    let password: String = (0..PASSWORD_LEN)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();

    println!("{:?}", password);
}
```
**结果**
```shell
"F@QNgOrsviJ2tqM$zOSJSR^Hjevvce"
```

# 日期库chrono
Rust的时间操作主要用到chrono库，接下来我将简单选一些常用的操作进行介绍，如果想了解更多细节，请查看官方文档。

```rs
use chrono::{Datelike, Duration, Local, TimeZone, Timelike};
 
fn main() {
    let fmt = "%Y年%m月%d日 %H:%M:%S";
    let now = Local::now().format(fmt);
    println!("{}", now);
 
    let mut parse = Local
        .datetime_from_str("2022年3月19日 13:30:59", fmt)
        .unwrap();
    println!("{:?}", parse);
    println!(
        "{}-{}-{} {}:{}:{}",
        parse.year(),
        parse.month(),
        parse.day(),
        parse.hour(),
        parse.minute(),
        parse.second()
    );
    println!("{}", parse.date());
    parse = Local.ymd(2012, 12, 12).and_hms(12, 12, 12);
    println!("{}", parse);
    parse = parse + Duration::days(2);
    println!("{}", parse);
    parse = parse + Duration::hours(2);
    println!("{}", parse);
    parse = parse + Duration::minutes(2);
    println!("{}", parse);
    parse = parse + Duration::seconds(2);
    println!("{}", parse);
 
}
```
# 日期及时间


## 测量运行时间
计算从 time::Instant::now 开始运行的时间 time::Instant::elapsed。

调用 time::Instant::elapsed 将返回 time::Duration，我们将在实例末尾打印该时间。此方法不会更改或者重置 time::Instant 对象。

```rs
use std::time::Instant;
use std::{thread, time};
fn main(){
    let start = Instant::now();
    // 设置休眠时间，1s，
    let ten_millis = time::Duration::from_millis(1);
    thread::sleep(ten_millis);
    let duration = start.elapsed();
    //计算获得的时间会超过1s，因为系统运行也会消耗一定时间
    println!("显示两行代码之间消耗的时间() is: {:?}", duration);
}
```
**结果**
```shell
显示两行代码之间消耗的时间() is: 8.1735ms
```
## chrono 
在rust中，使用日期库需要引入第三方库，chrono 是在rsut中使用最多的库，所以我们接下来的的日期处理都基于此库。所以需要我们在Cargo.toml引入`chrono = "0.4.31"`
### 时间计算
chrono 中提供的时间计算的方法有很多，接下来我将介绍几种常用的方法。

```rs
use chrono::{DateTime, Duration, Utc, Days, Months};

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
```
在计算时间差比较麻烦，需要先指定格式，以下是计算时间差的代码
```rs
    let start_of_period = Utc.ymd(2020, 1, 1).and_hms(0, 0, 0);
    let end_of_period = Utc.ymd(2021, 1, 1).and_hms(0, 0, 0);
    let duration = end_of_period - start_of_period;
    println!("num days = {}", duration.num_days());
```

### 时间的时区转换


使用 offset::Local::now 获取本地时间并显示，然后使用 DateTime::from_utc 结构体方法将其转换为 UTC 标准格式。最后，使用 offset::FixedOffset 结构体，可以将 UTC 时间转换为 UTC+8 和 UTC-2。


```rs
use chrono::{DateTime, FixedOffset, Local, Utc};

fn main() {
    let local_time = Local::now();
    // 设置时间格式
    let utc_time = DateTime::<Utc>::from_utc(local_time.naive_utc(), Utc);
    // 进行时间偏移
    let china_timezone = FixedOffset::east(8 * 3600);
    println!("现在时间 {}", local_time);
    println!("UTC 时间 {}", utc_time);
    println!(
        "香港时间 {}",
        utc_time.with_timezone(&china_timezone)
    );
}

```




### 检查日期和时间
chrono-badge cat-date-and-time-badge

通过 Timelike 获取当前 UTC DateTime 及其时/分/秒，通过 Datelike 获取其年/月/日/工作日。

```rs
use chrono::{Datelike, Timelike, Utc};

fn main() {
    let now = Utc::now();

    let (is_pm, hour) = now.hour12(); //把时间转化为12小时制
    println!(
        "The current UTC time is {:02}:{:02}:{:02} {}", //设置格式
        hour,
        now.minute(),
        now.second(),
        if is_pm { "PM" } else { "AM" }
    );
    println!(
        "And there have been {} seconds since midnight",
        now.num_seconds_from_midnight() //输出到午夜的时间
    );

    let (is_common_era, year) = now.year_ce();//把时间转化为一年为单位
    println!(
        "The current UTC date is {}-{:02}-{:02} {:?} ({})",
        year,
        now.month(),
        now.day(),
        now.weekday(),
        if is_common_era { "CE" } else { "BCE" } //判断时间是公元前，还是公元后
    );
    println!(
        "And the Common Era began {} days ago", //据公元开始有多少年
        now.num_days_from_ce()
    );
}

```




### 日期和时间的格式化显示

使用 Utc::now 获取并显示当前 UTC 时间。使用 DateTime::to_rfc2822 将当前时间格式化为熟悉的 RFC 2822 格式，使用 DateTime::to_rfc3339 将当前时间格式化为熟悉的 RFC 3339 格式，也可以使用 DateTime::format 自定义时间格式。
```rs
use chrono::{DateTime, Utc};

fn main() {
    let now: DateTime<Utc> = Utc::now();

    println!("UTC now is: {}", now);
    println!("UTC now in RFC 2822 is: {}", now.to_rfc2822());
    println!("UTC now in RFC 3339 is: {}", now.to_rfc3339());
    println!("UTC now in a custom format is: {}", now.format("%a %b %e %T %Y"));
}
```
**效果**
```shell
UTC now is: 2023-12-02 13:22:23.639812500 UTC
UTC now in RFC 2822 is: Sat, 2 Dec 2023 13:22:23 +0000
UTC now in RFC 3339 is: 2023-12-02T13:22:23.639812500+00:00
UTC now in a custom format is: Sat Dec  2 13:22:23 2023
```

### 将字符串解析为 DateTime 结构体

熟悉的时间格式 RFC 2822、RFC 3339，以及自定义时间格式，通常用字符串表达。要将这些字符串解析为 DateTime 结构体，可以分别用 DateTime::parse_from_rfc2822、DateTime::parse_from_rfc3339，以及 DateTime::parse_from_str。

可以在 chrono::format::strftime 中找到适用于 DateTime::parse_from_str 的转义序列。注意：DateTime::parse_from_str 要求这些 DateTime 结构体必须是可创建的，以便它唯一地标识日期和时间。要解析不带时区的日期和时间，请使用 NaiveDate、NaiveTime，以及 NaiveDateTime。
```rs
use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime};
use chrono::format::ParseError;


fn main() -> Result<(), ParseError> {
    let rfc2822 = DateTime::parse_from_rfc2822("Tue, 1 Jul 2003 10:52:37 +0200")?;
    println!("{}", rfc2822);

    let rfc3339 = DateTime::parse_from_rfc3339("1996-12-19T16:39:57-08:00")?;
    println!("{}", rfc3339);

    let custom = DateTime::parse_from_str("5.8.1994 8:00 am +0000", "%d.%m.%Y %H:%M %P %z")?;
    println!("{}", custom);

    let time_only = NaiveTime::parse_from_str("23:56:04", "%H:%M:%S")?;
    println!("{}", time_only);

    let date_only = NaiveDate::parse_from_str("2015-09-05", "%Y-%m-%d")?;
    println!("{}", date_only);

    let no_timezone = NaiveDateTime::parse_from_str("2015-09-05 23:56:04", "%Y-%m-%d %H:%M:%S")?;
    println!("{}", no_timezone);

    Ok(())
}


```
**效果**
```shell
2003-07-01 10:52:37 +02:00
1996-12-19 16:39:57 -08:00
1994-08-05 08:00:00 +00:00
23:56:04
2015-09-05
2015-09-05 23:56:04
```

### 日期和 UNIX 时间戳的互相转换

使用 NaiveDateTime::timestamp 将由 NaiveDate::from_ymd 生成的日期和由 NaiveTime::from_hms 生成的时间转换为 UNIX 时间戳。然后，它使用 NaiveDateTime::from_timestamp 计算自 UTC 时间 1970 年 01 月 01 日 00:00:00 开始的 10 亿秒后的日期。
```rs
use chrono::{NaiveDate, NaiveDateTime};

fn main() {
    let date_time: NaiveDateTime = NaiveDate::from_ymd(2017, 11, 12).and_hms(17, 33, 44);
    println!(
        "Number of seconds between 1970-01-01 00:00:00 and {} is {}.",
        date_time, date_time.timestamp());

    let date_time_after_a_billion_seconds = NaiveDateTime::from_timestamp(1_000_000_000, 0);
    println!(
        "Date after a billion seconds since 1970-01-01 00:00:00 was {}.",
        date_time_after_a_billion_seconds);
}
```
# clap
Clap 是一个用于命令行参数解析的 Rust 库。它提供了一种简单的方式来定义命令行参数，并使用这些参数来解析命令行输入。Clap 支持多种类型的参数，包括选项、子命令、环境变量和配置文件。
Clap 提供了多种功能，包括：
1. 命令行参数的解析：Clap 可以解析命令行参数，并自动将参数转换为指定的类型。
2. 帮助信息：Clap 可以自动生成帮助信息，包括参数的描述、默认值、示例等。

## 例子


