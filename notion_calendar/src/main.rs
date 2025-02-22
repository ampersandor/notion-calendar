use chrono::{Datelike, Local, NaiveDate, Timelike, Weekday, FixedOffset};
use image::{ImageBuffer, Rgb, RgbImage};
use rusttype::{point, Font, Scale};
use std::error::Error;
use std::thread;
use std::time::Duration;

struct Holiday {
    date: NaiveDate,
    name: String,
}

#[derive(Clone, Copy)]
enum Theme {
    Dark,
    Light,
}

struct ThemeColors {
    background_start: [u8; 3],
    background_end: [u8; 3],
    title: [u8; 3],
    border: [u8; 3],
    today_bg: [u8; 3],
    text_normal: [u8; 3],
    text_sunday: [u8; 3],
    text_saturday: [u8; 3],
    text_today: [u8; 3],
    text_holiday: [u8; 3],
}

impl Theme {
    fn colors(&self) -> ThemeColors {
        match self {
            Theme::Dark => ThemeColors {
                background_start: [28, 31, 51],
                background_end: [18, 18, 26],
                title: [135, 206, 235],
                border: [90, 90, 90],
                today_bg: [70, 40, 60],
                text_normal: [200, 200, 200],
                text_sunday: [255, 99, 99],
                text_saturday: [99, 149, 255],
                text_today: [255, 182, 193],
                text_holiday: [255, 99, 99],
            },
            Theme::Light => ThemeColors {
                background_start: [245, 250, 255],    // 위쪽은 거의 흰색
                background_end: [200, 225, 255],      // 아래쪽은 연한 하늘색
                title: [70, 130, 180],
                border: [200, 200, 200],
                today_bg: [255, 240, 245],
                text_normal: [60, 60, 60],
                text_sunday: [220, 50, 50],
                text_saturday: [50, 100, 220],
                text_today: [220, 50, 150],
                text_holiday: [220, 50, 50],
            },
        }
    }
}

fn draw_text(
    image: &mut RgbImage,
    font: &Font,
    text: &str,
    x: i32,
    y: i32,
    size: f32,
    color: Rgb<u8>,
) {
    let scale = Scale::uniform(size);
    let v_metrics = font.v_metrics(scale);
    let offset = point(x as f32, y as f32 + v_metrics.ascent);

    // 텍스트를 이미지에 그리기
    for glyph in font.layout(text, scale, offset) {
        if let Some(bounding_box) = glyph.pixel_bounding_box() {
            glyph.draw(|x, y, v| {
                let x = x as i32 + bounding_box.min.x;
                let y = y as i32 + bounding_box.min.y;
                
                if x >= 0 && x < image.width() as i32 && y >= 0 && y < image.height() as i32 {
                    let pixel = image.get_pixel_mut(x as u32, y as u32);
                    *pixel = Rgb([
                        ((1.0 - v) * pixel[0] as f32 + v * color[0] as f32) as u8,
                        ((1.0 - v) * pixel[1] as f32 + v * color[1] as f32) as u8,
                        ((1.0 - v) * pixel[2] as f32 + v * color[2] as f32) as u8,
                    ]);
                }
            });
        }
    }
}

fn draw_calendar(theme: Theme) -> Result<(), Box<dyn Error>> {
    let colors = theme.colors();
    let mut img = ImageBuffer::new(1920, 1080);
    let kst = FixedOffset::east_opt(9 * 3600).unwrap();

    // 그라데이션 배경
    for y in 0..1080 {
        let alpha = y as f64 / 1080.0;
        let r = (colors.background_start[0] as f64 * (1.0 - alpha) + colors.background_end[0] as f64 * alpha) as u8;
        let g = (colors.background_start[1] as f64 * (1.0 - alpha) + colors.background_end[1] as f64 * alpha) as u8;
        let b = (colors.background_start[2] as f64 * (1.0 - alpha) + colors.background_end[2] as f64 * alpha) as u8;
        
        for x in 0..1920 {
            img.put_pixel(x, y, Rgb([r, g, b]));
        }
    }

    // 폰트 로드
    let font_data = include_bytes!("../assets/BinggraeSamanco.otf");
    let font = Font::try_from_vec(font_data.to_vec())
        .ok_or("Error loading font")?;

    // 배경 생성 후...
    let today = Local::now().with_timezone(&kst).date_naive();
    let title = format!("{}년 {:02}월 {:02}일!", today.year(), today.month(), today.day());
    draw_text(&mut img, &font, &title, 60, 60, 72.0, Rgb([colors.title[0], colors.title[1], colors.title[2]]));

    // 제목 그린 후...
    let weekdays = ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];
    for (i, day) in weekdays.iter().enumerate() {
        let x = 60 + (i as i32 * 258);  // 258은 셀 너비
        let color = match i {
            0 => Rgb([colors.text_sunday[0], colors.text_sunday[1], colors.text_sunday[2]]),    // 일요일은 빨간색
            6 => Rgb([colors.text_saturday[0], colors.text_saturday[1], colors.text_saturday[2]]),   // 토요일은 파란색
            _ => Rgb([colors.text_normal[0], colors.text_normal[1], colors.text_normal[2]]),  // 평일은 회색
        };
        draw_text(&mut img, &font, day, x + 15, 150, 36.0, color);
    }

    // Holiday 데이터 추가
    let holidays = vec![
        Holiday {
            date: NaiveDate::from_ymd_opt(2024, 2, 9).unwrap(),
            name: "설날".to_string(),
        },
        Holiday {
            date: NaiveDate::from_ymd_opt(2024, 2, 10).unwrap(),
            name: "설날".to_string(),
        },
        Holiday {
            date: NaiveDate::from_ymd_opt(2024, 3, 1).unwrap(),
            name: "삼일절".to_string(),
        },
    ];

    // 요일 그린 후...
    let first_day = NaiveDate::from_ymd_opt(today.year(), today.month(), 1).unwrap();
    let last_day = if today.month() == 12 {
        NaiveDate::from_ymd_opt(today.year() + 1, 1, 1).unwrap()
    } else {
        NaiveDate::from_ymd_opt(today.year(), today.month() + 1, 1).unwrap()
    }.pred_opt().unwrap();

    let mut current_date: NaiveDate = first_day;
    let mut week = 0;
    let first_weekday = first_day.weekday().num_days_from_sunday() as i32;

    // 날짜 그리기
    while current_date <= last_day {
        let day_pos_x = 60 + ((first_weekday + (current_date.day() - 1) as i32) % 7 * 258);
        let day_pos_y = 200 + (week * 160);  // 160은 셀 높이

        // 오늘 날짜 배경색 추가
        if current_date == today {
            for x in 1..257 {
                for y in 1..159 {
                    img.put_pixel(
                        (day_pos_x + x) as u32,
                        (day_pos_y + y) as u32,
                        Rgb([colors.today_bg[0], colors.today_bg[1], colors.today_bg[2]])  // 어두운 보라색 배경
                    );
                }
            }
        }

        // 테두리 그리기 (회색)
        for x in 0..258 {
            for y in 0..160 {
                if x == 0 || x == 257 || y == 0 || y == 159 {
                    img.put_pixel(
                        (day_pos_x + x) as u32,
                        (day_pos_y + y) as u32,
                        Rgb([colors.border[0], colors.border[1], colors.border[2]])
                    );
                }
            }
        }

        // 날짜 색상 설정
        let color = if current_date == today {
            Rgb([colors.text_today[0], colors.text_today[1], colors.text_today[2]])  // 오늘 날짜는 분홍색
        } else if current_date.weekday() == Weekday::Sun {
            Rgb([colors.text_sunday[0], colors.text_sunday[1], colors.text_sunday[2]])    // 일요일은 빨간색
        } else if current_date.weekday() == Weekday::Sat {
            Rgb([colors.text_saturday[0], colors.text_saturday[1], colors.text_saturday[2]])   // 토요일은 파란색
        } else {
            Rgb([colors.text_normal[0], colors.text_normal[1], colors.text_normal[2]])  // 평일은 회색
        };

        // 날짜 그리기
        draw_text(&mut img, &font, &current_date.day().to_string(),
            day_pos_x + 10, day_pos_y + 10, 36.0, color);

        // 공휴일 표시 추가
        if let Some(holiday) = holidays.iter().find(|h| h.date == current_date) {
            draw_text(&mut img, &font, &holiday.name,
                day_pos_x + 20, day_pos_y + 70, 24.0, Rgb([colors.text_holiday[0], colors.text_holiday[1], colors.text_holiday[2]]));
        }

        if (current_date.day() as i32 + first_weekday) % 7 == 0 {
            week += 1;
        }
        current_date = current_date.succ_opt().unwrap();
    }

    // 한국 시간대 설정 (UTC+9)
    let kst = FixedOffset::east_opt(9 * 3600).unwrap();
    let now = Local::now().with_timezone(&kst);
    
    // 마지막 업데이트 시간 표시
    let update_time = format!("updated: {}", now.format("%Y-%m-%d %H:%M:%S"));
    draw_text(&mut img, &font, &update_time, 
        1920 - 280, 1080 - 60, 24.0,
        Rgb([colors.text_normal[0], colors.text_normal[1], colors.text_normal[2]]));

    // 이미지 저장
    img.save("calendar.png")?;
    println!("✅ calendar.png 생성 완료! ({})", Local::now().format("%Y-%m-%d %H:%M:%S"));
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    loop {
        // 한국 시간 기준으로 테마 변경
        let kst = FixedOffset::east_opt(9 * 3600).unwrap();
        let now = Local::now().with_timezone(&kst);
        let hour = now.hour();
        
        let theme = if hour >= 6 && hour < 23 {
            Theme::Light  // 오전 6시 ~ 오후 11시는 라이트 모드
        } else {
            Theme::Dark   // 그 외 시간은 다크 모드
        };

        if let Err(e) = draw_calendar(theme) {
            eprintln!("캘린더 업데이트 중 오류 발생: {}", e);
        }
        thread::sleep(Duration::from_secs(600));
    }
}