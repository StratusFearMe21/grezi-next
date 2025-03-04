use grezi_parser::parse::GrzFile;

struct GrzLsp {
    egui_ctx: egui::Context,
    grz_file: GrzFile,
    version: i32,
}

impl GrzLsp {
    fn run(self) {}
}
