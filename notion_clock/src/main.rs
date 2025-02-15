use eframe::egui;
use chrono::Local;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([300.0, 150.0])  // 시계 크기를 좀 더 작게 조정
            .with_decorations(false)
            .with_transparent(true)
            .with_position([1366.0 - 320.0, 10.0])  // 1366x768 해상도 기준으로 위치 조정
            .with_always_on_top()  // 인자 없이 호출
            .with_window_level(egui::WindowLevel::AlwaysOnTop),  // 이 방법도 가능
        ..Default::default()
    };
    
    eframe::run_native(
        "Clock",
        options,
        Box::new(|cc| {
            // 기본 폰트 사용
            let mut fonts = egui::FontDefinitions::default();
            
            // 기본 폰트 크기 조정
            fonts.families
                .get_mut(&egui::FontFamily::Proportional)
                .unwrap()
                .insert(0, "Roboto-Regular".to_owned());
                
            cc.egui_ctx.set_fonts(fonts);
            
            Box::new(ClockApp::default())
        }),
    )
}

struct ClockApp {
    background_color: egui::Color32,
}

impl Default for ClockApp {
    fn default() -> Self {
        Self {
            background_color: egui::Color32::from_rgba_premultiplied(0, 0, 0, 200),
        }
    }
}

impl eframe::App for ClockApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let now = Local::now();
        
        egui::CentralPanel::default()
            .frame(egui::Frame::none().fill(self.background_color))
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(15.0);  // 상단 여백 조정
                    
                    // 시간 표시 (폰트 크기 조정)
                    ui.heading(
                        egui::RichText::new(now.format("%H:%M:%S").to_string())
                            .size(54.0)  // 폰트 크기 조정
                            .color(egui::Color32::from_rgb(135, 206, 235))
                    );
                    
                    // 날짜 표시 (폰트 크기 조정)
                    ui.label(
                        egui::RichText::new(now.format("%Y-%m-%d").to_string())
                            .size(18.0)  // 폰트 크기 조정
                            .color(egui::Color32::from_rgb(200, 200, 200))
                    );
                });
            });
            
        // 1초마다 화면 갱신
        ctx.request_repaint_after(std::time::Duration::from_secs(1));
    }
} 