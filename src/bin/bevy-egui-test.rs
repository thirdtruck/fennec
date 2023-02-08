use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin};
use egui::*;

struct AppState {
    example_image: Handle<bevy::prelude::Image>,
    example_image2: Handle<bevy::prelude::Image>,
}

impl FromWorld for AppState {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.get_resource_mut::<AssetServer>().unwrap();

        Self {
            example_image: asset_server.load("resources/dungeonfont.png"),
            example_image2: asset_server.load("sources/manual_pages/page01.jpg"),
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        // Systems that create Egui widgets should be run during the `CoreStage::Update` stage,
        // or after the `EguiSystem::BeginFrame` system (which belongs to the `CoreStage::PreUpdate` stage).
        .add_system(ui_example_system)
        .run();
}

fn ui_example_system(
    mut egui_ctx: ResMut<EguiContext>,
    app_state: Local<AppState>,
    mut rendered_texture_id: Local<egui::TextureId>,
    mut rendered_texture_id2: Local<egui::TextureId>,
    mut is_initialized: Local<bool>,
    mut word_size: Local<egui::Vec2>,
    mut glyph_size: Local<egui::Vec2>,
    mut stroke_width: Local<f32>,
    mut circle_radius: Local<f32>,
) {
    if !*is_initialized {
        *word_size = egui::vec2(1.0, 1.0);
        *glyph_size = egui::Vec2::new(0.45, 0.85);
        *is_initialized = true;
        *rendered_texture_id = egui_ctx.add_image(app_state.example_image.clone_weak());
        *rendered_texture_id2 = egui_ctx.add_image(app_state.example_image2.clone_weak());
    }

    egui::Window::new("Images").show(egui_ctx.ctx_mut(), |ui| {
        ui.label("images");

        ui.add(egui::widgets::Image::new(
            *rendered_texture_id,
            [256.0, 256.0],
        ));

        ui.add(egui::widgets::Image::new(
            *rendered_texture_id2,
            [256.0, 256.0],
        ));
    });

    egui::Window::new("Render Params").show(egui_ctx.ctx_mut(), |ui| {
        let word_size = &mut (*word_size);
        let glyph_size = &mut (*glyph_size);

        ui.label("Word Width");
        ui.add(egui::Slider::new(&mut word_size.x, 0.0..=10.0));

        ui.label("Word Height");
        ui.add(egui::Slider::new(&mut word_size.y, 0.0..=10.0));

        ui.label("Glyph Width");
        ui.add(egui::Slider::new(&mut glyph_size.x, 0.0..=1.0));

        ui.label("Glyph Height");
        ui.add(egui::Slider::new(&mut glyph_size.y, 0.0..=1.0));
    });

    egui::Window::new("Glyphs").show(egui_ctx.ctx_mut(), |ui| {
        let glyph_size = *glyph_size;
        let word_size = *word_size;

        let height = ui.spacing().interact_size.y;

        ui.set_row_height(height);
        ui.set_max_height(height);

        //println!("===");

        let layout = egui::Layout::left_to_right(egui::Align::Center)
            .with_main_wrap(true);

        ui.with_layout(layout, |ui| {
            ui.add(glyph(glyph_size,  egui::Color32::RED));

            ui.add(glyph(glyph_size, egui::Color32::YELLOW));

            ui.monospace("English Word");

            ui.add(glyph(glyph_size, egui::Color32::BLUE));

            ui.monospace("STAMINA-POINTS");

            ui.monospace("CHECK-POINT");

            ui.add(glyph(glyph_size, egui::Color32::YELLOW));

            ui.add(glyph(glyph_size, egui::Color32::BLUE));

            ui.monospace("HP");

            ui.add(tunic_word(word_size, glyph_size, egui::Color32::YELLOW));

            ui.add(glyph(glyph_size, egui::Color32::YELLOW));

            ui.add(glyph(glyph_size, egui::Color32::BLUE));

            ui.add(tunic_word(word_size, glyph_size, egui::Color32::RED));
        });
    });
}

fn tunic_word(word_size: egui::Vec2, glyph_size: egui::Vec2, color: egui::Color32) -> impl egui::Widget + 'static {
    move |ui: &mut egui::Ui| tunic_word_ui(ui, word_size, glyph_size, color)
}

fn tunic_word_ui(ui: &mut egui::Ui, word_size: egui::Vec2, glyph_size: egui::Vec2, color: egui::Color32) -> egui::Response {
    let glyph_count = 3;

    let desired_size = ui.spacing().interact_size.x * egui::vec2(1.0, 1.0);

    let (rect, mut response) = ui.allocate_exact_size(desired_size, egui::Sense::hover());

    response.mark_changed();

    let layout = egui::Layout::left_to_right(egui::Align::Center);

    ui.with_layout(layout, |ui| {
        for _ in 0..glyph_count {
            ui.add_sized(word_size, glyph(glyph_size, color));
        }
    });

    response
}

fn glyph(glyph_size: egui::Vec2, color: egui::Color32) -> impl egui::Widget + 'static {
    move |ui: &mut egui::Ui| glyph_ui(ui, glyph_size, color)
}

fn glyph_ui(ui: &mut egui::Ui, glyph_size: egui::Vec2, color: egui::Color32) -> egui::Response {
    let desired_size = ui.spacing().interact_size.x * egui::vec2(glyph_size.x, glyph_size.y);

    //println!("Spacing: {:?}", ui.spacing());

    let (rect, mut response) = ui.allocate_exact_size(desired_size, egui::Sense::hover());
    //println!("desired_size: {:?} -> rect: {:?}", desired_size, rect);

    response.mark_changed();

    if ui.is_rect_visible(rect) {
        let stroke = Stroke {
            width: 0.5,
            color,
        };

        let offsets = [
            ((0.5, 0.0), (0.0, 0.25)),
            ((0.5, 0.0), (1.0, 0.25)),

            ((0.5, 0.0), (0.5, 0.5)),

            ((0.5, 0.5), (0.0, 0.25)),
            ((0.5, 0.5), (1.0, 0.25)),

            ((0.0, 0.25), (0.0, 0.75)),

            ((0.5, 0.5), (0.0, 0.75)),
            ((0.5, 0.5), (1.0, 0.75)),

            ((0.5, 0.5), (0.5, 1.0)),

            ((0.5, 1.0), (0.0, 0.75)),
            ((0.5, 1.0), (1.0, 0.75)),
        ];

        let width = desired_size.x;
        let height = desired_size.y;

        let line_points = [
            Pos2 { x: rect.min.x, y: rect.min.y + (height * 0.5) },
            Pos2 { x: rect.min.x + width, y: rect.min.y + (height * 0.5) },
        ];

        ui.painter()
            .line_segment(line_points, stroke);

        for offset in offsets.iter() {
            let x1 = width * offset.0.0;
            let y1 = height * offset.0.1;
            let x2 = width * offset.1.0;
            let y2 = height * offset.1.1;

            let point_pair = [
                Pos2 { x: rect.min.x + x1, y: rect.min.y + y1 },
                Pos2 { x: rect.min.x + x2, y: rect.min.y + y2 },
            ];
            //println!("point pair {:?}", point_pair);
            ui.painter()
                .line_segment(point_pair, stroke);
        }

        let circle_x = width * 0.5;
        let circle_y = height * 0.95;
        let circle_radius = width * 0.20;

        let circle_pos = Pos2::new(circle_x + rect.min.x, circle_y + rect.min.y);

        let circle_fill = Color32::from_rgb(40, 40, 40);

        ui.painter()
            .circle(circle_pos, circle_radius, circle_fill, stroke);
    }

    response
}
