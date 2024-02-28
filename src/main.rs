mod file;
mod form;

use std::borrow::Cow;
use eframe::{NativeOptions, run_native};
use eframe::egui::{CentralPanel, CtxRef, FontDefinitions, FontFamily, ScrollArea, Vec2};
use eframe::epi::{App, Frame, Storage};

const WIDTH: f32 = 890.0;
const HEIGHT: f32 = 1200.0;
const FONT: &str = "JetBrainsMono";
const H_FONT_SIZE: f32 = 30.0;
const B_FONT_SIZE: f32 = 19.0;


#[derive(Default)]
struct Hex;

impl Hex
{
   fn new() -> Hex
   {
      return Hex::default();
   }
   fn font_config(&self, ctx: &CtxRef)
   {
      let mut font_def = FontDefinitions::default();
      font_def.font_data.insert(FONT.to_string(), Cow::Borrowed(include_bytes!("../res/font/JetBrainsMono-Bold.ttf")));
      font_def.family_and_size.insert(eframe::egui::TextStyle::Heading, (FontFamily::Proportional, H_FONT_SIZE), );
      font_def.family_and_size.insert(eframe::egui::TextStyle::Body, (FontFamily::Proportional, B_FONT_SIZE), );
      font_def.fonts_for_family.get_mut(&FontFamily::Proportional).unwrap().insert(0, FONT.to_string());
      ctx.set_fonts(font_def);
   }
}
impl App for Hex
{
   fn setup(&mut self, ctx: &CtxRef, _frame: &mut Frame<'_>, _storage: Option<&dyn Storage>)
   {
      self.font_config(ctx);
   }
   fn update(&mut self, ctx: &CtxRef, _frame: &mut Frame<'_>)
   {
      CentralPanel::default().show(ctx, |ui|
         {
            ScrollArea::auto_sized().show(ui, |ui|
            {
               ui.label(file::gen_mem());
            })
         });
   }
   fn name(&self) -> &str { "Hex Editor" }
}
fn main()
{
   let app = Hex::new();
   let mut win_option = NativeOptions::default();
   win_option.initial_window_size = Some(Vec2::new(HEIGHT, WIDTH));
   win_option.resizable = false;
   run_native(Box::new(app), win_option);
}
