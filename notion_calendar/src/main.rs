use chrono::{Datelike, Local, NaiveDate, Weekday};
use plotters::prelude::*;
use std::error::Error;
use std::thread;
use std::time::Duration;
use plotters_backend::FontFamily;


struct Holiday {
    date: NaiveDate,
    name: String,
}

fn is_holiday(date: NaiveDate, holidays: &[Holiday]) -> Option<&str> {
    holidays.iter()
        .find(|h| h.date == date)
        .map(|h| h.name.as_str())
}

fn create_gradient_background(root: &DrawingArea<BitMapBackend, plotters::coord::Shift>) -> Result<(), Box<dyn Error>> {
    let width = 1920;
    let height = 1080;
    let steps = 1080;

    for y in 0..steps {
        let y_pos = (y * height) / steps;
        let y_next = ((y + 1) * height) / steps;
        
        let alpha = y as f64 / steps as f64;
        let r = (28.0 * (1.0 - alpha) + 18.0 * alpha) as u8;
        let g = (31.0 * (1.0 - alpha) + 18.0 * alpha) as u8;
        let b = (51.0 * (1.0 - alpha) + 26.0 * alpha) as u8;

        root.draw(&Rectangle::new(
            [(0, y_pos), (width, y_next)],
            Into::<ShapeStyle>::into(&RGBColor(r, g, b)).filled(),
        ))?;
    }
    Ok(())
}

fn draw_calendar() -> Result<(), Box<dyn Error>> {
    let today = Local::now().date_naive();
    
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

    let root = BitMapBackend::new("calendar.png", (1920, 1080)).into_drawing_area();
    create_gradient_background(&root)?;

    let title = format!("📅 {}.{:02}", today.year(), today.month());
    let font_title = FontDesc::new(FontFamily::Name("SAEEUM"), 72.0, FontStyle::Normal);
    println!("font_title: {}", font_title.get_family().as_str());
    root.draw_text(&title, &font_title.color(&RGBColor(135, 206, 235)), (60, 60))?;

    let cell_width = 258;
    let cell_height = 160;
    let grid_start_x = 60;
    let grid_start_y = 200;
    let day_font = FontDesc::new(FontFamily::Name("SAEEUM"), 36.0, FontStyle::Normal);

    let weekdays = ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];
    for (i, day) in weekdays.iter().enumerate() {
        let x = grid_start_x + (i as i32 * cell_width);
        let color = match i {
            0 => RGBColor(255, 99, 99),    // 일요일은 연한 빨간색
            6 => RGBColor(99, 149, 255),   // 토요일은 연한 파란색
            _ => RGBColor(200, 200, 200),  // 평일은 밝은 회색
        };
        root.draw_text(
            day,
            &day_font.color(&color),
            (x + 15, grid_start_y - 50),
        )?;
    }

    let first_day = NaiveDate::from_ymd_opt(today.year(), today.month(), 1).unwrap();
    let last_day = if today.month() == 12 {
        NaiveDate::from_ymd_opt(today.year() + 1, 1, 1).unwrap()
    } else {
        NaiveDate::from_ymd_opt(today.year(), today.month() + 1, 1).unwrap()
    }.pred_opt().unwrap();

    let mut current_date = first_day;
    let mut week = 0;
    let first_weekday = first_day.weekday().num_days_from_sunday() as i32;

    while current_date <= last_day {
        let day_pos_x: i32 = grid_start_x + ((first_weekday + (current_date.day() - 1) as i32) % 7 * cell_width);
        let day_pos_y: i32 = grid_start_y + (week * cell_height);

        root.draw(&Rectangle::new(
            [(day_pos_x + 1, day_pos_y + cell_height), (day_pos_x + cell_width + 1, day_pos_y + cell_height + 2)],
            Into::<ShapeStyle>::into(&RGBColor(40, 40, 40)).filled(),
        ))?;
        root.draw(&Rectangle::new(
            [(day_pos_x + cell_width, day_pos_y + 1), (day_pos_x + cell_width + 2, day_pos_y + cell_height + 1)],
            Into::<ShapeStyle>::into(&RGBColor(40, 40, 40)).filled(),
        ))?;

        root.draw(&Rectangle::new(
            [(day_pos_x, day_pos_y), (day_pos_x + cell_width, day_pos_y + cell_height)],
            Into::<ShapeStyle>::into(&RGBColor(90, 90, 90)).stroke_width(1),
        ))?;

        root.draw(&Rectangle::new(
            [(day_pos_x, day_pos_y), (day_pos_x + cell_width, day_pos_y + 1)],
            Into::<ShapeStyle>::into(&RGBColor(100, 100, 100)).filled(),
        ))?;
        root.draw(&Rectangle::new(
            [(day_pos_x, day_pos_y), (day_pos_x + 1, day_pos_y + cell_height)],
            Into::<ShapeStyle>::into(&RGBColor(100, 100, 100)).filled(),
        ))?;

        let holiday_name = is_holiday(current_date, &holidays);
        let is_today = current_date == today;

        if is_today {
            root.draw(&Rectangle::new(
                [(day_pos_x + 2, day_pos_y + 2), (day_pos_x + cell_width - 2, day_pos_y + cell_height - 2)],
                Into::<ShapeStyle>::into(&RGBColor(70, 40, 60)).filled(),
            ))?;
        }

        let date_color = if is_today {
            RGBColor(255, 182, 193)
        } else if holiday_name.is_some() || current_date.weekday() == Weekday::Sun {
            RGBColor(255, 99, 99)
        } else if current_date.weekday() == Weekday::Sat {
            RGBColor(99, 149, 255)
        } else {
            RGBColor(200, 200, 200)
        };

        root.draw_text(
            &current_date.day().to_string(),
            &day_font.color(&date_color),
            (day_pos_x + 10, day_pos_y + 10),
        )?;

        if let Some(holiday) = holiday_name {
            let holiday_font = FontDesc::new(FontFamily::Name("SAEEUM"), 24.0, FontStyle::Normal);
            root.draw_text(
                holiday,
                &holiday_font.color(&RGBColor(255, 99, 99)),
                (day_pos_x + 20, day_pos_y + 70),
            )?;
        }

        if (current_date.day() as i32 + first_weekday) % 7 == 0 {
            week += 1;
        }
        current_date = current_date.succ_opt().unwrap();
    }

    root.present()?;
    println!("✅ calendar.png 생성 완료! ({})", Local::now().format("%Y-%m-%d %H:%M:%S"));
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    // assets 디렉토리에서 폰트 로드
    let font_data = include_bytes!("../assets/SAEEUM.otf");
    
    match plotters::style::register_font("SAEEUM", FontStyle::Normal, font_data) {
        Ok(_) => println!("✅ SAEEUM 폰트 등록 성공"),
        Err(_) => {
            println!("❌ SAEEUM 폰트 등록 실패");
            // 폰트 등록 실패 시 기본 폰트 사용
            return Err("폰트 등록 실패".into());
        }
    }
    
    loop {
        if let Err(e) = draw_calendar() {
            eprintln!("캘린더 업데이트 중 오류 발생: {}", e);
        }
        thread::sleep(Duration::from_secs(600));
    }
}