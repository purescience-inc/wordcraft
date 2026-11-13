//! The Pure edition (PureWriter): everything that is ours, in one place.
//!
//! This fork builds two apps from one tree. Without the cargo feature `pure` it is upstream's
//! WordCraft, unchanged. With it, the visible names, the palette, the sizes and a few pieces of
//! chrome come from this module. Upstream files reach it through short guards
//! (`if crate::pure::ON { … }`), so both sides always compile and a new upstream release merges
//! with conflicts of a line or two.
//!
//! - Names: [`NAME`], [`app_name`], [`rename`], [`about_lines`].
//! - What the host owns: [`host_owns`], [`hides_tab`], [`hides_page`].
//! - Colours: the "Paper" palette ([`Paper`]), put into upstream's tokens by [`tokens`] and into
//!   egui's own widgets by [`style`].
//! - Measure and type: [`RADIUS`], [`radius`], [`type_size`].

use egui::{Color32, CornerRadius, Stroke};

use crate::theme::Tokens;

/// Whether this build is the Pure edition.
pub const ON: bool = cfg!(feature = "pure");

/// The app's visible name in the Pure edition.
pub const NAME: &str = "PureWriter (Craft)";

/// The name inside a sentence ("PureWriter keeps them when you save").
pub const SHORT_NAME: &str = "PureWriter";

/// The upstream app this edition is built on, as About credits it.
pub const UPSTREAM: &str = "WordCraft";

/// The name the interface shows for the app: ours in the Pure edition, upstream's otherwise.
pub fn app_name(upstream: &str) -> &str {
    if ON && upstream == UPSTREAM { NAME } else { upstream }
}

/// A sentence upstream wrote, with its product name replaced by ours in the Pure edition.
pub fn rename(text: &str) -> std::borrow::Cow<'_, str> {
    if ON && text.contains(UPSTREAM) { text.replace(UPSTREAM, SHORT_NAME).into() } else { text.into() }
}

/// What About says in the Pure edition: the name, what it is built on and where the licences are.
pub fn about_lines() -> [String; 3] {
    [
        NAME.to_string(),
        format!("Built on {UPSTREAM} {} ({}), © its contributors.", env!("CARGO_PKG_VERSION"), env!("CARGO_PKG_LICENSE").replace(" OR ", " or ")),
        "The licence texts, notices and asset attributions come with the app: LICENSE-MIT, LICENSE-APACHE, NOTICE and ATTRIBUTION.md.".to_string(),
    ]
}

/// Commands that are the host's in the Pure edition. New, Open, templates, Recover and Close
/// would put another document in the host's tab (or none), and the host would then save it over
/// the file the tab is bound to; Save As would hand the host the same file Save does under
/// another name; mail merge's Envelopes, Labels and Finish & Merge make new documents. A ribbon
/// button, a menu, a key or the File screen does not run them (`WordApp::run`); a caller of the
/// page command API (`window.wordcraft`, the host itself) still can.
pub fn host_owns(command: &str) -> bool {
    ON && matches!(
        command,
        "file.new"
            | "file.open"
            | "file.newFromTemplate"
            | "file.recover"
            | "file.close"
            | "file.closeDocument"
            | "file.saveAs"
            | "file.saveTemplate"
            | "file.autosave"
            | "view.newWindow"
            | "mailings.envelopes"
            | "mailings.labels"
            | "mailings.finish"
            | "tools.dictate"
            | "ui.discord"
            | "ui.checkForUpdates"
    ) || (ON && command.starts_with("ui.zotero."))
}

/// Ribbon tabs the Pure edition leaves out: Zotero needs the Zotero desktop program and an
/// account; Mailings makes new documents the host cannot bind.
pub fn hides_tab(tab: &str) -> bool {
    ON && matches!(tab, "Zotero" | "Mailings")
}

/// File screen pages the Pure edition leaves out (the host's, see [`host_owns`]).
pub fn hides_page(page: &str) -> bool {
    ON && matches!(page, "home" | "new" | "open" | "recover" | "saveAs" | "close")
}

/// The page shown for `page`: itself, or Info for one the Pure edition leaves out.
pub fn page(page: &str) -> &str {
    if hides_page(page) { "info" } else { page }
}

// ---------------------------------------------------------------- measure

/// Corners of buttons, inputs, menus and dialogs. Bars and panels have none.
pub const RADIUS: f32 = 3.0;

/// A corner radius upstream wrote for a button, input or menu: ours in the Pure edition.
pub fn radius(upstream: f32) -> f32 {
    if ON { RADIUS } else { upstream }
}

/// The type scale, in whole sizes: labels and the status bar; ribbon controls and inputs; ribbon
/// tabs, menus and the File side list; the document title; File screen headings.
pub const TYPE_LABEL: f32 = 11.0;
pub const TYPE_CONTROL: f32 = 12.0;
pub const TYPE_TAB: f32 = 13.0;
pub const TYPE_TITLE: f32 = 14.0;
pub const TYPE_HEADING: f32 = 20.0;

/// A chrome text size upstream wrote, moved onto our type scale in the Pure edition.
pub fn type_size(upstream: f32) -> f32 {
    if !ON {
        upstream
    } else if upstream < 11.75 {
        TYPE_LABEL
    } else if upstream < 12.75 {
        TYPE_CONTROL
    } else if upstream < 13.75 {
        TYPE_TAB
    } else if upstream < 17.0 {
        TYPE_TITLE
    } else {
        TYPE_HEADING
    }
}

/// The size of a large ribbon icon's box (upstream 32) and a small one's (upstream 17).
pub fn big_icon(upstream: f32) -> f32 {
    if ON { 28.0 } else { upstream }
}
pub fn small_icon(upstream: f32) -> f32 {
    if ON { 16.0 } else { upstream }
}

// ---------------------------------------------------------------- colour

/// PureDesktop's "Paper" palette, one value per role.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Paper {
    pub frame: Color32,
    pub surface: Color32,
    pub popup: Color32,
    pub canvas: Color32,
    pub rule: Color32,
    pub line: Color32,
    pub strong_line: Color32,
    pub text: Color32,
    pub text_2: Color32,
    pub disabled: Color32,
    pub hover: Color32,
    pub pressed: Color32,
    pub on_fill: Color32,
    pub on_mark: Color32,
    pub accent: Color32,
    pub accent_hover: Color32,
    pub on_accent: Color32,
    pub selection: Color32,
    pub danger: Color32,
}

const fn rgb(hex: u32) -> Color32 {
    Color32::from_rgb((hex >> 16) as u8, (hex >> 8) as u8, hex as u8)
}

pub const LIGHT: Paper = Paper {
    frame: rgb(0xF7F8F9),
    surface: rgb(0xFFFFFF),
    popup: rgb(0xFFFFFF),
    canvas: rgb(0xE9EBEE),
    rule: rgb(0xE3E5E9),
    line: rgb(0xCDD1D7),
    strong_line: rgb(0xAAB0B9),
    text: rgb(0x1F2328),
    text_2: rgb(0x4F5661),
    disabled: rgb(0x9AA1AB),
    hover: rgb(0xF0F2F4),
    pressed: rgb(0xE3E5E9),
    on_fill: rgb(0xE9EFF9),
    on_mark: rgb(0x1F3A5F),
    accent: rgb(0x1F3A5F),
    accent_hover: rgb(0x172D4B),
    on_accent: rgb(0xFFFFFF),
    selection: rgb(0xCFDCEE),
    danger: rgb(0xA8291F),
};

pub const DARK: Paper = Paper {
    frame: rgb(0x1B1F23),
    surface: rgb(0x1F2328),
    popup: rgb(0x262B31),
    canvas: rgb(0x16191D),
    rule: rgb(0x2D3238),
    line: rgb(0x3A4047),
    strong_line: rgb(0x555C65),
    text: rgb(0xE7E9EC),
    text_2: rgb(0xB3B9C2),
    disabled: rgb(0x6B727C),
    hover: rgb(0x2A2F35),
    pressed: rgb(0x32383F),
    on_fill: rgb(0x26364B),
    on_mark: rgb(0x9DBBE4),
    accent: rgb(0x9DBBE4),
    accent_hover: rgb(0xB7CDEC),
    on_accent: rgb(0x16191D),
    selection: rgb(0x34506F),
    danger: rgb(0xF0968D),
};

/// The palette for a theme.
pub const fn paper(dark: bool) -> &'static Paper {
    if dark { &DARK } else { &LIGHT }
}

/// Upstream's tokens with every chrome colour mapped to its Paper role (the Pure edition), or as
/// they are. What is drawn on the page (selection, guides, handles, balloons' author colours,
/// the ruler) stays upstream's: the document canvas is not redesigned.
pub fn tokens(mut t: Tokens) -> Tokens {
    if !ON {
        return t;
    }
    let p = paper(t.dark);
    t.title_bar = p.frame;
    t.ribbon = p.surface;
    t.tab_strip = p.frame;
    t.canvas = p.canvas;
    t.panel = p.surface;
    t.border = p.rule;
    t.border_strong = p.line;
    t.text = p.text;
    t.text_dim = p.text_2;
    t.text_disabled = p.disabled;
    t.icon = p.text_2;
    t.hover = p.hover;
    t.pressed = p.pressed;
    t.checked = p.on_fill;
    t.accent = p.accent;
    t.nav = p.frame;
    t.accent_text = p.accent;
    t.on_accent = p.on_accent;
    t.input = if t.dark { p.canvas } else { p.surface };
    t.input_border = p.line;
    t.group_label = p.text_2;
    t.status_bar = p.frame;
    t.menu = p.popup;
    t.markup_area = if t.dark { p.frame } else { p.frame };
    t.markup_edge = p.rule;
    t.balloon = if t.dark { p.popup } else { p.surface };
    t
}

/// After upstream's `theme::apply`: egui's own widgets (buttons, fields, menus, dialogs) take the
/// Pure edition's corners, lines and states, the text styles its type scale, and nothing moves
/// for its own sake (the animation setting's default is off).
pub fn style(ctx: &egui::Context) {
    if !ON {
        return;
    }
    let corner = CornerRadius::same(RADIUS as u8);
    for theme in [egui::Theme::Light, egui::Theme::Dark] {
        let p = paper(theme == egui::Theme::Dark);
        ctx.style_mut_of(theme, |s| {
            let v = &mut s.visuals;
            v.window_stroke = Stroke::new(1.0, p.line);
            v.window_corner_radius = corner;
            v.menu_corner_radius = corner;
            v.hyperlink_color = p.accent;
            v.error_fg_color = p.danger;
            let lift = if v.dark_mode { Color32::from_black_alpha(90) } else { Color32::from_black_alpha(28) };
            v.popup_shadow = egui::epaint::Shadow { offset: [0, 2], blur: 8, spread: 0, color: lift };
            v.window_shadow = egui::epaint::Shadow { offset: [0, 4], blur: 16, spread: 0, color: lift };
            let w = &mut v.widgets;
            for state in [&mut w.noninteractive, &mut w.inactive, &mut w.hovered, &mut w.active, &mut w.open] {
                state.corner_radius = corner;
                state.expansion = 0.0;
            }
            w.inactive.bg_stroke = Stroke::new(1.0, p.line);
            w.hovered.bg_stroke = Stroke::new(1.0, p.strong_line);
            w.active.bg_stroke = Stroke::new(1.0, p.strong_line);
            w.open.bg_stroke = Stroke::new(1.0, p.strong_line);
        });
    }
    ctx.all_styles_mut(|s| {
        s.text_styles.insert(egui::TextStyle::Body, crate::theme::regular(TYPE_TAB));
        s.text_styles.insert(egui::TextStyle::Button, crate::theme::regular(TYPE_TAB));
        s.text_styles.insert(egui::TextStyle::Small, crate::theme::regular(TYPE_LABEL));
        s.text_styles.insert(egui::TextStyle::Heading, crate::theme::semibold(TYPE_HEADING));
    });
}

// ---------------------------------------------------------------- the File side list

/// The File side list's text and icons: white on upstream's blue bar, our text on the frame.
pub fn nav_ink(t: &Tokens, upstream: Color32) -> Color32 {
    if ON { t.text } else { upstream }
}

/// A greyed-out entry's ink (`alpha` is upstream's white alpha).
pub fn nav_ink_off(t: &Tokens, alpha: u8) -> Color32 {
    if ON { t.text_disabled } else { Color32::from_white_alpha(alpha) }
}

/// The chosen entry's fill (upstream: white at `alpha` over its blue bar).
pub fn nav_chosen(t: &Tokens, alpha: u8) -> Color32 {
    if ON { t.checked } else { Color32::from_white_alpha(alpha) }
}

/// A hovered entry's fill.
pub fn nav_hover(t: &Tokens, alpha: u8) -> Color32 {
    if ON { t.hover } else { Color32::from_white_alpha(alpha) }
}

/// The 2px accent bar at the leading edge of the chosen entry, and the line at the list's far edge.
pub fn nav_marks(ui: &egui::Ui, row: egui::Rect, chosen: bool, rtl: bool) {
    if !ON || !chosen {
        return;
    }
    let t = Tokens::get(ui.ctx());
    let x = if rtl { row.max.x - 2.0 } else { row.min.x };
    ui.painter().rect_filled(egui::Rect::from_min_size(egui::pos2(x, row.min.y), egui::vec2(2.0, row.height())), 0.0, t.accent);
}

// ---------------------------------------------------------------- the tab strip

/// A ribbon tab's label colour: the active one in text colour (its rule is the accent), the
/// others secondary, a contextual one (Table Design, Picture Format) in the accent.
pub fn tab_color(t: &Tokens, active: bool, contextual: bool, upstream: Color32) -> Color32 {
    if !ON {
        upstream
    } else if active {
        t.text
    } else if contextual {
        t.accent
    } else {
        t.text_dim
    }
}

/// Share's fill and ink: upstream's filled accent button, or (Pure edition) an outlined one.
pub fn share_colors(t: &Tokens, resp: &egui::Response, pure_share: bool) -> (Color32, Color32) {
    if !ON {
        (if resp.hovered() { t.accent_text } else { t.accent }, t.on_accent)
    } else if pure_share {
        (Color32::TRANSPARENT, t.text)
    } else {
        (Color32::TRANSPARENT, Color32::TRANSPARENT)
    }
}

/// A flat outlined button's frame: no fill at rest, the hover or pressed fill, a 1px line (the
/// strong line while hovered), 3px corners.
pub fn outlined(ui: &egui::Ui, r: egui::Rect, resp: &egui::Response, t: &Tokens, shown: bool) {
    if !shown {
        return;
    }
    let fill = if resp.is_pointer_button_down_on() {
        t.pressed
    } else if resp.hovered() {
        t.hover
    } else {
        Color32::TRANSPARENT
    };
    let line = if resp.hovered() { paper(t.dark).strong_line } else { t.border_strong };
    ui.painter().rect_filled(r, RADIUS, fill);
    ui.painter().rect_stroke(r, RADIUS, Stroke::new(1.0, line), egui::StrokeKind::Inside);
    if resp.has_focus() {
        ui.painter().rect_stroke(r.expand(1.0), RADIUS, Stroke::new(1.0, t.accent), egui::StrokeKind::Outside);
    }
}

// ---------------------------------------------------------------- the host

impl crate::WordApp {
    /// Share: tells the host that the person pressed Share on this document. Sharing is the
    /// host's; the app shows nothing itself, and without a host this does nothing.
    pub fn host_share(&self) {
        let Some(share) = &self.services.host_share else { return };
        share(&self.title_stem());
    }
}

/// The line at the File side list's far edge (Pure edition), between it and the page.
pub fn nav_edge(ui: &egui::Ui, rtl: bool) {
    if !ON {
        return;
    }
    let t = Tokens::get(ui.ctx());
    let r = ui.max_rect();
    let x = if rtl { r.min.x + 0.5 } else { r.max.x - 0.5 };
    ui.painter().line_segment([egui::pos2(x, r.min.y - 40.0), egui::pos2(x, r.max.y + 40.0)], Stroke::new(1.0, t.border_strong));
}

/// A contributor's name in the credits: a link to their GitHub profile upstream, plain text in
/// the Pure edition, which links to no outside site.
pub fn credit(ui: &mut egui::Ui, name: impl Into<egui::WidgetText>, url: String) -> egui::Response {
    if ON { ui.label(name) } else { ui.hyperlink_to(name, url) }
}

/// A ribbon button's label size: 12 in the Pure edition (upstream 11.5).
pub fn ribbon_label(upstream: f32) -> f32 {
    if ON { TYPE_CONTROL } else { upstream }
}

// ---------------------------------------------------------------- dialogs

/// A dialog's header: the title at the leading edge, a close button at the far edge, one rule
/// under them. Upstream's title bar puts the title at 20px in the middle of a grey band.
pub const DIALOG_HEADER: f32 = 44.0;

/// The space around a dialog's content.
pub const DIALOG_MARGIN: i8 = 16;

/// A dialog's OK and Cancel buttons at their least.
pub const DIALOG_BUTTON: egui::Vec2 = egui::vec2(76.0, 28.0);

/// A dialog's frame in the Pure edition: white (the popup colour in dark), one line, 3px corners,
/// no margin of its own (the header and the body keep theirs).
pub fn dialog_frame(style: &egui::Style) -> egui::Frame {
    egui::Frame::window(style).fill(paper(style.visuals.dark_mode).popup).inner_margin(0)
}

/// How far below the window's middle a dialog is centred on its first frames: 40 points, so it sits
/// over the page rather than the ribbon (upstream lifts it 40 above), but never so far that it
/// leaves the window at either edge; a dialog taller than the window keeps its top (and its title)
/// in view.
pub fn dialog_offset(ctx: &egui::Context, id: egui::Id) -> f32 {
    let screen = ctx.content_rect().height();
    let Some(r) = ctx.memory(|m| m.area_rect(id)) else { return 0.0 };
    let room = (screen - r.height()) / 2.0;
    let (lo, hi) = (8.0 - room, room - 8.0);
    if lo > hi { lo } else { 40.0f32.clamp(lo, hi) }
}

/// Inside a dialog's body: 24px control rows on the 4px step, and room between them.
pub fn dialog_spacing(ui: &mut egui::Ui) {
    let s = ui.spacing_mut();
    s.interact_size.y = 24.0;
    s.item_spacing = egui::vec2(8.0, 6.0);
    s.button_padding = egui::vec2(10.0, 4.0);
}

/// Paints a dialog's header into `rect` (the full width of the dialog, [`DIALOG_HEADER`] tall)
/// and handles its close button. True when the close button was pressed.
pub fn dialog_header(ui: &mut egui::Ui, rect: egui::Rect, title: &str, rtl: bool) -> bool {
    let p = paper(ui.visuals().dark_mode);
    let painter = ui.painter();
    let galley = painter.layout_no_wrap(title.to_string(), crate::theme::semibold(TYPE_TITLE), p.text);
    let inset = f32::from(DIALOG_MARGIN);
    let x = if rtl { rect.max.x - inset - galley.size().x } else { rect.min.x + inset };
    painter.galley(egui::pos2(x, rect.center().y - galley.size().y / 2.0), galley, p.text);
    // The close button: 28 points square, 8 in from the far edge; a quiet fill under the pointer.
    let side = 28.0;
    let cx = if rtl { rect.min.x + 8.0 + side / 2.0 } else { rect.max.x - 8.0 - side / 2.0 };
    let close = egui::Rect::from_center_size(egui::pos2(cx, rect.center().y), egui::vec2(side, side));
    let r = ui.interact(close, ui.id().with("pure_dialog_close"), egui::Sense::click());
    let label = tl!("Close");
    r.widget_info(|| egui::WidgetInfo::labeled(egui::Role::Button, true, label));
    let r = r.on_hover_text(label);
    let painter = ui.painter();
    if r.is_pointer_button_down_on() {
        painter.rect_filled(close, CornerRadius::same(RADIUS as u8), p.pressed);
    } else if r.hovered() {
        painter.rect_filled(close, CornerRadius::same(RADIUS as u8), p.hover);
    }
    let c = egui::pos2(painter.round_to_pixel_center(close.center().x), painter.round_to_pixel_center(close.center().y));
    let s = Stroke::new(1.25, if r.hovered() { p.text } else { p.text_2 });
    let g = 5.0;
    painter.line_segment([c + egui::vec2(-g, -g), c + egui::vec2(g, g)], s);
    painter.line_segment([c + egui::vec2(-g, g), c + egui::vec2(g, -g)], s);
    let y = painter.round_to_pixel_center(rect.max.y - 0.5);
    painter.line_segment([egui::pos2(rect.min.x, y), egui::pos2(rect.max.x, y)], Stroke::new(1.0, p.rule));
    r.clicked()
}

/// Before a dialog's OK and Cancel: room, and one rule the full width of the dialog.
pub fn dialog_footer_rule(ui: &mut egui::Ui) {
    let p = paper(ui.visuals().dark_mode);
    ui.add_space(12.0);
    let inset = f32::from(DIALOG_MARGIN);
    let r = ui.max_rect();
    let y = ui.painter().round_to_pixel_center(ui.cursor().min.y);
    ui.painter().line_segment([egui::pos2(r.min.x - inset, y), egui::pos2(r.max.x + inset, y)], Stroke::new(1.0, p.rule));
    ui.add_space(12.0);
}

/// A row of tabs in a dialog: the labels at 13px, the chosen one in the text colour, semibold, on
/// a 2px accent rule; the others in the secondary colour, a quiet fill under the pointer; one rule
/// under the row. Upstream draws raised, rounded, grey tabs. Returns the tab clicked.
pub fn dialog_tabs(ui: &mut egui::Ui, labels: &[String], selected: usize) -> Option<usize> {
    const H: f32 = 30.0;
    const PAD: f32 = 12.0;
    let p = paper(ui.visuals().dark_mode);
    let font = |on: bool| if on { crate::theme::semibold(TYPE_TAB) } else { crate::theme::regular(TYPE_TAB) };
    let need: f32 = labels.iter().map(|l| ui.painter().layout_no_wrap(l.clone(), font(true), p.text).size().x + 2.0 * PAD).sum();
    ui.set_min_width(need);
    let origin = ui.cursor().min;
    let width = ui.available_width();
    let rtl = crate::rtl::flows(ui);
    let mut x = if rtl { origin.x + width } else { origin.x };
    let mut clicked = None;
    for (i, label) in labels.iter().enumerate() {
        let on = i == selected;
        // Width from the semibold label, so a tab doesn't change width as it is chosen.
        let w = ui.painter().layout_no_wrap(label.clone(), font(true), p.text).size().x + 2.0 * PAD;
        let left = if rtl { x - w } else { x };
        let r = egui::Rect::from_min_size(egui::pos2(left, origin.y), egui::vec2(w, H));
        let resp = ui.interact(r, ui.scope_id().with(("pure_dialog_tab", i)), egui::Sense::click());
        resp.widget_info(|| egui::WidgetInfo::selected(egui::Role::Button, true, on, label));
        let painter = ui.painter();
        if !on && resp.hovered() {
            painter.rect_filled(r.shrink2(egui::vec2(0.0, 3.0)), CornerRadius::same(RADIUS as u8), p.hover);
        }
        let ink = if on || resp.hovered() { p.text } else { p.text_2 };
        let galley = painter.layout_no_wrap(label.clone(), font(on), ink);
        painter.galley(egui::pos2(r.center().x - galley.size().x / 2.0, r.center().y - galley.size().y / 2.0 - 1.0), galley, ink);
        if on {
            painter.rect_filled(egui::Rect::from_min_max(egui::pos2(r.min.x + 4.0, r.max.y - 2.0), egui::pos2(r.max.x - 4.0, r.max.y)), 0.0, p.accent);
        }
        if resp.clicked() && !on {
            clicked = Some(i);
        }
        x += if rtl { -w } else { w };
    }
    ui.allocate_space(egui::vec2(width, H));
    let y = ui.painter().round_to_pixel_center(origin.y + H - 0.5);
    ui.painter().line_segment([egui::pos2(origin.x, y), egui::pos2(origin.x + width, y)], Stroke::new(1.0, p.rule));
    ui.add_space(4.0);
    clicked
}
