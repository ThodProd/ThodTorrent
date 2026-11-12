use super::*;
use egui::{
    Align, Align2, Color32 as Color, FontFamily, FontId, Margin, Pos2, Rect, RichText as Text,
    Sense, Stroke, StrokeKind, Vec2,
};

const BOLD: &str = "Bold";
const ROW_H: f32 = 58.;
const HEAD_H: f32 = 34.;

#[derive(Clone, Copy)]
struct Palette {
    dark: bool,
    bg: Color,
    side: Color,
    surface: Color,
    field: Color,
    hover: Color,
    text: Color,
    muted: Color,
    line: Color,
    blue: Color,
    blue_fill: Color,
    selected: Color,
    green: Color,
    red: Color,
}

fn rgb(r: u8, g: u8, b: u8) -> Color {
    Color::from_rgb(r, g, b)
}

impl Palette {
    fn new(dark: bool) -> Self {
        if dark {
            Self {
                dark,
                bg: rgb(28, 29, 32),
                side: rgb(23, 24, 27),
                surface: rgb(37, 38, 42),
                field: rgb(42, 43, 48),
                hover: rgb(47, 48, 54),
                text: rgb(236, 237, 240),
                muted: rgb(152, 156, 165),
                line: rgb(48, 49, 54),
                blue: rgb(129, 170, 255),
                blue_fill: rgb(47, 111, 228),
                selected: rgb(35, 49, 74),
                green: rgb(107, 193, 153),
                red: rgb(224, 106, 117),
            }
        } else {
            Self {
                dark,
                bg: Color::WHITE,
                side: rgb(245, 245, 247),
                surface: rgb(247, 248, 250),
                field: rgb(243, 244, 246),
                hover: rgb(240, 241, 244),
                text: rgb(36, 38, 43),
                muted: rgb(118, 123, 133),
                line: rgb(232, 233, 237),
                blue: rgb(54, 118, 232),
                blue_fill: rgb(54, 118, 232),
                selected: rgb(234, 242, 254),
                green: rgb(42, 140, 105),
                red: rgb(185, 47, 58),
            }
        }
    }
}

fn tint(c: Color, a: u8) -> Color {
    Color::from_rgba_unmultiplied(c.r(), c.g(), c.b(), a)
}

fn bold(size: f32) -> FontId {
    FontId::new(size, FontFamily::Name(BOLD.into()))
}

#[derive(Clone, Copy, PartialEq)]
enum Ico {
    Menu,
    Download,
    Check,
    Pause,
    Play,
    Plus,
    Search,
    File,
    Settings,
    Close,
    Folder,
    Trash,
    Info,
    Alert,
    Sun,
    Moon,
    External,
}

fn icon(ui: &mut egui::Ui, kind: Ico, color: Color, size: f32) {
    let (rect, _) = ui.allocate_exact_size(Vec2::splat(size), Sense::hover());
    draw_icon(ui.painter(), rect, kind, color);
}

fn draw_icon(p: &egui::Painter, rect: Rect, kind: Ico, color: Color) {
    let c = rect.center();
    let s = rect.width() * 0.5;
    let w = (rect.width() * 0.085).clamp(1.15, 2.0);
    let st = Stroke::new(w, color);
    match kind {
        Ico::Menu => {
            for (dy, k) in [(-0.46, 0.72), (0.0, 0.86), (0.46, 0.58)] {
                p.line_segment(
                    [
                        Pos2::new(c.x - s * k, c.y + s * dy),
                        Pos2::new(c.x + s * k, c.y + s * dy),
                    ],
                    st,
                );
            }
        }
        Ico::Download => {
            p.line_segment(
                [
                    Pos2::new(c.x, c.y - s * 0.82),
                    Pos2::new(c.x, c.y + s * 0.28),
                ],
                st,
            );
            p.line_segment(
                [
                    Pos2::new(c.x - s * 0.42, c.y - s * 0.12),
                    Pos2::new(c.x, c.y + s * 0.30),
                ],
                st,
            );
            p.line_segment(
                [
                    Pos2::new(c.x + s * 0.42, c.y - s * 0.12),
                    Pos2::new(c.x, c.y + s * 0.30),
                ],
                st,
            );
            p.line_segment(
                [
                    Pos2::new(c.x - s * 0.62, c.y + s * 0.64),
                    Pos2::new(c.x + s * 0.62, c.y + s * 0.64),
                ],
                st,
            );
        }
        Ico::Check => {
            p.line_segment(
                [
                    Pos2::new(c.x - s * 0.62, c.y + s * 0.02),
                    Pos2::new(c.x - s * 0.16, c.y + s * 0.46),
                ],
                st,
            );
            p.line_segment(
                [
                    Pos2::new(c.x - s * 0.16, c.y + s * 0.46),
                    Pos2::new(c.x + s * 0.64, c.y - s * 0.50),
                ],
                st,
            );
        }
        Ico::Pause => {
            for dx in [-0.28, 0.28] {
                p.line_segment(
                    [
                        Pos2::new(c.x + s * dx, c.y - s * 0.52),
                        Pos2::new(c.x + s * dx, c.y + s * 0.52),
                    ],
                    Stroke::new(w * 1.45, color),
                );
            }
        }
        Ico::Play => {
            p.line_segment(
                [
                    Pos2::new(c.x - s * 0.34, c.y - s * 0.55),
                    Pos2::new(c.x + s * 0.58, c.y),
                ],
                st,
            );
            p.line_segment(
                [
                    Pos2::new(c.x + s * 0.58, c.y),
                    Pos2::new(c.x - s * 0.34, c.y + s * 0.55),
                ],
                st,
            );
            p.line_segment(
                [
                    Pos2::new(c.x - s * 0.34, c.y + s * 0.55),
                    Pos2::new(c.x - s * 0.34, c.y - s * 0.55),
                ],
                st,
            );
        }
        Ico::Plus => {
            p.line_segment(
                [
                    Pos2::new(c.x - s * 0.56, c.y),
                    Pos2::new(c.x + s * 0.56, c.y),
                ],
                st,
            );
            p.line_segment(
                [
                    Pos2::new(c.x, c.y - s * 0.56),
                    Pos2::new(c.x, c.y + s * 0.56),
                ],
                st,
            );
        }
        Ico::Search => {
            p.circle_stroke(c + Vec2::new(-s * 0.10, -s * 0.10), s * 0.56, st);
            p.line_segment(
                [
                    Pos2::new(c.x + s * 0.32, c.y + s * 0.32),
                    Pos2::new(c.x + s * 0.74, c.y + s * 0.74),
                ],
                st,
            );
        }
        Ico::File => {
            let r = Rect::from_center_size(c, Vec2::new(s * 1.18, s * 1.52));
            p.rect_stroke(r, 2.5, st, StrokeKind::Inside);
            p.line_segment(
                [
                    Pos2::new(r.left() + s * 0.22, r.top() + s * 0.48),
                    Pos2::new(r.right() - s * 0.22, r.top() + s * 0.48),
                ],
                Stroke::new(w * 0.85, color),
            );
            p.line_segment(
                [
                    Pos2::new(r.left() + s * 0.22, r.top() + s * 0.82),
                    Pos2::new(r.right() - s * 0.40, r.top() + s * 0.82),
                ],
                Stroke::new(w * 0.85, color),
            );
        }
        Ico::Settings => {
            p.circle_stroke(c, s * 0.32, st);
            p.circle_filled(c, s * 0.10, color);
            for i in 0..8 {
                let a = std::f32::consts::TAU * i as f32 / 8.0;
                let d = Vec2::new(a.cos(), a.sin());
                p.line_segment([c + d * s * 0.55, c + d * s * 0.80], st);
            }
        }
        Ico::Close => {
            p.line_segment(
                [
                    Pos2::new(c.x - s * 0.42, c.y - s * 0.42),
                    Pos2::new(c.x + s * 0.42, c.y + s * 0.42),
                ],
                st,
            );
            p.line_segment(
                [
                    Pos2::new(c.x + s * 0.42, c.y - s * 0.42),
                    Pos2::new(c.x - s * 0.42, c.y + s * 0.42),
                ],
                st,
            );
        }
        Ico::Folder => {
            let pts = [
                Pos2::new(c.x - s * 0.72, c.y - s * 0.42),
                Pos2::new(c.x - s * 0.26, c.y - s * 0.42),
                Pos2::new(c.x - s * 0.10, c.y - s * 0.20),
                Pos2::new(c.x + s * 0.72, c.y - s * 0.20),
                Pos2::new(c.x + s * 0.72, c.y + s * 0.55),
                Pos2::new(c.x - s * 0.72, c.y + s * 0.55),
                Pos2::new(c.x - s * 0.72, c.y - s * 0.42),
            ];
            for pair in pts.windows(2) {
                p.line_segment([pair[0], pair[1]], st);
            }
        }
        Ico::Trash => {
            p.line_segment(
                [
                    Pos2::new(c.x - s * 0.58, c.y - s * 0.52),
                    Pos2::new(c.x + s * 0.58, c.y - s * 0.52),
                ],
                st,
            );
            p.line_segment(
                [
                    Pos2::new(c.x - s * 0.22, c.y - s * 0.52),
                    Pos2::new(c.x - s * 0.16, c.y - s * 0.72),
                ],
                st,
            );
            p.line_segment(
                [
                    Pos2::new(c.x + s * 0.22, c.y - s * 0.52),
                    Pos2::new(c.x + s * 0.16, c.y - s * 0.72),
                ],
                st,
            );
            p.line_segment(
                [
                    Pos2::new(c.x - s * 0.44, c.y - s * 0.30),
                    Pos2::new(c.x - s * 0.34, c.y + s * 0.66),
                ],
                st,
            );
            p.line_segment(
                [
                    Pos2::new(c.x + s * 0.44, c.y - s * 0.30),
                    Pos2::new(c.x + s * 0.34, c.y + s * 0.66),
                ],
                st,
            );
            p.line_segment(
                [
                    Pos2::new(c.x - s * 0.34, c.y + s * 0.66),
                    Pos2::new(c.x + s * 0.34, c.y + s * 0.66),
                ],
                st,
            );
        }
        Ico::Info => {
            p.circle_stroke(c, s * 0.70, st);
            p.circle_filled(c + Vec2::new(0., -s * 0.30), s * 0.09, color);
            p.line_segment(
                [
                    Pos2::new(c.x, c.y - s * 0.02),
                    Pos2::new(c.x, c.y + s * 0.36),
                ],
                st,
            );
        }
        Ico::Alert => {
            p.circle_stroke(c, s * 0.70, st);
            p.line_segment(
                [
                    Pos2::new(c.x, c.y - s * 0.34),
                    Pos2::new(c.x, c.y + s * 0.12),
                ],
                st,
            );
            p.circle_filled(c + Vec2::new(0., s * 0.36), s * 0.09, color);
        }
        Ico::Sun => {
            p.circle_stroke(c, s * 0.34, st);
            for i in 0..8 {
                let a = std::f32::consts::TAU * i as f32 / 8.0;
                let d = Vec2::new(a.cos(), a.sin());
                p.line_segment([c + d * s * 0.56, c + d * s * 0.82], st);
            }
        }
        Ico::Moon => {
            p.circle_stroke(c, s * 0.62, st);
            p.circle_stroke(
                c + Vec2::new(s * 0.42, -s * 0.22),
                s * 0.52,
                Stroke::new(w, tint(color, 0)),
            );
        }
        Ico::External => {
            p.line_segment(
                [
                    Pos2::new(c.x - s * 0.34, c.y - s * 0.54),
                    Pos2::new(c.x - s * 0.62, c.y - s * 0.54),
                ],
                st,
            );
            p.line_segment(
                [
                    Pos2::new(c.x - s * 0.62, c.y - s * 0.54),
                    Pos2::new(c.x - s * 0.62, c.y + s * 0.54),
                ],
                st,
            );
            p.line_segment(
                [
                    Pos2::new(c.x - s * 0.62, c.y + s * 0.54),
                    Pos2::new(c.x + s * 0.46, c.y + s * 0.54),
                ],
                st,
            );
            p.line_segment(
                [
                    Pos2::new(c.x - s * 0.04, c.y + s * 0.06),
                    Pos2::new(c.x + s * 0.62, c.y - s * 0.58),
                ],
                st,
            );
            p.line_segment(
                [
                    Pos2::new(c.x + s * 0.16, c.y - s * 0.58),
                    Pos2::new(c.x + s * 0.62, c.y - s * 0.58),
                ],
                st,
            );
            p.line_segment(
                [
                    Pos2::new(c.x + s * 0.62, c.y - s * 0.58),
                    Pos2::new(c.x + s * 0.62, c.y - s * 0.12),
                ],
                st,
            );
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Btn {
    Primary,
    Secondary,
    Ghost,
    Danger,
}

fn button(
    ui: &mut egui::Ui,
    label: &str,
    ico: Option<Ico>,
    kind: Btn,
    p: Palette,
    enabled: bool,
) -> egui::Response {
    let font = FontId::proportional(13.);
    let galley = ui
        .painter()
        .layout_no_wrap(label.to_owned(), font.clone(), p.text);
    let icon_w = if ico.is_some() { 22. } else { 0. };
    let width = (galley.size().x + 24. + icon_w).ceil();
    let sense = if enabled {
        Sense::click()
    } else {
        Sense::hover()
    };
    let (rect, resp) = ui.allocate_exact_size(Vec2::new(width, 32.), sense);
    let hovered = enabled && resp.hovered();
    let (bg, fg, border) = match (kind, enabled) {
        (Btn::Primary, true) => (
            if hovered {
                if p.dark {
                    p.blue_fill.gamma_multiply(1.12)
                } else {
                    p.blue_fill.gamma_multiply(0.92)
                }
            } else {
                p.blue_fill
            },
            Color::WHITE,
            Stroke::NONE,
        ),
        (Btn::Primary, false) => (p.surface, p.muted, Stroke::new(1.0f32, p.line)),
        (Btn::Secondary, true) => (
            if hovered { p.hover } else { p.surface },
            p.text,
            Stroke::new(1.0f32, p.line),
        ),
        (Btn::Secondary, false) => (p.surface, p.muted, Stroke::new(1.0f32, p.line)),
        (Btn::Ghost, true) => (
            if hovered { p.hover } else { Color::TRANSPARENT },
            p.text,
            Stroke::NONE,
        ),
        (Btn::Ghost, false) => (Color::TRANSPARENT, p.muted, Stroke::NONE),
        (Btn::Danger, true) => (
            if hovered {
                p.red.gamma_multiply(1.08)
            } else {
                p.red
            },
            Color::WHITE,
            Stroke::NONE,
        ),
        (Btn::Danger, false) => (p.surface, p.muted, Stroke::new(1.0f32, p.line)),
    };
    if bg != Color::TRANSPARENT {
        ui.painter().rect_filled(rect, 8., bg);
    }
    if border != Stroke::NONE {
        ui.painter()
            .rect_stroke(rect, 8., border, StrokeKind::Inside);
    }
    if enabled && resp.has_focus() {
        ui.painter().rect_stroke(
            rect.expand(2.),
            10.,
            Stroke::new(1.5f32, tint(p.blue, 190)),
            StrokeKind::Inside,
        );
    }
    let mut x = rect.left() + 12.;
    if let Some(kind) = ico {
        draw_icon(
            ui.painter(),
            Rect::from_center_size(Pos2::new(x + 8., rect.center().y), Vec2::splat(16.)),
            kind,
            fg,
        );
        x += 22.;
    }
    ui.painter().text(
        Pos2::new(x, rect.center().y),
        Align2::LEFT_CENTER,
        label,
        font,
        fg,
    );
    resp
}

fn nav_row(
    ui: &mut egui::Ui,
    label: &str,
    ico: Ico,
    selected: bool,
    count: Option<usize>,
    p: Palette,
) -> egui::Response {
    let (rect, resp) = ui.allocate_exact_size(Vec2::new(ui.available_width(), 36.), Sense::click());
    if selected {
        ui.painter().rect_filled(rect, 8., p.selected);
    } else if resp.hovered() {
        ui.painter().rect_filled(rect, 8., p.hover);
    }
    let fg = if selected { p.blue } else { p.text };
    draw_icon(
        ui.painter(),
        Rect::from_center_size(
            Pos2::new(rect.left() + 19., rect.center().y),
            Vec2::splat(18.),
        ),
        ico,
        if selected { p.blue } else { p.muted },
    );
    ui.painter().text(
        Pos2::new(rect.left() + 37., rect.center().y),
        Align2::LEFT_CENTER,
        label,
        FontId::proportional(13.),
        fg,
    );
    if let Some(c) = count {
        let color = if selected { p.blue } else { p.muted };
        ui.painter().text(
            Pos2::new(rect.right() - 12., rect.center().y),
            Align2::RIGHT_CENTER,
            c.to_string(),
            FontId::proportional(11.5),
            color,
        );
    }
    if resp.has_focus() {
        ui.painter().rect_stroke(
            rect,
            8.,
            Stroke::new(1.0f32, tint(p.blue, 150)),
            StrokeKind::Inside,
        );
    }
    resp
}

fn search_field(
    ui: &mut egui::Ui,
    value: &mut String,
    p: Palette,
    focus: &mut bool,
    width: f32,
) -> egui::Response {
    let width = width.clamp(132., 208.);
    let frame = egui::Frame::new()
        .fill(p.surface)
        .corner_radius(9)
        .inner_margin(Margin::symmetric(11, 6))
        .show(ui, |ui| {
            ui.set_width(width - 22.);
            ui.with_layout(egui::Layout::left_to_right(Align::Center), |ui| {
                icon(ui, Ico::Search, p.muted, 16.);
                ui.add_space(2.);
                let resp = ui.add(
                    egui::TextEdit::singleline(value)
                        .id(egui::Id::new("torrent-search"))
                        .frame(false)
                        .hint_text("Поиск")
                        .desired_width(width - 51.)
                        .margin(Vec2::ZERO)
                        .text_color(p.text)
                        .font(FontId::proportional(13.)),
                );
                if *focus {
                    resp.request_focus();
                    *focus = false;
                }
                resp
            })
            .inner
        });
    let focused = frame.inner.has_focus();
    let stroke = if focused {
        Stroke::new(1.0f32, p.blue)
    } else if frame.response.hovered() {
        Stroke::new(
            1.0f32,
            p.line.gamma_multiply(if p.dark { 1.8 } else { 0.86 }),
        )
    } else {
        Stroke::new(1.0f32, p.line)
    };
    ui.painter()
        .rect_stroke(frame.response.rect, 9., stroke, StrokeKind::Inside);
    frame.inner
}

fn field(
    ui: &mut egui::Ui,
    value: &mut String,
    placeholder: &str,
    ico: Option<Ico>,
    width: f32,
    id: &str,
    focus: bool,
    p: Palette,
) -> egui::Response {
    let frame = egui::Frame::new()
        .fill(p.surface)
        .corner_radius(9)
        .inner_margin(Margin::symmetric(12, 7))
        .show(ui, |ui| {
            ui.set_width(width - 24.);
            ui.horizontal(|ui| {
                let mut x = 0.;
                if let Some(kind) = ico {
                    icon(ui, kind, p.muted, 16.);
                    ui.add_space(4.);
                    x = 20.;
                }
                let resp = ui.add(
                    egui::TextEdit::singleline(value)
                        .id(egui::Id::new(id))
                        .frame(false)
                        .hint_text(placeholder)
                        .desired_width((width - 24. - x).max(40.))
                        .margin(Vec2::ZERO)
                        .text_color(p.text)
                        .font(FontId::proportional(13.5)),
                );
                if focus {
                    resp.request_focus();
                }
                resp
            })
            .inner
        });
    let focused = frame.inner.has_focus();
    let stroke = if focused {
        Stroke::new(1.0f32, p.blue)
    } else {
        Stroke::new(1.0f32, p.line)
    };
    ui.painter()
        .rect_stroke(frame.response.rect, 9., stroke, StrokeKind::Inside);
    frame.inner
}

fn checkbox(ui: &mut egui::Ui, checked: &mut bool, label: &str, p: Palette) -> bool {
    let (rect, resp) = ui.allocate_exact_size(Vec2::new(ui.available_width(), 26.), Sense::click());
    let bx = Rect::from_center_size(
        Pos2::new(rect.left() + 9., rect.center().y),
        Vec2::splat(18.),
    );
    if *checked {
        ui.painter().rect_filled(bx, 5., p.blue_fill);
        draw_icon(ui.painter(), bx.shrink(3.), Ico::Check, Color::WHITE);
    } else {
        ui.painter().rect_filled(bx, 5., p.bg);
        ui.painter().rect_stroke(
            bx,
            5.,
            Stroke::new(
                1.0f32,
                p.line.gamma_multiply(if p.dark { 1.7 } else { 0.82 }),
            ),
            StrokeKind::Inside,
        );
    }
    ui.painter().text(
        Pos2::new(rect.left() + 28., rect.center().y),
        Align2::LEFT_CENTER,
        label,
        FontId::proportional(13.),
        p.text,
    );
    if resp.has_focus() {
        ui.painter().rect_stroke(
            rect,
            6.,
            Stroke::new(1.0f32, tint(p.blue, 120)),
            StrokeKind::Inside,
        );
    }
    if resp.clicked() {
        *checked = !*checked;
        true
    } else {
        false
    }
}

fn segmented(ui: &mut egui::Ui, dark: &mut bool, p: Palette) -> bool {
    let (rect, _) = ui.allocate_exact_size(Vec2::new(232., 34.), Sense::hover());
    ui.painter().rect_filled(rect, 9., p.surface);
    ui.painter()
        .rect_stroke(rect, 9., Stroke::new(1.0f32, p.line), StrokeKind::Inside);
    let half = rect.width() / 2.;
    let mut changed = false;
    for (i, (is_dark, label, ico)) in [(false, "Светлая", Ico::Sun), (true, "Тёмная", Ico::Moon)]
        .into_iter()
        .enumerate()
    {
        let seg = Rect::from_min_size(
            Pos2::new(rect.left() + i as f32 * half + 2., rect.top() + 2.),
            Vec2::new(half - 4., rect.height() - 4.),
        );
        let hit = ui.interact(seg, ui.id().with(("theme", i)), Sense::click());
        let active = *dark == is_dark;
        if active {
            ui.painter().rect_filled(seg, 7., p.bg);
            ui.painter()
                .rect_stroke(seg, 7., Stroke::new(1.0f32, p.line), StrokeKind::Inside);
        }
        draw_icon(
            ui.painter(),
            Rect::from_center_size(
                Pos2::new(seg.left() + 19., seg.center().y),
                Vec2::splat(15.),
            ),
            ico,
            if active { p.blue } else { p.muted },
        );
        ui.painter().text(
            Pos2::new(seg.left() + 32., seg.center().y),
            Align2::LEFT_CENTER,
            label,
            FontId::proportional(12.5),
            if active { p.text } else { p.muted },
        );
        if hit.clicked() && !active {
            *dark = is_dark;
            changed = true;
        }
    }
    changed
}

fn speed_badge(ui: &mut egui::Ui, v: f64, p: Palette) {
    let text = fmt_speed(v);
    let galley = ui
        .painter()
        .layout_no_wrap(text.clone(), FontId::proportional(11.), p.muted);
    let w = 16. + 4. + galley.size().x;
    let (r, _) = ui.allocate_exact_size(Vec2::new(w, 16.), Sense::hover());
    let color = if v > 0. { p.blue } else { p.muted };
    draw_icon(
        ui.painter(),
        Rect::from_center_size(Pos2::new(r.left() + 7., r.center().y), Vec2::splat(13.)),
        Ico::Download,
        color,
    );
    ui.painter().text(
        Pos2::new(r.left() + 18., r.center().y),
        Align2::LEFT_CENTER,
        text,
        FontId::proportional(11.),
        p.muted,
    );
}

fn section(ui: &mut egui::Ui, label: &str, p: Palette) {
    ui.label(Text::new(label).size(12.).color(p.muted));
}

fn plural(n: usize, one: &str, few: &str, many: &str) -> String {
    let tail = n % 100;
    let word = if (11..=14).contains(&tail) {
        many
    } else {
        match n % 10 {
            1 => one,
            2..=4 => few,
            _ => many,
        }
    };
    format!("{n} {word}")
}

fn state(row: &Value) -> (&'static str, u8) {
    let s = &row["stats"];
    if s["finished"] == true {
        ("Завершено", 2)
    } else {
        match s["state"].as_str().unwrap_or("") {
            "live" => ("Скачивание", 1),
            "paused" => ("Остановлено", 3),
            "error" => ("Ошибка", 4),
            _ => ("Подготовка", 0),
        }
    }
}

fn status_color(kind: u8, p: Palette) -> Color {
    match kind {
        1 => p.blue,
        2 => p.green,
        3 => p.muted,
        4 => p.red,
        _ => p.muted,
    }
}

fn matches(row: &Value, filter: usize) -> bool {
    filter == 0 || state(row).1 == filter as u8
}

fn speed(row: &Value, key: &str) -> f64 {
    row["stats"]["live"][key]["mbps"].as_f64().unwrap_or(0.) * 1048576.
}

fn eta(row: &Value) -> String {
    row["stats"]["live"]["time_remaining"]["human_readable"]
        .as_str()
        .unwrap_or("")
        .to_owned()
}

fn fmt_pct(v: f64) -> String {
    format!("{:.1}", v * 100.).replace('.', ",")
}

fn fmt_speed(v: f64) -> String {
    if v <= 0. {
        "—".into()
    } else {
        format!("{}/с", size(v))
    }
}

fn fmt_time(v: f64) -> String {
    if v <= 0. { "—".into() } else { size(v) }
}

impl App {
    fn configure(&self, ctx: &egui::Context, p: Palette) {
        let mut style = egui::Style::default();
        style.animation_time = if std::env::var("THODTORRENT_RENDER_CHECK").is_ok() {
            0.0
        } else {
            0.14
        };
        style
            .text_styles
            .insert(egui::TextStyle::Body, FontId::proportional(14.));
        style
            .text_styles
            .insert(egui::TextStyle::Button, FontId::proportional(13.));
        style
            .text_styles
            .insert(egui::TextStyle::Small, FontId::proportional(12.));
        style.spacing.item_spacing = Vec2::new(8., 8.);
        style.spacing.button_padding = Vec2::new(12., 7.);
        style.spacing.interact_size = Vec2::new(0., 32.);
        style.spacing.scroll.floating = true;
        style.spacing.scroll.bar_width = 8.;
        style.spacing.scroll.floating_width = 4.;
        style.spacing.scroll.bar_inner_margin = 2.;
        style.visuals = if self.settings.dark {
            egui::Visuals::dark()
        } else {
            egui::Visuals::light()
        };
        style.visuals.override_text_color = Some(p.text);
        style.visuals.panel_fill = p.bg;
        style.visuals.window_fill = p.bg;
        style.visuals.window_stroke = Stroke::new(1.0f32, p.line);
        style.visuals.window_corner_radius = 16.into();
        style.visuals.menu_corner_radius = 10.into();
        style.visuals.extreme_bg_color = p.field;
        style.visuals.faint_bg_color = p.surface;
        style.visuals.code_bg_color = p.surface;
        style.visuals.selection.bg_fill = p.selected;
        style.visuals.selection.stroke = Stroke::new(1.0f32, p.blue);
        for w in [
            &mut style.visuals.widgets.inactive,
            &mut style.visuals.widgets.hovered,
            &mut style.visuals.widgets.active,
            &mut style.visuals.widgets.open,
        ] {
            w.corner_radius = 8.into();
            w.bg_stroke = Stroke::new(1.0f32, p.line);
            w.fg_stroke = Stroke::new(1.0f32, p.text);
            w.expansion = 0.;
        }
        style.visuals.widgets.inactive.bg_fill = p.surface;
        style.visuals.widgets.inactive.weak_bg_fill = p.surface;
        style.visuals.widgets.hovered.bg_fill = p.hover;
        style.visuals.widgets.hovered.weak_bg_fill = p.hover;
        style.visuals.widgets.hovered.bg_stroke = Stroke::new(1.0f32, p.line);
        style.visuals.widgets.active.bg_fill = p.selected;
        style.visuals.widgets.active.weak_bg_fill = p.selected;
        style.visuals.widgets.active.bg_stroke = Stroke::new(1.0f32, p.blue);
        style.visuals.widgets.noninteractive.bg_fill = p.surface;
        style.visuals.widgets.noninteractive.bg_stroke = Stroke::new(1.0f32, p.line);
        style.visuals.widgets.noninteractive.fg_stroke = Stroke::new(1.0f32, p.text);
        ctx.set_style(style);
    }

    fn open_add(&mut self) {
        self.adding = true;
        self.focus_source = true;
    }

    fn open_file(&mut self) {
        if let Some(path) = rfd::FileDialog::new()
            .add_filter("Торрент", &["torrent"])
            .pick_file()
        {
            self.source = path.display().to_string();
            self.open_add();
        }
    }

    fn sidebar(&mut self, ctx: &egui::Context, p: Palette) {
        egui::SidePanel::left("sidebar")
            .exact_width(216.)
            .resizable(false)
            .frame(
                egui::Frame::new()
                    .fill(p.side)
                    .inner_margin(Margin::symmetric(16, 20)),
            )
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    let size = 26.;
                    let (rect, _) = ui.allocate_exact_size(Vec2::splat(size), Sense::hover());
                    ui.painter().image(
                        self.icon_tex.id(),
                        rect,
                        Rect::from_min_max(Pos2::ZERO, Pos2::new(1., 1.)),
                        Color::WHITE,
                    );
                    ui.add_space(2.);
                    ui.label(
                        Text::new("ThodTorrent")
                            .size(16.)
                            .font(bold(16.))
                            .color(p.text),
                    );
                });
                ui.add_space(30.);
                section(ui, "Библиотека", p);
                ui.add_space(6.);
                ui.scope(|ui| {
                    ui.spacing_mut().item_spacing.y = 2.;
                    for (filter, label, ico) in [
                        (0, "Все загрузки", Ico::Menu),
                        (1, "Скачиваются", Ico::Download),
                        (2, "Завершены", Ico::Check),
                        (3, "Остановлены", Ico::Pause),
                    ] {
                        let count = self.rows.iter().filter(|r| matches(r, filter)).count();
                        let selected = self.filter == filter;
                        let resp = nav_row(ui, label, ico, selected, Some(count), p);
                        if resp.clicked() {
                            self.filter = filter;
                            self.selected = None;
                        }
                    }
                });
                ui.with_layout(egui::Layout::bottom_up(Align::LEFT), |ui| {
                    ui.label(
                        Text::new(format!("Версия {}", env!("CARGO_PKG_VERSION")))
                            .size(11.)
                            .color(p.muted),
                    );
                    ui.add_space(6.);
                    if nav_row(ui, "Настройки", Ico::Settings, false, None, p).clicked() {
                        self.preferences = true;
                    }
                    ui.add_space(8.);
                    let (line, _) =
                        ui.allocate_exact_size(Vec2::new(ui.available_width(), 1.), Sense::hover());
                    ui.painter().line_segment(
                        [line.left_top(), line.right_top()],
                        Stroke::new(1.0f32, p.line),
                    );
                });
            });
    }

    fn header(&mut self, ui: &mut egui::Ui, p: Palette) {
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.add_space(2.);
                let title =
                    ["Все загрузки", "Скачиваются", "Завершены", "Остановлены"][self.filter];
                ui.label(Text::new(title).size(27.).font(bold(27.)).color(p.text));
                ui.add_space(3.);
                let shown = self
                    .rows
                    .iter()
                    .filter(|r| matches(r, self.filter))
                    .filter(|r| {
                        r["name"]
                            .as_str()
                            .unwrap_or("")
                            .to_lowercase()
                            .contains(&self.search.to_lowercase())
                    })
                    .count();
                let down: f64 = self.rows.iter().map(|r| speed(r, "download_speed")).sum();
                let mut subtitle = if self.search.is_empty() {
                    plural(shown, "загрузка", "загрузки", "загрузок")
                } else {
                    format!("Найдено: {shown}")
                };
                if down > 0. {
                    subtitle.push_str(&format!("  ·  {}", fmt_speed(down)));
                }
                ui.label(Text::new(subtitle).size(12.5).color(p.muted));
            });
            ui.with_layout(egui::Layout::right_to_left(Align::Center), |ui| {
                if button(ui, "Добавить", Some(Ico::Plus), Btn::Primary, p, true).clicked()
                {
                    self.open_add();
                }
                ui.add_space(4.);
                let search_w = (ui.available_width() - 8.).clamp(132., 208.);
                search_field(ui, &mut self.search, p, &mut self.focus_search, search_w);
            });
        });
        ui.add_space(16.);
        ui.horizontal(|ui| {
            if button(ui, "Открыть файл", Some(Ico::File), Btn::Secondary, p, true).clicked()
            {
                self.open_file();
            }
            let has_sel = self.selected.is_some();
            let can_start = has_sel
                && self
                    .selected
                    .and_then(|id| self.rows.iter().find(|r| r["id"].as_u64() == Some(id)))
                    .map(|r| state(r).1 != 2)
                    .unwrap_or(false);
            let can_pause = has_sel
                && self
                    .selected
                    .and_then(|id| self.rows.iter().find(|r| r["id"].as_u64() == Some(id)))
                    .map(|r| state(r).1 == 1)
                    .unwrap_or(false);
            if button(
                ui,
                "Продолжить",
                Some(Ico::Play),
                Btn::Secondary,
                p,
                can_start,
            )
            .clicked()
            {
                self.send(json!({"method":"start","id":self.selected}));
            }
            if button(
                ui,
                "Остановить",
                Some(Ico::Pause),
                Btn::Secondary,
                p,
                can_pause,
            )
            .clicked()
            {
                self.send(json!({"method":"pause","id":self.selected}));
            }
            ui.menu_button(Text::new("Столбцы").size(13.), |ui| {
                ui.set_width(190.);
                for (i, name) in COLUMNS.iter().enumerate().skip(1) {
                    if ui.checkbox(&mut self.settings.columns[i], *name).changed() {
                        self.save();
                    }
                }
            });
        });
    }

    fn context_menu(&mut self, ui: &mut egui::Ui, row: &Value, ctx: &egui::Context, p: Palette) {
        let id = row["id"].as_u64().unwrap_or(0);
        for (label, method) in [
            ("Продолжить загрузку", "start"),
            ("Остановить", "pause"),
            ("Сведения и файлы", "details"),
        ] {
            if ui.button(label).clicked() {
                self.send(json!({"method":method,"id":id}));
                ui.close_menu();
            }
        }
        ui.separator();
        if ui.button("Показать в папке").clicked() {
            let _ = Command::new("explorer.exe")
                .arg(row["output_folder"].as_str().unwrap_or(""))
                .spawn();
            ui.close_menu();
        }
        if ui.button("Скопировать magnet-ссылку").clicked() {
            ctx.copy_text(format!(
                "magnet:?xt=urn:btih:{}",
                row["info_hash"].as_str().unwrap_or("")
            ));
            ui.close_menu();
        }
        ui.separator();
        if ui.button("Убрать из списка").clicked() {
            self.send(json!({"method":"forget","id":id}));
            ui.close_menu();
        }
        if ui
            .button(Text::new("Удалить вместе с файлами…").color(p.red))
            .clicked()
        {
            self.confirm = Some(id);
            ui.close_menu();
        }
    }

    fn empty_state(&mut self, ui: &mut egui::Ui, p: Palette) {
        let no_rows = self.rows.is_empty();
        ui.add_space((ui.available_height() * 0.18).max(28.));
        ui.vertical_centered(|ui| {
            let (rect, _) = ui.allocate_exact_size(Vec2::splat(76.), Sense::hover());
            ui.painter()
                .rect_filled(rect, 20., tint(p.blue, if p.dark { 34 } else { 18 }));
            draw_icon(
                ui.painter(),
                rect.shrink(23.),
                if no_rows { Ico::Download } else { Ico::Search },
                p.blue,
            );
            ui.add_space(18.);
            ui.label(
                Text::new(if no_rows {
                    "Всё начинается со ссылки"
                } else {
                    "Ничего не найдено"
                })
                .size(22.)
                .font(bold(22.))
                .color(p.text),
            );
            ui.add_space(7.);
            ui.label(
                Text::new(if no_rows {
                    "Добавьте magnet-ссылку или откройте файл .torrent."
                } else {
                    "Измените запрос или выберите другой раздел."
                })
                .size(13.)
                .color(p.muted),
            );
            ui.add_space(20.);
            ui.horizontal(|ui| {
                let total_w = if no_rows { 316. } else { 150. };
                ui.add_space((ui.available_width() - total_w).max(0.) / 2.);
                if no_rows {
                    if button(
                        ui,
                        "Добавить загрузку",
                        Some(Ico::Plus),
                        Btn::Primary,
                        p,
                        true,
                    )
                    .clicked()
                    {
                        self.open_add();
                    }
                    if button(ui, "Открыть файл", Some(Ico::File), Btn::Secondary, p, true)
                        .clicked()
                    {
                        self.open_file();
                    }
                } else if button(ui, "Сбросить поиск", None, Btn::Secondary, p, true).clicked()
                {
                    self.search.clear();
                }
            });
            if no_rows {
                ui.add_space(14.);
                ui.label(
                    Text::new("Можно перетащить .torrent прямо в окно")
                        .size(11.5)
                        .color(p.muted),
                );
            }
        });
    }

    fn table(&mut self, ui: &mut egui::Ui, p: Palette) {
        let rows: Vec<_> = self
            .rows
            .iter()
            .filter(|r| matches(r, self.filter))
            .filter(|r| {
                r["name"]
                    .as_str()
                    .unwrap_or("")
                    .to_lowercase()
                    .contains(&self.search.to_lowercase())
            })
            .cloned()
            .collect();
        if rows.is_empty() {
            self.empty_state(ui, p);
            return;
        }
        let cols: Vec<_> = (0..COLUMNS.len())
            .filter(|&i| self.settings.columns[i])
            .collect();
        if cols.is_empty() {
            return;
        }
        let base = [
            304., 170., 88., 96., 104., 92., 92., 104., 210., 92., 76., 72., 240.,
        ];
        let mut widths = base;
        let sum: f32 = cols.iter().map(|&i| widths[i]).sum();
        let avail = ui.available_width();
        if cols.contains(&0) && avail > sum {
            widths[0] += avail - sum;
        }
        let total_w: f32 = cols.iter().map(|&i| widths[i]).sum();

        egui::ScrollArea::horizontal()
            .id_salt("table-h")
            .auto_shrink([false, false])
            .show(ui, |ui| {
                ui.set_min_width(total_w);
                ui.spacing_mut().item_spacing = Vec2::ZERO;
                let (head, _) = ui.allocate_exact_size(Vec2::new(total_w, HEAD_H), Sense::hover());
                ui.painter().rect_filled(head, 8., p.surface);
                let mut x = head.left();
                for &i in &cols {
                    let cell =
                        Rect::from_min_size(Pos2::new(x, head.top()), Vec2::new(widths[i], HEAD_H));
                    x += widths[i];
                    let right = i != 0 && i != 1 && i != 7 && i != 8 && i != 12;
                    let tx = if right {
                        cell.right() - 14.
                    } else {
                        cell.left() + 14.
                    };
                    ui.painter().text(
                        Pos2::new(tx, cell.center().y),
                        if right {
                            Align2::RIGHT_CENTER
                        } else {
                            Align2::LEFT_CENTER
                        },
                        COLUMNS[i],
                        FontId::proportional(11.),
                        p.muted,
                    );
                }
                ui.add_space(6.);
                let vh = ui.available_height().max(80.);
                egui::ScrollArea::vertical()
                    .id_salt("table-v")
                    .auto_shrink([false, false])
                    .max_height(vh)
                    .show(ui, |ui| {
                        ui.set_min_width(total_w);
                        ui.spacing_mut().item_spacing = Vec2::ZERO;
                        for row in rows {
                            self.table_row(ui, &row, &cols, &widths, total_w, p);
                        }
                    });
            });
    }

    fn table_row(
        &mut self,
        ui: &mut egui::Ui,
        row: &Value,
        cols: &[usize],
        widths: &[f32; 13],
        total_w: f32,
        p: Palette,
    ) {
        let id = row["id"].as_u64().unwrap_or(0);
        let s = &row["stats"];
        let total = s["total_bytes"].as_f64().unwrap_or(0.);
        let done = s["progress_bytes"].as_f64().unwrap_or(0.);
        let progress = if total > 0. {
            (done / total).clamp(0., 1.)
        } else {
            0.
        };
        let (label, kind) = state(row);
        let selected = self.selected == Some(id);

        let (rect, resp) = ui.allocate_exact_size(Vec2::new(total_w, ROW_H), Sense::click());
        if selected {
            ui.painter().rect_filled(rect, 8., p.selected);
            ui.painter().rect_filled(
                Rect::from_center_size(
                    Pos2::new(rect.left() + 2.5, rect.center().y),
                    Vec2::new(3., 26.),
                ),
                1.5,
                p.blue,
            );
        } else if resp.hovered() {
            ui.painter().rect_filled(rect, 8., p.hover);
        }
        if resp.clicked() || resp.secondary_clicked() {
            self.selected = Some(id);
        }
        if resp.double_clicked() {
            self.send(json!({"method":"details","id":id}));
        }
        let ctx = ui.ctx().clone();
        resp.context_menu(|ui| self.context_menu(ui, row, &ctx, p));

        let mut x = rect.left();
        for &i in cols {
            let cell = Rect::from_min_size(Pos2::new(x, rect.top()), Vec2::new(widths[i], ROW_H));
            let painter = ui.painter().with_clip_rect(cell);
            x += widths[i];
            match i {
                0 => {
                    let color = status_color(kind, p);
                    let ir = Rect::from_min_size(
                        Pos2::new(cell.left() + 14., cell.center().y - 15.),
                        Vec2::splat(30.),
                    );
                    painter.rect_filled(ir, 8., tint(color, if p.dark { 38 } else { 22 }));
                    draw_icon(&painter, ir.shrink(7.), Ico::File, color);
                    let name = row["name"].as_str().unwrap_or("Получение метаданных…");
                    let max = (cell.width() - 64.).max(20.);
                    let mut job = egui::text::LayoutJob::simple_singleline(
                        name.to_owned(),
                        FontId::proportional(13.5),
                        p.text,
                    );
                    job.wrap = egui::text::TextWrapping {
                        max_width: max,
                        max_rows: 1,
                        break_anywhere: true,
                        overflow_character: Some('…'),
                    };
                    let galley = painter.layout_job(job);
                    let clipped = painter.with_clip_rect(Rect::from_min_size(
                        Pos2::new(cell.left() + 54., cell.top() + 10.),
                        Vec2::new(max, 20.),
                    ));
                    clipped.galley(
                        Pos2::new(cell.left() + 54., cell.top() + 10.),
                        galley,
                        p.text,
                    );
                    painter.circle_filled(
                        Pos2::new(cell.left() + 57., cell.top() + 39.),
                        2.5,
                        color,
                    );
                    painter.text(
                        Pos2::new(cell.left() + 65., cell.top() + 39.),
                        Align2::LEFT_CENTER,
                        label,
                        FontId::proportional(11.5),
                        color,
                    );
                }
                1 => {
                    let pct = if total > 0. {
                        fmt_pct(progress)
                    } else {
                        "—".into()
                    };
                    painter.text(
                        Pos2::new(cell.left() + 14., cell.top() + 20.),
                        Align2::LEFT_CENTER,
                        pct,
                        FontId::proportional(12.5),
                        if total > 0. { p.text } else { p.muted },
                    );
                    let track = Rect::from_min_size(
                        Pos2::new(cell.left() + 14., cell.top() + 36.),
                        Vec2::new((cell.width() - 28.).max(10.), 4.),
                    );
                    painter.rect_filled(track, 2., p.line);
                    if progress > 0. {
                        painter.rect_filled(
                            Rect::from_min_size(
                                track.min,
                                Vec2::new(track.width() * progress as f32, 4.),
                            ),
                            2.,
                            status_color(kind, p),
                        );
                    }
                }
                7 => {
                    let color = status_color(kind, p);
                    let galley =
                        painter.layout_no_wrap(label.to_owned(), FontId::proportional(11.5), color);
                    let pill_w = galley.size().x + 22.;
                    let pill = Rect::from_min_size(
                        Pos2::new(cell.left() + 14., cell.center().y - 10.),
                        Vec2::new(pill_w, 20.),
                    );
                    painter.rect_filled(pill, 6., tint(color, if p.dark { 34 } else { 18 }));
                    painter.circle_filled(Pos2::new(pill.left() + 9., pill.center().y), 2.2, color);
                    painter.galley(
                        Pos2::new(pill.left() + 15., pill.center().y - galley.size().y / 2.),
                        galley,
                        color,
                    );
                }
                8 => {
                    let folder = row["output_folder"].as_str().unwrap_or("");
                    painter.text(
                        Pos2::new(cell.left() + 14., cell.center().y),
                        Align2::LEFT_CENTER,
                        folder,
                        FontId::proportional(12.),
                        p.muted,
                    );
                }
                _ => {
                    let text = match i {
                        2 => fmt_time(total),
                        3 => fmt_time(done),
                        4 => fmt_speed(speed(row, "download_speed")),
                        5 => fmt_speed(speed(row, "upload_speed")),
                        6 => {
                            let e = eta(row);
                            if e.is_empty() { "—".into() } else { e }
                        }
                        9 => fmt_time(s["uploaded_bytes"].as_f64().unwrap_or(0.)),
                        10 => {
                            let up = s["uploaded_bytes"].as_f64().unwrap_or(0.);
                            if up > 0. && done > 0. {
                                format!("{:.2}", up / done).replace('.', ",")
                            } else {
                                "—".into()
                            }
                        }
                        11 => {
                            let n = row["total_pieces"].as_u64().unwrap_or(0);
                            if n > 0 { n.to_string() } else { "—".into() }
                        }
                        12 => row["info_hash"].as_str().unwrap_or("—").into(),
                        _ => String::new(),
                    };
                    let color = if i == 4 && kind == 1 && speed(row, "download_speed") > 0. {
                        p.blue
                    } else if i == 3 || i == 4 || i == 6 || i == 9 || i == 10 || i == 11 {
                        p.text
                    } else {
                        p.text
                    };
                    let font = if i == 12 {
                        FontId::monospace(11.)
                    } else {
                        FontId::proportional(12.5)
                    };
                    let color = if (i == 3 || i == 4 || i == 6 || i == 9 || i == 10 || i == 11)
                        && text == "—"
                    {
                        p.muted
                    } else {
                        color
                    };
                    painter.text(
                        Pos2::new(cell.right() - 14., cell.center().y),
                        Align2::RIGHT_CENTER,
                        text,
                        font,
                        color,
                    );
                }
            }
        }
        ui.painter().line_segment(
            [
                Pos2::new(rect.left() + 14., rect.bottom()),
                Pos2::new(rect.right() - 14., rect.bottom()),
            ],
            Stroke::new(0.5f32, p.line),
        );
    }

    fn dialogs(&mut self, ctx: &egui::Context, p: Palette) {
        let backdrop = Color::from_black_alpha(if p.dark { 120 } else { 64 });
        if self.confirm.is_some() {
            self.confirm_dialog(ctx, p, backdrop);
        } else if self.details.is_some() {
            self.details_dialog(ctx, p, backdrop);
        } else if self.preferences {
            self.settings_dialog(ctx, p, backdrop);
        } else if self.adding {
            self.add_dialog(ctx, p, backdrop);
        }
    }

    fn modal_frame(&self, p: Palette) -> egui::Frame {
        egui::Frame::new()
            .fill(p.bg)
            .stroke(Stroke::new(1.0f32, p.line))
            .corner_radius(16)
            .inner_margin(24)
            .shadow(egui::epaint::Shadow {
                offset: [0, 18],
                blur: 48,
                spread: 0,
                color: Color::from_black_alpha(if p.dark { 110 } else { 54 }),
            })
    }

    fn dialog_header(&mut self, ui: &mut egui::Ui, title: &str, p: Palette, close: &mut bool) {
        ui.horizontal(|ui| {
            let w = (ui.available_width() - 36.).max(80.);
            ui.allocate_ui_with_layout(
                Vec2::new(w, 28.),
                egui::Layout::left_to_right(Align::Center),
                |ui| {
                    ui.add(
                        egui::Label::new(Text::new(title).size(21.).font(bold(21.)).color(p.text))
                            .truncate(),
                    );
                },
            );
            ui.with_layout(egui::Layout::right_to_left(Align::Center), |ui| {
                let (rect, resp) = ui.allocate_exact_size(Vec2::splat(28.), Sense::click());
                if resp.hovered() {
                    ui.painter().rect_filled(rect, 7., p.hover);
                }
                draw_icon(
                    ui.painter(),
                    Rect::from_center_size(rect.center(), Vec2::splat(14.)),
                    Ico::Close,
                    p.muted,
                );
                if resp.clicked() {
                    *close = true;
                }
            });
        });
    }

    fn add_dialog(&mut self, ctx: &egui::Context, p: Palette, backdrop: Color) {
        let mut close = false;
        let mut submit = false;
        let modal = egui::Modal::new(egui::Id::new("new-download"))
            .backdrop_color(backdrop)
            .frame(self.modal_frame(p))
            .show(ctx, |ui| {
                ui.set_width(500.);
                self.dialog_header(ui, "Новая загрузка", p, &mut close);
                ui.add_space(6.);
                ui.label(
                    Text::new("Вставьте ссылку или выберите торрент-файл.")
                        .size(13.)
                        .color(p.muted),
                );
                ui.add_space(20.);

                section(ui, "Ссылка или файл", p);
                ui.add_space(6.);
                let focus = self.focus_source;
                self.focus_source = false;
                let resp = field(
                    ui,
                    &mut self.source,
                    "magnet:?xt=… или https://…",
                    Some(Ico::External),
                    ui.available_width(),
                    "source-field",
                    focus,
                    p,
                );
                if resp.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                    submit = true;
                }
                if button(
                    ui,
                    "Выбрать файл .torrent",
                    Some(Ico::File),
                    Btn::Ghost,
                    p,
                    true,
                )
                .clicked()
                {
                    self.open_file();
                }

                ui.add_space(14.);
                section(ui, "Сохранить в", p);
                ui.add_space(6.);
                let bw = 104.;
                let fw = (ui.available_width() - bw - 8.).max(120.);
                ui.horizontal(|ui| {
                    field(
                        ui,
                        &mut self.folder,
                        "Папка загрузки",
                        Some(Ico::Folder),
                        fw,
                        "folder-field",
                        false,
                        p,
                    );
                    if button(ui, "Выбрать…", None, Btn::Secondary, p, true).clicked() {
                        if let Some(path) = rfd::FileDialog::new().pick_folder() {
                            self.folder = path.display().to_string();
                        }
                    }
                });
                ui.add_space(4.);
                checkbox(
                    ui,
                    &mut self.remember_folder,
                    "Использовать эту папку для новых загрузок",
                    p,
                );
                ui.add_space(14.);
                egui::Frame::new()
                    .fill(p.surface)
                    .corner_radius(10)
                    .inner_margin(Margin::symmetric(12, 10))
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            icon(ui, Ico::Check, p.green, 16.);
                            ui.add_space(2.);
                            ui.label(
                                Text::new("После завершения загрузка остановится автоматически.")
                                    .size(12.)
                                    .color(p.muted),
                            );
                        });
                    });
                ui.add_space(20.);
                let valid = !self.source.trim().is_empty() && !self.folder.trim().is_empty();
                ui.horizontal(|ui| {
                    ui.with_layout(egui::Layout::right_to_left(Align::Center), |ui| {
                        if button(ui, "Начать загрузку", None, Btn::Primary, p, valid).clicked()
                        {
                            submit = true;
                        }
                        ui.add_space(2.);
                        if button(ui, "Отмена", None, Btn::Secondary, p, true).clicked() {
                            close = true;
                        }
                    });
                });
            });
        if modal.should_close() {
            close = true;
        }
        if submit {
            if self.remember_folder {
                self.settings.folder = self.folder.clone();
                self.save();
            }
            self.send(json!({"method":"add","source":self.source.trim(),"folder":self.folder}));
            self.source.clear();
            self.notice = "Получение метаданных…".into();
            close = true;
        }
        if close {
            self.adding = false;
        }
    }

    fn settings_dialog(&mut self, ctx: &egui::Context, p: Palette, backdrop: Color) {
        let mut close = false;
        let gap = if ctx.screen_rect().height() < 660. {
            14.
        } else {
            22.
        };
        let modal = egui::Modal::new(egui::Id::new("preferences"))
            .backdrop_color(backdrop)
            .frame(self.modal_frame(p))
            .show(ctx, |ui| {
                ui.set_width(500.);
                self.dialog_header(ui, "Настройки", p, &mut close);
                ui.add_space(gap);

                section(ui, "Оформление", p);
                ui.add_space(8.);
                let mut dark = self.settings.dark;
                if segmented(ui, &mut dark, p) {
                    self.settings.dark = dark;
                    self.save();
                }

                ui.add_space(gap);
                section(ui, "Папка загрузок", p);
                ui.add_space(8.);
                ui.horizontal(|ui| {
                    let bw = 132.;
                    let fw = (ui.available_width() - bw - 8.).max(120.);
                    egui::Frame::new()
                        .fill(p.surface)
                        .corner_radius(9)
                        .inner_margin(Margin::symmetric(12, 8))
                        .show(ui, |ui| {
                            ui.set_width(fw - 24.);
                            ui.add(
                                egui::Label::new(
                                    Text::new(&self.settings.folder).size(12.5).color(p.text),
                                )
                                .truncate(),
                            );
                        });
                    if button(ui, "Выбрать папку", None, Btn::Secondary, p, true).clicked() {
                        if let Some(path) = rfd::FileDialog::new().pick_folder() {
                            self.settings.folder = path.display().to_string();
                            self.folder = self.settings.folder.clone();
                            self.save();
                        }
                    }
                });
                ui.add_space(6.);
                ui.label(
                    Text::new("Новые загрузки сохраняются сюда, если не выбрать другую папку.")
                        .size(11.5)
                        .color(p.muted),
                );

                ui.add_space(gap);
                section(ui, "Открывать торренты здесь", p);
                ui.add_space(8.);
                ui.label(
                    Text::new("Выберите ThodTorrent для .torrent и magnet-ссылок\nв настройках приложений Windows.")
                        .size(12.5)
                        .color(p.muted),
                );
                ui.add_space(8.);
                if button(ui, "Настроить в Windows", Some(Ico::External), Btn::Secondary, p, true)
                    .clicked()
                {
                    match integration::run(false) {
                        Ok(()) => {
                            let _ = Command::new("explorer.exe")
                                .arg("ms-settings:defaultapps")
                                .spawn();
                        }
                        Err(e) => self.error = format!("Регистрация: {e:#}"),
                    }
                }

                ui.add_space(gap);
                egui::Frame::new()
                    .fill(p.surface)
                    .corner_radius(10)
                    .inner_margin(Margin::symmetric(12, 10))
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            icon(ui, Ico::Info, p.muted, 16.);
                            ui.add_space(2.);
                            ui.label(
                                Text::new("Отдача отключена. Завершённые загрузки останавливаются.")
                                    .size(12.)
                                    .color(p.muted),
                            );
                        });
                    });
                ui.add_space(if gap < 20. { 14. } else { 20. });
                ui.horizontal(|ui| {
                    ui.with_layout(egui::Layout::right_to_left(Align::Center), |ui| {
                        if button(ui, "Готово", None, Btn::Primary, p, true).clicked() {
                            close = true;
                        }
                    });
                });
            });
        if modal.should_close() {
            close = true;
        }
        if close {
            self.preferences = false;
        }
    }

    fn details_dialog(&mut self, ctx: &egui::Context, p: Palette, backdrop: Color) {
        let Some(details) = self.details.clone() else {
            return;
        };
        let id = details["id"].as_u64();
        let row = id.and_then(|id| self.rows.iter().find(|r| r["id"].as_u64() == Some(id)));
        let total = row
            .map(|r| r["stats"]["total_bytes"].as_f64().unwrap_or(0.))
            .unwrap_or_else(|| {
                details["files"]
                    .as_array()
                    .map(|f| f.iter().map(|x| x["length"].as_f64().unwrap_or(0.)).sum())
                    .unwrap_or(0.)
            });
        let done = row
            .map(|r| r["stats"]["progress_bytes"].as_f64().unwrap_or(0.))
            .unwrap_or(0.);
        let (label, kind) = row.map(|r| state(r)).unwrap_or(("Подготовка", 0));
        let folder = details["output_folder"].as_str().unwrap_or("");
        let name = details["name"].as_str().unwrap_or("Торрент");
        let mut close = false;
        let mut open_folder = false;
        let files_h = (ctx.screen_rect().height() * 0.36).clamp(110., 280.);
        let modal = egui::Modal::new(egui::Id::new("details"))
            .backdrop_color(backdrop)
            .frame(self.modal_frame(p))
            .show(ctx, |ui| {
                ui.set_width(540.);
                self.dialog_header(ui, name, p, &mut close);
                ui.add_space(3.);
                ui.horizontal(|ui| {
                    icon(ui, Ico::Folder, p.muted, 14.);
                    ui.add(egui::Label::new(Text::new(folder).size(12.).color(p.muted)).truncate());
                });
                ui.add_space(18.);

                egui::Frame::new()
                    .fill(p.surface)
                    .corner_radius(12)
                    .inner_margin(Margin::symmetric(6, 12))
                    .show(ui, |ui| {
                        ui.columns(4, |cols| {
                            let stats = [
                                ("Размер", fmt_time(total)),
                                ("Скачано", fmt_time(done)),
                                (
                                    "Осталось",
                                    if total > 0. && done > 0. {
                                        fmt_time(total - done)
                                    } else {
                                        "—".into()
                                    },
                                ),
                                ("Состояние", label.to_string()),
                            ];
                            for (i, (cap, val)) in stats.into_iter().enumerate() {
                                cols[i].vertical_centered(|ui| {
                                    ui.label(Text::new(cap).size(10.5).color(p.muted));
                                    ui.add_space(3.);
                                    ui.label(Text::new(val).size(13.).font(bold(13.)).color(
                                        if i == 3 {
                                            status_color(kind, p)
                                        } else {
                                            p.text
                                        },
                                    ));
                                });
                            }
                        });
                    });

                ui.add_space(18.);
                let files_n = details["files"].as_array().map(|f| f.len()).unwrap_or(0);
                section(ui, &format!("Файлы · {files_n}"), p);
                ui.add_space(6.);
                egui::Frame::new()
                    .fill(p.surface)
                    .corner_radius(10)
                    .inner_margin(Margin::symmetric(12, 4))
                    .show(ui, |ui| {
                        ui.set_width(ui.available_width());
                        egui::ScrollArea::vertical()
                            .max_height(files_h)
                            .auto_shrink([false, true])
                            .show(ui, |ui| {
                                ui.spacing_mut().item_spacing = Vec2::ZERO;
                                if let Some(files) = details["files"].as_array() {
                                    for (n, file) in files.iter().enumerate() {
                                        let (r, _) = ui.allocate_exact_size(
                                            Vec2::new(ui.available_width(), 34.),
                                            Sense::hover(),
                                        );
                                        draw_icon(
                                            ui.painter(),
                                            Rect::from_center_size(
                                                Pos2::new(r.left() + 10., r.center().y),
                                                Vec2::splat(15.),
                                            ),
                                            Ico::File,
                                            p.muted,
                                        );
                                        ui.painter().text(
                                            Pos2::new(r.left() + 25., r.center().y),
                                            Align2::LEFT_CENTER,
                                            file["name"].as_str().unwrap_or(""),
                                            FontId::proportional(12.5),
                                            p.text,
                                        );
                                        ui.painter().text(
                                            Pos2::new(r.right() - 4., r.center().y),
                                            Align2::RIGHT_CENTER,
                                            size(file["length"].as_f64().unwrap_or(0.)),
                                            FontId::proportional(11.5),
                                            p.muted,
                                        );
                                        if n + 1 < files.len() {
                                            ui.painter().line_segment(
                                                [
                                                    Pos2::new(r.left() + 25., r.bottom()),
                                                    Pos2::new(r.right(), r.bottom()),
                                                ],
                                                Stroke::new(0.5f32, p.line),
                                            );
                                        }
                                    }
                                } else {
                                    ui.add_space(8.);
                                    ui.label(
                                        Text::new(
                                            "Список файлов появится после получения метаданных.",
                                        )
                                        .size(12.)
                                        .color(p.muted),
                                    );
                                    ui.add_space(8.);
                                }
                            });
                    });

                ui.add_space(18.);
                ui.horizontal(|ui| {
                    ui.with_layout(egui::Layout::right_to_left(Align::Center), |ui| {
                        if button(ui, "Готово", None, Btn::Primary, p, true).clicked() {
                            close = true;
                        }
                        ui.add_space(2.);
                        if button(
                            ui,
                            "Показать в папке",
                            Some(Ico::Folder),
                            Btn::Secondary,
                            p,
                            true,
                        )
                        .clicked()
                        {
                            open_folder = true;
                        }
                    });
                });
            });
        if modal.should_close() {
            close = true;
        }
        if open_folder {
            let _ = Command::new("explorer.exe").arg(folder).spawn();
        }
        if close {
            self.details = None;
        }
    }

    fn confirm_dialog(&mut self, ctx: &egui::Context, p: Palette, backdrop: Color) {
        let Some(id) = self.confirm else {
            return;
        };
        let name = self
            .rows
            .iter()
            .find(|r| r["id"].as_u64() == Some(id))
            .and_then(|r| r["name"].as_str())
            .unwrap_or("Торрент")
            .to_owned();
        let mut close = false;
        let mut delete = false;
        let modal = egui::Modal::new(egui::Id::new("delete-confirm"))
            .backdrop_color(backdrop)
            .frame(self.modal_frame(p))
            .show(ctx, |ui| {
                ui.set_width(420.);
                ui.horizontal(|ui| {
                    let (r, _) = ui.allocate_exact_size(Vec2::splat(34.), Sense::hover());
                    ui.painter().rect_filled(r, 10., tint(p.red, if p.dark { 40 } else { 18 }));
                    draw_icon(ui.painter(), r.shrink(9.), Ico::Trash, p.red);
                    ui.add_space(4.);
                    ui.label(
                        Text::new("Удалить файлы?")
                            .size(20.)
                            .font(bold(20.))
                            .color(p.text),
                    );
                });
                ui.add_space(16.);
                egui::Frame::new()
                    .fill(p.surface)
                    .corner_radius(9)
                    .inner_margin(Margin::symmetric(12, 10))
                    .show(ui, |ui| {
                        ui.set_width(ui.available_width());
                        ui.label(Text::new(&name).size(12.5).color(p.text));
                    });
                ui.add_space(12.);
                ui.label(
                    Text::new("Файлы будут удалены с диска безвозвратно. Чтобы сохранить их, выберите «Убрать из списка» в меню загрузки.")
                        .size(12.5)
                        .color(p.muted),
                );
                ui.add_space(20.);
                ui.horizontal(|ui| {
                    ui.with_layout(egui::Layout::right_to_left(Align::Center), |ui| {
                        if button(ui, "Удалить файлы", Some(Ico::Trash), Btn::Danger, p, true)
                            .clicked()
                        {
                            delete = true;
                        }
                        ui.add_space(2.);
                        if button(ui, "Отмена", None, Btn::Secondary, p, true).clicked() {
                            close = true;
                        }
                    });
                });
            });
        if modal.should_close() {
            close = true;
        }
        if delete {
            self.send(json!({"method":"delete","id":id}));
            close = true;
        }
        if close {
            self.confirm = None;
        }
    }

    fn footer(&mut self, ctx: &egui::Context, p: Palette) {
        egui::TopBottomPanel::bottom("status")
            .frame(
                egui::Frame::new()
                    .fill(p.bg)
                    .inner_margin(Margin::symmetric(24, 10)),
            )
            .show(ctx, |ui| {
                let r = ui.max_rect();
                ui.painter()
                    .hline(r.x_range(), r.top(), Stroke::new(1.0f32, p.line));
                ui.horizontal(|ui| {
                    ui.label(
                        Text::new(plural(self.rows.len(), "загрузка", "загрузки", "загрузок"))
                            .size(11.)
                            .color(p.muted),
                    );
                    ui.with_layout(egui::Layout::right_to_left(Align::Center), |ui| {
                        ui.label(Text::new("Отдача отключена").size(11.).color(p.muted));
                        ui.add_space(10.);
                        speed_badge(
                            ui,
                            self.rows.iter().map(|r| speed(r, "download_speed")).sum(),
                            p,
                        );
                    });
                });
            });
    }

    fn drop_overlay(&self, ctx: &egui::Context, p: Palette) {
        let hovered = ctx.input(|i| !i.raw.hovered_files.is_empty());
        if !hovered {
            return;
        }
        let painter = ctx.layer_painter(egui::LayerId::new(
            egui::Order::Foreground,
            egui::Id::new("drop-overlay"),
        ));
        let screen = ctx.screen_rect();
        painter.rect_filled(
            screen,
            0.,
            Color::from_black_alpha(if p.dark { 90 } else { 50 }),
        );
        let r = screen.shrink(18.);
        painter.rect_stroke(r, 18., Stroke::new(2.0f32, p.blue), StrokeKind::Inside);
        let c = screen.center();
        let ir = Rect::from_center_size(c + Vec2::new(0., -32.), Vec2::splat(56.));
        painter.rect_filled(ir, 16., tint(p.blue, 36));
        draw_icon(&painter, ir.shrink(16.), Ico::Download, p.blue);
        painter.text(
            c + Vec2::new(0., 24.),
            Align2::CENTER_CENTER,
            "Отпустите, чтобы добавить",
            bold(20.),
            p.text,
        );
        painter.text(
            c + Vec2::new(0., 52.),
            Align2::CENTER_CENTER,
            "magnet-ссылка или файл .torrent",
            FontId::proportional(13.),
            p.muted,
        );
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        if data_dir().join("runtime/stop").exists() {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        }
        if let Ok(Some(code)) = self.child.try_wait() {
            self.error = format!(
                "Движок завершился ({code}). Перезапустите приложение. Журнал: {}",
                data_dir().join("logs/worker.log").display()
            );
        }
        let any_dialog =
            self.adding || self.preferences || self.details.is_some() || self.confirm.is_some();
        if !any_dialog && ctx.input(|i| i.key_pressed(egui::Key::Escape)) && !self.search.is_empty()
        {
            self.search.clear();
        }
        if ctx.input(|i| i.modifiers.command && i.key_pressed(egui::Key::O)) {
            self.open_file();
        }
        if ctx.input(|i| i.modifiers.command && i.key_pressed(egui::Key::N)) {
            self.open_add();
        }
        if ctx.input(|i| i.modifiers.command && i.key_pressed(egui::Key::F)) {
            self.focus_search = true;
        }
        if let Ok(files) = std::fs::read_dir(data_dir().join("runtime/inbox")) {
            for entry in files.flatten() {
                if !self.adding {
                    if let Ok(bytes) = std::fs::read(entry.path()) {
                        if let Ok(target) = serde_json::from_slice::<String>(&bytes) {
                            self.source = target;
                            if !self.source.is_empty() {
                                self.open_add();
                            }
                            ctx.send_viewport_cmd(egui::ViewportCommand::Focus);
                            let _ = std::fs::remove_file(entry.path());
                        }
                    }
                }
            }
        }
        let dropped = ctx.input(|i| i.raw.dropped_files.clone());
        if let Some(file) = dropped.first() {
            let mut target = None;
            if let Some(path) = &file.path {
                if path
                    .extension()
                    .and_then(|e| e.to_str())
                    .map(|e| e.eq_ignore_ascii_case("torrent"))
                    .unwrap_or(false)
                {
                    target = Some(path.display().to_string());
                }
            }
            if target.is_none() {
                if let Some(bytes) = &file.bytes {
                    if let Ok(text) = std::str::from_utf8(bytes) {
                        let text = text.trim();
                        if text.starts_with("magnet:")
                            || text.starts_with("http://")
                            || text.starts_with("https://")
                        {
                            target = Some(text.to_owned());
                        }
                    }
                }
            }
            if let Some(target) = target {
                self.source = target;
                self.open_add();
                ctx.send_viewport_cmd(egui::ViewportCommand::Focus);
            }
        }
        while let Ok(v) = self.rx.try_recv() {
            if let Some(rows) = v["torrents"].as_array() {
                self.rows = rows.clone();
            }
            if let Some(e) = v["error"].as_str() {
                self.error = e.into();
                self.notice.clear();
            }
            if !v["details"].is_null() {
                self.details = Some(v["details"].clone());
            }
            if v["ok"] == true {
                self.notice.clear();
            }
        }
        if let Ok(output) = std::env::var("THODTORRENT_RENDER_CHECK") {
            self.preview_frames += 1;
            self.settings.dark = std::env::var("THODTORRENT_RENDER_THEME").as_deref() == Ok("dark");
            if std::env::var("THODTORRENT_RENDER_DIALOG").is_ok() {
                self.adding = true;
            }
            if std::env::var("THODTORRENT_RENDER_PREFS").is_ok() {
                self.preferences = true;
            }
            if std::env::var("THODTORRENT_RENDER_DETAILS").is_ok() {
                self.details = Some(json!({
                    "id": 0,
                    "name": "Ubuntu 24.04.3 Desktop amd64.iso",
                    "output_folder": "C:\\Users\\Thod\\Downloads",
                    "files": [
                        {"name": "ubuntu-24.04.3-desktop-amd64.iso", "length": 6000000000_f64},
                        {"name": "SHA256SUMS", "length": 4096_f64},
                        {"name": "README.txt", "length": 2048_f64},
                    ],
                }));
            }
            if std::env::var("THODTORRENT_RENDER_CONFIRM").is_ok() {
                self.confirm = Some(0);
            }
            if std::env::var("THODTORRENT_RENDER_ROWS").is_ok() {
                self.rows = vec![
                    json!({"id":0,"info_hash":"a1b2c3d4e5f60718293a4b5c6d7e8f9012345678","total_pieces":4128,"output_folder":"C:\\Users\\Thod\\Downloads","name":"Ubuntu 24.04.3 Desktop amd64.iso","stats":{"state":"live","total_bytes":6000000000_f64,"progress_bytes":3750000000_f64,"uploaded_bytes":0,"live":{"download_speed":{"mbps":12.8},"upload_speed":{"mbps":0.0},"time_remaining":{"human_readable":"02:48"}}}}),
                    json!({"id":1,"info_hash":"b1b2c3d4e5f60718293a4b5c6d7e8f9012345678","total_pieces":812,"output_folder":"C:\\Users\\Thod\\Downloads","name":"Архив фотографий — лето 2026","stats":{"state":"paused","finished":true,"total_bytes":1800000000_f64,"progress_bytes":1800000000_f64,"uploaded_bytes":0,"live":null}}),
                    json!({"id":2,"info_hash":"c1b2c3d4e5f60718293a4b5c6d7e8f9012345678","total_pieces":401,"output_folder":"D:\\Media","name":"Blender — открытые материалы","stats":{"state":"paused","total_bytes":980000000_f64,"progress_bytes":210000000_f64,"uploaded_bytes":0,"live":null}}),
                ];
            }
            if self.preview_frames == 5 {
                ctx.send_viewport_cmd(egui::ViewportCommand::Screenshot(Default::default()));
            }
            for event in ctx.input(|i| i.events.clone()) {
                if let egui::Event::Screenshot { image, .. } = event {
                    let bytes: Vec<u8> = image.pixels.iter().flat_map(|p| p.to_array()).collect();
                    let _ = image::save_buffer(
                        &output,
                        &bytes,
                        image.size[0] as u32,
                        image.size[1] as u32,
                        image::ColorType::Rgba8,
                    );
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                }
            }
        }
        let p = Palette::new(self.settings.dark);
        self.configure(ctx, p);
        self.sidebar(ctx, p);
        self.footer(ctx, p);
        egui::CentralPanel::default()
            .frame(
                egui::Frame::new()
                    .fill(p.bg)
                    .inner_margin(Margin::symmetric(28, 24)),
            )
            .show(ctx, |ui| {
                self.header(ui, p);
                ui.add_space(18.);
                if !self.error.is_empty() {
                    egui::Frame::new()
                        .fill(tint(p.red, if p.dark { 34 } else { 14 }))
                        .stroke(Stroke::new(
                            1.0f32,
                            tint(p.red, if p.dark { 80 } else { 42 }),
                        ))
                        .corner_radius(10)
                        .inner_margin(Margin::symmetric(12, 10))
                        .show(ui, |ui| {
                            ui.horizontal(|ui| {
                                icon(ui, Ico::Alert, p.red, 17.);
                                ui.add_space(2.);
                                ui.vertical(|ui| {
                                    ui.label(
                                        Text::new("Не удалось выполнить действие")
                                            .size(12.5)
                                            .font(bold(12.5))
                                            .color(p.text),
                                    );
                                    ui.collapsing("Технические сведения", |ui| {
                                        ui.label(Text::new(&self.error).size(11.5).color(p.muted));
                                    });
                                });
                                ui.with_layout(egui::Layout::right_to_left(Align::Center), |ui| {
                                    if button(ui, "Скрыть", None, Btn::Ghost, p, true).clicked()
                                    {
                                        self.error.clear();
                                    }
                                });
                            });
                        });
                    ui.add_space(12.);
                }
                if !self.notice.is_empty() {
                    ui.horizontal(|ui| {
                        ui.add(egui::Spinner::new().size(13.).color(p.blue));
                        ui.label(Text::new(&self.notice).size(12.).color(p.muted));
                    });
                    ui.add_space(10.);
                }
                self.table(ui, p);
            });
        self.dialogs(ctx, p);
        self.drop_overlay(ctx, p);
        ctx.request_repaint_after(Duration::from_millis(250));
    }
}
