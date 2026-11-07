use std::time::{Duration, Instant};

use avr_rust_emu::board::{Atmega328pBoard, Machine, PinMode, PinState};
use eframe::egui::{self, Color32, Pos2, Stroke, Vec2, pos2, vec2};

const MAX_FRAME: Duration = Duration::from_millis(50);
const BOARD_SIZE: Vec2 = vec2(680.0, 400.0);

// ---------------------------------------------------------------------------
// Layout: where each pin is drawn (presentation only, no hardware knowledge)
// ---------------------------------------------------------------------------

struct PinLayout {
    name: &'static str,         // matches PinState.name
    header: Vec2,               // header pad position, relative to the board
    led: Option<(Vec2, Color32)>, // LED position + color, if the pin drives one
    bend_y: f32,                // height of the trace's horizontal segment
}

fn make_layout() -> Vec<PinLayout> {
    const NAMES: [&str; 8] = ["PB0", "PB1", "PB2", "PB3", "PB4", "PB5", "PB6", "PB7"];
    let led_colors = [
        Color32::from_rgb(255, 40, 40),   // red
        Color32::from_rgb(255, 150, 20),  // orange
        Color32::from_rgb(255, 230, 40),  // yellow
        Color32::from_rgb(60, 230, 80),   // green
        Color32::from_rgb(60, 140, 255),  // blue
        Color32::from_rgb(230, 230, 255), // white
    ];

    NAMES
        .iter()
        .enumerate()
        .map(|(i, name)| {
            let led = if i < 6 {
                Some((vec2(110.0 + i as f32 * 100.0, 300.0), led_colors[i]))
            } else {
                None // PB6/PB7 are the crystal pins: header pad only
            };
            PinLayout {
                name,
                header: vec2(60.0 + i as f32 * 38.0, 54.0),
                led,
                // Right-hand traces turn earlier, so traces never cross
                bend_y: 200.0 - i as f32 * 14.0,
            }
        })
        .collect()
}

// ---------------------------------------------------------------------------
// App
// ---------------------------------------------------------------------------

struct BoardApp {
    machine: Box<dyn Machine>,
    layout: Vec<PinLayout>,
    last_frame: Instant,
}

impl BoardApp {
    fn new() -> Self {
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/examples/code.hex");
        let hex = std::fs::read_to_string(path).expect("cannot read code.hex");
        let board = Atmega328pBoard::from_hex(&hex).expect("hex load failed");
        Self {
            machine: Box::new(board),
            layout: make_layout(),
            last_frame: Instant::now(),
        }
    }
}

impl eframe::App for BoardApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        // 1. Advance the machine by the real time elapsed (capped)
        let now = Instant::now();
        let elapsed = (now - self.last_frame).min(MAX_FRAME);
        self.last_frame = now;

        let hz = self.machine.clock_hz() as f64;
        let target = self.machine.cycles() + (elapsed.as_secs_f64() * hz) as u64;
        if let Err(e) = self.machine.run_until(target) {
            eprintln!("Machine error: {e}");
        }
        ui.ctx().request_repaint();

        // 2. Draw
        egui::CentralPanel::default().show(ui, |ui| {
            ui.heading("Arduino Uno");
            let (response, painter) = ui.allocate_painter(BOARD_SIZE, egui::Sense::hover());
            let pins = self.machine.pins();
            draw_board(&painter, response.rect.min, &self.layout, &pins);
        });
    }
}

// ---------------------------------------------------------------------------
// Drawing
// ---------------------------------------------------------------------------

fn draw_board(painter: &egui::Painter, origin: Pos2, layout: &[PinLayout], pins: &[PinState]) {
    let find = |name: &str| pins.iter().find(|p| p.name == name);

    // PCB body: dark edge, then the board on top
    let board = egui::Rect::from_min_size(origin, BOARD_SIZE);
    painter.rect_filled(board, 14.0, Color32::from_rgb(0, 60, 70));
    painter.rect_filled(board.shrink(4.0), 12.0, Color32::from_rgb(0, 105, 115));

    // Ground rail
    let rail_y = 345.0;
    painter.line_segment(
        [origin + vec2(60.0, rail_y), origin + vec2(640.0, rail_y)],
        Stroke::new(4.0, Color32::from_rgb(70, 175, 165)),
    );
    painter.text(
        origin + vec2(40.0, rail_y),
        egui::Align2::RIGHT_CENTER,
        "GND",
        egui::FontId::proportional(11.0),
        Color32::from_rgb(200, 235, 230),
    );

    // Traces first (so the header strip and parts cover their ends)
    for l in layout {
        let Some((led_pos, _)) = l.led else { continue };
        let Some(pin) = find(l.name) else { continue };
        let lit = pin.mode == PinMode::Output && pin.high;

        let trace_color = if lit {
            Color32::from_rgb(255, 215, 100)
        } else {
            Color32::from_rgb(70, 175, 165)
        };
        let stroke = Stroke::new(3.0, trace_color);

        let start = origin + l.header;
        let bend1 = origin + vec2(l.header.x, l.bend_y);
        let bend2 = origin + vec2(led_pos.x, l.bend_y);
        let led_top = origin + led_pos - vec2(0.0, 12.0);

        painter.line_segment([start, bend1], stroke);
        painter.line_segment([bend1, bend2], stroke);
        painter.line_segment([bend2, led_top], stroke);

        // LED cathode down to the ground rail
        painter.line_segment(
            [origin + led_pos + vec2(0.0, 12.0), origin + vec2(led_pos.x, rail_y)],
            Stroke::new(3.0, Color32::from_rgb(70, 175, 165)),
        );
    }

    // Header strip with its pads
    painter.rect_filled(
        egui::Rect::from_min_size(origin + vec2(40.0, 38.0), vec2(306.0, 32.0)),
        4.0,
        Color32::from_rgb(25, 25, 30),
    );
    for l in layout {
        if let Some(pin) = find(l.name) {
            draw_header_pad(painter, origin + l.header, pin);
        }
    }

    // Resistors and LEDs
    for l in layout {
        let (Some((led_pos, color)), Some(pin)) = (l.led, find(l.name)) else {
            continue;
        };
        let lit = pin.mode == PinMode::Output && pin.high;

        // Resistor on the trace, halfway between the bend and the LED
        let r_center = origin + vec2(led_pos.x, led_pos.y - 50.0);
        draw_resistor(painter, r_center);

        draw_led(painter, origin + led_pos, color, lit);

        // Label to the right of the LED
        painter.text(
            origin + led_pos + vec2(20.0, 0.0),
            egui::Align2::LEFT_CENTER,
            format!("{}\n{}", pin.label, pin.name),
            egui::FontId::proportional(11.0),
            Color32::from_rgb(220, 240, 238),
        );
    }

    // A little silkscreen and a chip, for looks
    painter.text(
        origin + vec2(60.0, 372.0),
        egui::Align2::LEFT_CENTER,
        "ARDUINO UNO  -  emulated",
        egui::FontId::proportional(13.0),
        Color32::from_rgb(220, 240, 238),
    );
    let chip = egui::Rect::from_min_size(origin + vec2(500.0, 358.0), vec2(130.0, 32.0));
    painter.rect_filled(chip, 3.0, Color32::from_rgb(30, 30, 34));
    painter.text(
        chip.center(),
        egui::Align2::CENTER_CENTER,
        "ATmega328P",
        egui::FontId::proportional(11.0),
        Color32::from_rgb(170, 170, 175),
    );
}

fn draw_header_pad(painter: &egui::Painter, pos: Pos2, pin: &PinState) {
    // Gold ring, with a center that shows the pin state
    painter.circle_filled(pos, 9.0, Color32::from_rgb(212, 175, 55));
    let center = match (pin.mode, pin.high) {
        (PinMode::Output, true) => Color32::from_rgb(255, 240, 120),
        (PinMode::Output, false) => Color32::from_rgb(60, 50, 20),
        (PinMode::InputPullup, _) => Color32::from_rgb(70, 120, 200),
        (PinMode::Input, _) => Color32::from_rgb(90, 90, 90),
    };
    painter.circle_filled(pos, 5.0, center);

    painter.text(
        pos - vec2(0.0, 40.0),
        egui::Align2::CENTER_CENTER,
        &pin.name,
        egui::FontId::proportional(10.0),
        Color32::from_rgb(200, 225, 222),
    );
    painter.text(
        pos - vec2(0.0, 27.0),
        egui::Align2::CENTER_CENTER,
        &pin.label,
        egui::FontId::proportional(11.0),
        Color32::WHITE,
    );
}

fn draw_resistor(painter: &egui::Painter, center: Pos2) {
    let body = egui::Rect::from_center_size(center, vec2(12.0, 26.0));
    painter.rect_filled(body, 3.0, Color32::from_rgb(222, 196, 150));
    for dy in [-6.0, -1.0, 4.0] {
        painter.line_segment(
            [center + vec2(-6.0, dy), center + vec2(6.0, dy)],
            Stroke::new(2.0, Color32::from_rgb(120, 60, 30)),
        );
    }
}

fn draw_led(painter: &egui::Painter, center: Pos2, color: Color32, lit: bool) {
    // Soft glow when lit: three translucent circles of decreasing size
    if lit {
        for (radius, alpha) in [(30.0, 25), (22.0, 45), (16.0, 80)] {
            painter.circle_filled(
                center,
                radius,
                Color32::from_rgba_unmultiplied(color.r(), color.g(), color.b(), alpha),
            );
        }
    }

    // Body: bright when lit, dim version of its own color when off
    let body_color = if lit { color } else { color.gamma_multiply(0.25) };
    painter.circle_filled(center, 12.0, body_color);
    painter.circle_stroke(center, 12.0, Stroke::new(2.0, Color32::from_rgb(20, 20, 24)));

    // Small highlight to look like a glossy dome
    painter.circle_filled(
        center + vec2(-4.0, -4.0),
        3.0,
        Color32::from_rgba_unmultiplied(255, 255, 255, if lit { 170 } else { 60 }),
    );
}

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([720.0, 500.0]),
        ..Default::default()
    };
    eframe::run_native(
        "AVR emulator",
        options,
        Box::new(|_cc| Ok(Box::new(BoardApp::new()))),
    )
}