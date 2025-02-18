use chrono::{Datelike, Local, NaiveDate, Weekday};
use image::{ImageBuffer, Rgb, RgbImage};
use rusttype::{point, Font, Point, Scale};
use std::error::Error;

struct Holiday {
    date: NaiveDate,
    name: String,
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

fn main() -> Result<(), Box<dyn Error>> {
    // 1920x1080 검은 배경의 이미지 생성
    let mut img = ImageBuffer::new(1920, 1080);
    
    // 폰트 로드
    let font_data = include_bytes!("../assets/BinggraeSamanco.otf");
    let font = Font::try_from_vec(font_data.to_vec())
        .ok_or("Error loading font")?;

    // 그라데이션 배경 생성
    for y in 0..1080 {
        let alpha = y as f64 / 1080.0;
        let r = (28.0 * (1.0 - alpha) + 18.0 * alpha) as u8;
        let g = (31.0 * (1.0 - alpha) + 18.0 * alpha) as u8;
        let b = (51.0 * (1.0 - alpha) + 26.0 * alpha) as u8;
        
        for x in 0..1920 {
            img.put_pixel(x, y, Rgb([r, g, b]));
        }
    }

    // 배경 생성 후...
    let today = Local::now().date_naive();
    let title = format!("{}년 {:02}월 {:02}일!", today.year(), today.month(), today.day());
    draw_text(&mut img, &font, &title, 60, 60, 72.0, Rgb([135, 206, 235]));

    // 제목 그린 후...
    let weekdays = ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];
    for (i, day) in weekdays.iter().enumerate() {
        let x = 60 + (i as i32 * 258);  // 258은 셀 너비
        let color = match i {
            0 => Rgb([255, 99, 99]),    // 일요일은 빨간색
            6 => Rgb([99, 149, 255]),   // 토요일은 파란색
            _ => Rgb([200, 200, 200]),  // 평일은 회색
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
    let today = Local::now().date_naive();
    let first_day = NaiveDate::from_ymd_opt(today.year(), today.month(), 1).unwrap();
    let last_day = if today.month() == 12 {
        NaiveDate::from_ymd_opt(today.year() + 1, 1, 1).unwrap()
    } else {
        NaiveDate::from_ymd_opt(today.year(), today.month() + 1, 1).unwrap()
    }.pred_opt().unwrap();

    let mut current_date = first_day;
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
                        Rgb([70, 40, 60])  // 어두운 보라색 배경
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
                        Rgb([90, 90, 90])
                    );
                }
            }
        }

        // 날짜 색상 설정
        let color = if current_date == today {
            Rgb([255, 182, 193])  // 오늘 날짜는 분홍색
        } else if current_date.weekday() == Weekday::Sun {
            Rgb([255, 99, 99])    // 일요일은 빨간색
        } else if current_date.weekday() == Weekday::Sat {
            Rgb([99, 149, 255])   // 토요일은 파란색
        } else {
            Rgb([200, 200, 200])  // 평일은 회색
        };

        // 날짜 그리기
        draw_text(&mut img, &font, &current_date.day().to_string(),
            day_pos_x + 10, day_pos_y + 10, 36.0, color);

        // 공휴일 표시 추가
        if let Some(holiday) = holidays.iter().find(|h| h.date == current_date) {
            draw_text(&mut img, &font, &holiday.name,
                day_pos_x + 20, day_pos_y + 70, 24.0, Rgb([255, 99, 99]));
        }

        if (current_date.day() as i32 + first_weekday) % 7 == 0 {
            week += 1;
        }
        current_date = current_date.succ_opt().unwrap();
    }

    // 이미지 저장
    img.save("calendar.png")?;
    println!("✅ calendar.png 생성 완료!");
    Ok(())
}