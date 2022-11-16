use eframe::egui;
struct Bmi {
    weight: String,
    height: String,
    tf:bool,
    bmi:String,
}
impl eframe::App for Bmi {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::Window::new("BMI").show(ctx, |ui| {
            ui.label("BMI calculater");
            ui.label("Enter your weight in kg");
            ui.text_edit_singleline(&mut self.weight);
            ui.label("Enter your height in centimeteres");
            ui.text_edit_singleline(&mut self.height);
            if self.tf{
                ui.label(&self.bmi);
            }
            if ui.button("Check BMI").clicked(){
                let weight:f64= self.weight.trim().parse().unwrap();
                let height:f64= self.height.trim().parse().unwrap();
                let height=height/100.0;
                let height = height * height;
                let bmi:f64 = weight / height;
                self.bmi=bmi.to_string();
                self.tf=true
            }
        });
    }
}
fn main(){
    let options = eframe::NativeOptions {
        transparent: true,
        ..Default::default()
    };
    eframe::run_native(
        "BMI calculater",
        options,
        Box::new(|_| {
            Box::new(Bmi {
                height: "".to_string(),
                weight: "".to_string(),
                tf:false,
                bmi:"".to_string(),
            })
        }),
    )
}