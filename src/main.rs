#![allow(unused, dead_code)]
use chrono::Timelike;
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers, MouseButton, MouseEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph, Wrap},
    Frame, Terminal,
};
use tui_big_text::{BigText, PixelSize};
use std::{
    io,
    time::{Duration, Instant},
};

// ─────────────────────────────────────────────────────────────────────────────
// Themes
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Clone, Copy, Debug)]
struct Theme {
    name: &'static str,
    bg: Color,
    slab_bg: Color,
    card_top_bg: Color,
    card_bottom_bg: Color,
    card_border: Color,
    card_border_dim: Color,
    text_main: Color,
    text_dim: Color,
    text_muted: Color,
    accent_red: Color,
    accent_green: Color,
    accent_blue: Color,
    accent_yellow: Color,
    popup_bg: Color,
    transparent: bool,
}

const THEME_SYSTEM: Theme = Theme {
    name: "System (terminal)",
    // Reset = inherit terminal bg/fg + opacity (Ghostty theme shows through)
    bg: Color::Reset,
    slab_bg: Color::Reset,
    card_top_bg: Color::Reset,
    card_bottom_bg: Color::Reset,
    card_border: Color::DarkGray,
    card_border_dim: Color::DarkGray,
    text_main: Color::Reset,
    text_dim: Color::Gray,
    text_muted: Color::DarkGray,
    accent_red: Color::LightRed,
    accent_green: Color::LightGreen,
    accent_blue: Color::LightBlue,
    accent_yellow: Color::LightYellow,
    popup_bg: Color::Reset,
    transparent: true,
};

const THEME_EVERFOREST: Theme = Theme {
    name: "Everforest",
    bg: Color::Rgb(43, 51, 57),          // #2b3339
    slab_bg: Color::Rgb(35, 42, 46),     // darker slab
    card_top_bg: Color::Rgb(54, 62, 68),
    card_bottom_bg: Color::Rgb(60, 69, 75),
    card_border: Color::Rgb(93, 106, 107),
    card_border_dim: Color::Rgb(58, 66, 67),
    text_main: Color::Rgb(211, 198, 170), // #d3c6aa
    text_dim: Color::Rgb(152, 160, 140),
    text_muted: Color::Rgb(122, 132, 120),
    accent_red: Color::Rgb(230, 126, 128),   // #e67e80
    accent_green: Color::Rgb(167, 192, 128), // #a7c080
    accent_blue: Color::Rgb(127, 187, 179),  // #7fbbb3
    accent_yellow: Color::Rgb(219, 188, 127),// #dbbc7f
    popup_bg: Color::Rgb(35, 42, 46),
    transparent: false,
};

const THEME_CATP_MOCHA: Theme = Theme {
    name: "Catppuccin Mocha",
    bg: Color::Rgb(30, 30, 46),          // base #1e1e2e
    slab_bg: Color::Rgb(24, 24, 37),     // mantle
    card_top_bg: Color::Rgb(49, 50, 68), // surface0
    card_bottom_bg: Color::Rgb(56, 57, 76),
    card_border: Color::Rgb(88, 91, 112),// surface2
    card_border_dim: Color::Rgb(49, 50, 68),
    text_main: Color::Rgb(205, 214, 244),// text
    text_dim: Color::Rgb(166, 173, 200), // subtext
    text_muted: Color::Rgb(127, 132, 156),// overlay
    accent_red: Color::Rgb(243, 139, 168),  // red
    accent_green: Color::Rgb(166, 227, 161),// green
    accent_blue: Color::Rgb(137, 180, 250), // blue
    accent_yellow: Color::Rgb(249, 226, 175),// yellow
    popup_bg: Color::Rgb(24, 24, 37),
    transparent: false,
};

const THEME_TOKYONIGHT: Theme = Theme {
    name: "Tokyonight Night",
    bg: Color::Rgb(26, 27, 38),
    slab_bg: Color::Rgb(22, 22, 30),
    card_top_bg: Color::Rgb(36, 40, 59),
    card_bottom_bg: Color::Rgb(42, 46, 67),
    card_border: Color::Rgb(59, 66, 97),
    card_border_dim: Color::Rgb(36, 40, 59),
    text_main: Color::Rgb(192, 202, 245),
    text_dim: Color::Rgb(137, 144, 178),
    text_muted: Color::Rgb(86, 95, 137),
    accent_red: Color::Rgb(247, 118, 142),
    accent_green: Color::Rgb(158, 206, 106),
    accent_blue: Color::Rgb(122, 162, 247),
    accent_yellow: Color::Rgb(224, 175, 104),
    popup_bg: Color::Rgb(22, 22, 30),
    transparent: false,
};

const THEME_GRUVBOX: Theme = Theme {
    name: "Gruvbox Dark",
    bg: Color::Rgb(40, 40, 40),
    slab_bg: Color::Rgb(29, 32, 33),
    card_top_bg: Color::Rgb(60, 56, 54),
    card_bottom_bg: Color::Rgb(66, 62, 59),
    card_border: Color::Rgb(102, 92, 84),
    card_border_dim: Color::Rgb(60, 56, 54),
    text_main: Color::Rgb(235, 219, 178),
    text_dim: Color::Rgb(189, 174, 147),
    text_muted: Color::Rgb(146, 131, 116),
    accent_red: Color::Rgb(251, 73, 52),
    accent_green: Color::Rgb(184, 187, 38),
    accent_blue: Color::Rgb(131, 165, 152),
    accent_yellow: Color::Rgb(250, 189, 47),
    popup_bg: Color::Rgb(29, 32, 33),
    transparent: false,
};

const THEME_ROSE_PINE: Theme = Theme {
    name: "Rosé Pine",
    bg: Color::Rgb(25, 23, 36),          // base
    slab_bg: Color::Rgb(31, 29, 46),     // surface
    card_top_bg: Color::Rgb(38, 35, 58), // overlay
    card_bottom_bg: Color::Rgb(43, 40, 64),
    card_border: Color::Rgb(64, 61, 82),
    card_border_dim: Color::Rgb(38, 35, 58),
    text_main: Color::Rgb(224, 222, 244),// text
    text_dim: Color::Rgb(144, 140, 170), // subtle
    text_muted: Color::Rgb(110, 106, 134),// muted
    accent_red: Color::Rgb(235, 111, 146),  // love
    accent_green: Color::Rgb(156, 207, 216),// foam
    accent_blue: Color::Rgb(196, 167, 231), // iris
    accent_yellow: Color::Rgb(246, 193, 119),// gold
    popup_bg: Color::Rgb(31, 29, 46),
    transparent: false,
};

const THEME_TERAFOX: Theme = Theme {
    name: "Terafox",
    bg: Color::Rgb(15, 28, 30),
    slab_bg: Color::Rgb(16, 33, 36),
    card_top_bg: Color::Rgb(27, 46, 49),
    card_bottom_bg: Color::Rgb(32, 53, 56),
    card_border: Color::Rgb(67, 92, 96),
    card_border_dim: Color::Rgb(35, 56, 60),
    text_main: Color::Rgb(234, 238, 238),
    text_dim: Color::Rgb(170, 185, 185),
    text_muted: Color::Rgb(113, 131, 133),
    accent_red: Color::Rgb(232, 92, 81),
    accent_green: Color::Rgb(122, 181, 110),
    accent_blue: Color::Rgb(108, 170, 216),
    accent_yellow: Color::Rgb(231, 203, 123),
    popup_bg: Color::Rgb(16, 33, 36),
    transparent: false,
};

const THEMES: &[Theme] = &[
    THEME_SYSTEM,
    THEME_EVERFOREST,
    THEME_CATP_MOCHA,
    THEME_TOKYONIGHT,
    THEME_GRUVBOX,
    THEME_ROSE_PINE,
    THEME_TERAFOX,
];

// ─────────────────────────────────────────────────────────────────────────────
// Modes
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum AppMode {
    Clock,
    Pomodoro,
    Timer,
    Stopwatch,
}
impl AppMode {
    fn next(self) -> Self {
        match self {
            Self::Clock => Self::Pomodoro,
            Self::Pomodoro => Self::Timer,
            Self::Timer => Self::Stopwatch,
            Self::Stopwatch => Self::Clock,
        }
    }
    fn prev(self) -> Self {
        match self {
            Self::Clock => Self::Stopwatch,
            Self::Pomodoro => Self::Clock,
            Self::Timer => Self::Pomodoro,
            Self::Stopwatch => Self::Timer,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum PomodoroPhase {
    Work,
    ShortBreak,
    LongBreak,
}
impl PomodoroPhase {
    fn label(self) -> &'static str {
        match self {
            Self::Work => "FOCUS",
            Self::ShortBreak => "SHORT BREAK",
            Self::LongBreak => "LONG BREAK",
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Click actions (mouse)
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum ClickAction {
    Mode(AppMode),
    StartPause,
    Reset,
    Skip,
    Lap,
    TimerEdit,
    TimerSave,
    TimerCancel,
    ToggleSettings,
    SettingsField(SettingsField),
    ThemePrev,
    ThemeNext,
    SettingsClose,
    Toggle24h,
    ToggleSeconds,
    ToggleTransparent,
}

// ─────────────────────────────────────────────────────────────────────────────
// Flip digit
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Clone, Copy, Debug)]
struct FlipState {
    current: char,
    previous: char,
    progress: f32,
    animating: bool,
}
impl FlipState {
    fn new(c: char) -> Self {
        Self { current: c, previous: c, progress: 1.0, animating: false }
    }
    fn set(&mut self, new_c: char) {
        if new_c != self.current {
            self.previous = self.current;
            self.current = new_c;
            self.progress = 0.0;
            self.animating = true;
        }
    }
    fn tick(&mut self, dt: f32) {
        if self.animating {
            self.progress += dt * 7.0;
            if self.progress >= 1.0 {
                self.progress = 1.0;
                self.animating = false;
            }
        }
    }
    fn eased(&self) -> f32 {
        let t = self.progress.clamp(0.0, 1.0);
        if t < 0.5 { 4.0*t*t*t } else { 1.0 - (-2.0*t+2.0).powi(3)/2.0 }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Sub states
// ─────────────────────────────────────────────────────────────────────────────

struct PomodoroState {
    work_secs: u32,
    short_secs: u32,
    long_secs: u32,
    sessions_before_long: u32,
    phase: PomodoroPhase,
    remaining: Duration,
    running: bool,
    completed: u32,
    session_in_cycle: u32,
    auto_start_breaks: bool,
    auto_start_work: bool,
}
impl PomodoroState {
    fn new() -> Self {
        Self {
            work_secs: 25*60, short_secs: 5*60, long_secs: 15*60,
            sessions_before_long: 4,
            phase: PomodoroPhase::Work,
            remaining: Duration::from_secs(25*60),
            running: false, completed: 0, session_in_cycle: 1,
            auto_start_breaks: true, auto_start_work: false,
        }
    }
    fn phase_duration(&self) -> Duration {
        match self.phase {
            PomodoroPhase::Work => Duration::from_secs(self.work_secs as u64),
            PomodoroPhase::ShortBreak => Duration::from_secs(self.short_secs as u64),
            PomodoroPhase::LongBreak => Duration::from_secs(self.long_secs as u64),
        }
    }
    fn phase_color(&self, th: &Theme) -> Color {
        match self.phase {
            PomodoroPhase::Work => th.accent_red,
            PomodoroPhase::ShortBreak => th.accent_green,
            PomodoroPhase::LongBreak => th.accent_blue,
        }
    }
    fn reset_current(&mut self) { self.remaining = self.phase_duration(); self.running=false; }
    fn advance(&mut self) {
        match self.phase {
            PomodoroPhase::Work => {
                self.completed+=1;
                if self.session_in_cycle >= self.sessions_before_long {
                    self.phase=PomodoroPhase::LongBreak; self.session_in_cycle=1;
                } else { self.phase=PomodoroPhase::ShortBreak; self.session_in_cycle+=1; }
            }
            _ => { self.phase=PomodoroPhase::Work; }
        }
        self.remaining=self.phase_duration();
    }
}

struct TimerState {
    set_secs: u32,
    remaining: Duration,
    running: bool,
    finished: bool,
}
impl TimerState {
    fn new() -> Self { let s=5*60; Self{set_secs:s, remaining:Duration::from_secs(s as u64), running:false, finished:false} }
    fn sync(&mut self){ if !self.running { self.remaining=Duration::from_secs(self.set_secs as u64); self.finished=false; } }
}

struct StopwatchState {
    elapsed: Duration,
    running: bool,
    laps: Vec<Duration>,
}
impl StopwatchState { fn new()->Self{ Self{elapsed:Duration::ZERO, running:false, laps:Vec::new()} } }

// ─────────────────────────────────────────────────────────────────────────────
// Settings cursor
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum SettingsField { Theme, Transparent, Work, Short, Long, Cycles, AutoBreak, AutoWork }
impl SettingsField {
    fn next(self)->Self{
        match self{Self::Theme=>Self::Transparent, Self::Transparent=>Self::Work, Self::Work=>Self::Short, Self::Short=>Self::Long, Self::Long=>Self::Cycles, Self::Cycles=>Self::AutoBreak, Self::AutoBreak=>Self::AutoWork, Self::AutoWork=>Self::Theme}
    }
    fn prev(self)->Self{
        match self{Self::Theme=>Self::AutoWork, Self::Transparent=>Self::Theme, Self::Work=>Self::Transparent, Self::Short=>Self::Work, Self::Long=>Self::Short, Self::Cycles=>Self::Long, Self::AutoBreak=>Self::Cycles, Self::AutoWork=>Self::AutoBreak}
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// App
// ─────────────────────────────────────────────────────────────────────────────

struct App {
    mode: AppMode,
    pomodoro: PomodoroState,
    timer: TimerState,
    stopwatch: StopwatchState,
    use_24h: bool,
    show_seconds: bool,
    show_settings: bool,
    settings_field: SettingsField,
    flip_states: Vec<FlipState>,
    colon_blink: bool,
    colon_timer: f32,
    flash_timer: f32,
    should_quit: bool,
    info_msg: Option<(String, f32)>,
    theme_idx: usize,
    transparent_bg: bool,
    mouse_enabled: bool,
    clickables: Vec<(Rect, ClickAction)>,
    timer_editing: bool,
    timer_buffer: String,
}

impl Default for App {
    fn default()->Self{
        Self{
            mode: AppMode::Clock,
            pomodoro: PomodoroState::new(),
            timer: TimerState::new(),
            stopwatch: StopwatchState::new(),
            use_24h: true, show_seconds: true,
            show_settings: false, settings_field: SettingsField::Theme,
            flip_states: vec![FlipState::new('0'); 8],
            colon_blink: true, colon_timer: 0.0, flash_timer:0.0,
            should_quit:false,
            info_msg: None,
            theme_idx: 0, // System by default = inherits Ghostty
            transparent_bg: false,
            mouse_enabled: true,
            clickables: Vec::new(),
            timer_editing: false,
            timer_buffer: String::new(),
        }
    }
}

impl App {
    /// Resolve active theme. When `transparent_bg` is on, all background
    /// fills become `Color::Reset` so any theme inherits the terminal
    /// background + opacity (Ghostty etc). Text/accent colors are untouched.
    fn theme(&self) -> Theme {
        let mut th = THEMES[self.theme_idx % THEMES.len()];
        if self.transparent_bg {
            th.bg = Color::Reset;
            th.slab_bg = Color::Reset;
            th.card_top_bg = Color::Reset;
            th.card_bottom_bg = Color::Reset;
            th.popup_bg = Color::Reset;
        }
        th
    }
    fn base_theme(&self) -> Theme { THEMES[self.theme_idx % THEMES.len()] }
    fn ensure_flip(&mut self, n:usize){ if self.flip_states.len()<n { self.flip_states.resize(n, FlipState::new('0')); } }
    fn update_flips(&mut self, target:&[char]){
        self.ensure_flip(target.len());
        for (i,&c) in target.iter().enumerate(){ self.flip_states[i].set(c); }
    }
    fn tick_flips(&mut self, dt:f32){ for f in &mut self.flip_states { f.tick(dt);} }

    fn tick(&mut self, dt:f32){
        // steady colon — no blinking
        self.colon_blink = true;
        self.colon_timer = 0.0;
        if self.flash_timer>0.0{ self.flash_timer-=dt; }
        if let Some((_, t)) = &mut self.info_msg { *t-=dt; }
        if self.info_msg.as_ref().map(|(_,t)| *t<=0.0).unwrap_or(false){ self.info_msg=None; }

        match self.mode {
            AppMode::Pomodoro => {
                if self.pomodoro.running {
                    let was = self.pomodoro.remaining;
                    if was > Duration::from_secs_f32(dt) { self.pomodoro.remaining -= Duration::from_secs_f32(dt); }
                    else {
                        let finished_label = self.pomodoro.phase.label().to_string();
                        self.pomodoro.remaining=Duration::ZERO;
                        self.flash_timer=1.4;
                        self.info_msg=Some((format!("{finished_label} complete!"), 2.5));
                        self.pomodoro.advance();
                        let next = self.pomodoro.phase;
                        let next_len = format_duration_hms(self.pomodoro.phase_duration());
                        send_notification(
                            &format!("{finished_label} complete"),
                            &format!("Time for {} ({})", next.label(), next_len),
                        );
                        let auto = match next { PomodoroPhase::Work=>self.pomodoro.auto_start_work, _=>self.pomodoro.auto_start_breaks };
                        self.pomodoro.running=auto;
                    }
                }
            }
            AppMode::Timer => {
                if self.timer.running && !self.timer.finished {
                    if self.timer.remaining > Duration::from_secs_f32(dt) { self.timer.remaining-=Duration::from_secs_f32(dt); }
                    else {
                        self.timer.remaining=Duration::ZERO; self.timer.running=false; self.timer.finished=true; self.flash_timer=10.0;
                        let set = format_duration_hms(Duration::from_secs(self.timer.set_secs as u64));
                        self.info_msg=Some(("TIME'S UP! Press R to reset".into(), 4.0));
                        send_notification("Timer finished", &format!("{set} elapsed — press R to reset"));
                    }
                }
            }
            AppMode::Stopwatch => {
                if self.stopwatch.running { self.stopwatch.elapsed+=Duration::from_secs_f32(dt); }
            }
            _=>{}
        }
    }

    fn enter_timer_edit(&mut self) {
        if self.mode != AppMode::Timer || self.timer.running { return; }
        self.timer_editing = true;
        self.timer_buffer = format_duration_hms(Duration::from_secs(self.timer.set_secs as u64));
    }
    fn save_timer_edit(&mut self) {
        match parse_timer_input(&self.timer_buffer) {
            Some(secs) => {
                self.timer.set_secs = secs;
                self.timer.sync();
                self.timer_editing = false;
                self.timer_buffer.clear();
                self.info_msg = Some((format!("Timer set to {}", format_duration_hms(Duration::from_secs(secs as u64))), 2.0));
            }
            None => {
                self.info_msg = Some(("Invalid time — use MM:SS (e.g. 25:00)".into(), 2.5));
            }
        }
    }
    fn cancel_timer_edit(&mut self) {
        self.timer_editing = false;
        self.timer_buffer.clear();
    }

    fn do_action(&mut self, action: ClickAction) {
        match action {
            ClickAction::Mode(m) => { self.mode = m; self.cancel_timer_edit(); }
            ClickAction::StartPause => {
                match self.mode {
                    AppMode::Pomodoro => {
                        self.pomodoro.running=!self.pomodoro.running;
                        let left = format_duration_hms(self.pomodoro.remaining);
                        if self.pomodoro.running {
                            send_notification("Pomodoro started", &format!("{} — {} left", self.pomodoro.phase.label(), left));
                        } else {
                            send_notification("Pomodoro paused", &format!("{} — {} left", self.pomodoro.phase.label(), left));
                        }
                    }
                    AppMode::Timer => {
                        if self.timer.finished {
                            self.timer.finished=false; self.timer.remaining=Duration::from_secs(self.timer.set_secs as u64); self.timer.running=true; self.flash_timer=0.0;
                            send_notification("Timer started", &format!("{} left", format_duration_hms(self.timer.remaining)));
                        }
                        else {
                            self.timer.running=!self.timer.running;
                            if self.timer.running {
                                send_notification("Timer started", &format!("{} left", format_duration_hms(self.timer.remaining)));
                            } else {
                                send_notification("Timer paused", &format!("{} left", format_duration_hms(self.timer.remaining)));
                            }
                        }
                    }
                    AppMode::Stopwatch => self.stopwatch.running=!self.stopwatch.running,
                    _=>{}
                }
            }
            ClickAction::Reset => {
                match self.mode {
                    AppMode::Pomodoro => self.pomodoro.reset_current(),
                    AppMode::Timer => { self.timer.running=false; self.timer.finished=false; self.timer.remaining=Duration::from_secs(self.timer.set_secs as u64); self.flash_timer=0.0; self.cancel_timer_edit(); }
                    AppMode::Stopwatch => { self.stopwatch.running=false; self.stopwatch.elapsed=Duration::ZERO; self.stopwatch.laps.clear(); }
                    AppMode::Clock => { self.show_seconds=!self.show_seconds; }
                }
            }
            ClickAction::Skip => {
                if self.mode==AppMode::Pomodoro {
                    self.pomodoro.advance(); self.pomodoro.running=false; self.flash_timer=0.0;
                    send_notification("Pomodoro skipped", &format!("Now: {} ({})", self.pomodoro.phase.label(), format_duration_hms(self.pomodoro.phase_duration())));
                }
            }
            ClickAction::Lap => {
                if self.mode==AppMode::Stopwatch { if self.stopwatch.running || self.stopwatch.elapsed>Duration::ZERO { self.stopwatch.laps.push(self.stopwatch.elapsed); } }
            }
            ClickAction::TimerEdit => self.enter_timer_edit(),
            ClickAction::TimerSave => self.save_timer_edit(),
            ClickAction::TimerCancel => self.cancel_timer_edit(),
            ClickAction::ToggleSettings => { self.show_settings=!self.show_settings; }
            ClickAction::SettingsClose => { self.show_settings=false; }
            ClickAction::SettingsField(f) => {
                // clicking a row selects it; clicking boolean toggles, clicking value cycles?
                self.settings_field = f;
                match f {
                    SettingsField::AutoBreak => self.pomodoro.auto_start_breaks=!self.pomodoro.auto_start_breaks,
                    SettingsField::AutoWork => self.pomodoro.auto_start_work=!self.pomodoro.auto_start_work,
                    SettingsField::Transparent => self.transparent_bg=!self.transparent_bg,
                    SettingsField::Theme => { self.theme_idx=(self.theme_idx+1)%THEMES.len(); }
                    _ => {}
                }
                if !self.pomodoro.running { self.pomodoro.remaining=self.pomodoro.phase_duration(); }
            }
            ClickAction::ThemePrev => { self.theme_idx=(self.theme_idx+THEMES.len()-1)%THEMES.len(); }
            ClickAction::ThemeNext => { self.theme_idx=(self.theme_idx+1)%THEMES.len(); }
            ClickAction::Toggle24h => { self.use_24h=!self.use_24h; }
            ClickAction::ToggleSeconds => { self.show_seconds=!self.show_seconds; }
            ClickAction::ToggleTransparent => { self.transparent_bg=!self.transparent_bg; }
        }
    }

    fn handle_click(&mut self, x: u16, y: u16) {
        // settings popup takes priority (it's drawn last, on top)
        // iterate in reverse so top-most wins
        for (rect, action) in self.clickables.iter().rev() {
            if x>=rect.x && x<rect.x+rect.width && y>=rect.y && y<rect.y+rect.height {
                // if settings open, only allow settings-related clicks + close?
                // Actually allow all, but settings clicks first due to reverse order
                self.do_action(*action);
                return;
            }
        }
    }

    fn handle_key(&mut self, key: KeyEvent) {
        // timer direct-entry mode takes over all keys except Enter/Esc
        if self.timer_editing && !self.show_settings {
            match key.code {
                KeyCode::Esc => { self.cancel_timer_edit(); return; }
                KeyCode::Enter => { self.save_timer_edit(); return; }
                KeyCode::Backspace => { self.timer_buffer.pop(); return; }
                KeyCode::Char(c) if c.is_ascii_digit() || c==':' => {
                    if self.timer_buffer.len() < 8 { self.timer_buffer.push(c); }
                    return;
                }
                KeyCode::Char(c) if c=='h' || c=='H' || c=='m' || c=='M' || c=='s' || c=='S' => {
                    if self.timer_buffer.len() < 8 { self.timer_buffer.push(c.to_ascii_lowercase()); }
                    return;
                }
                _ => { return; }
            }
        }
        if self.show_settings {
            match key.code {
                KeyCode::Esc | KeyCode::Char('S') => { self.show_settings=false; return; }
                KeyCode::Char('q') if !key.modifiers.contains(KeyModifiers::CONTROL) => { self.show_settings=false; return; }
                KeyCode::Up | KeyCode::Char('k') => { self.settings_field=self.settings_field.prev(); }
                KeyCode::Down | KeyCode::Char('j') | KeyCode::Tab => { self.settings_field=self.settings_field.next(); }
                KeyCode::Left | KeyCode::Char('h') => { self.adjust_setting(-1); }
                KeyCode::Right | KeyCode::Char('l') => { self.adjust_setting(1); }
                KeyCode::Char('-') | KeyCode::Char('_') => { self.adjust_setting(-1); }
                KeyCode::Char('+') | KeyCode::Char('=') => { self.adjust_setting(1); }
                KeyCode::Char('[') => { self.theme_idx=(self.theme_idx+THEMES.len()-1)%THEMES.len(); }
                KeyCode::Char(']') => { self.theme_idx=(self.theme_idx+1)%THEMES.len(); }
                KeyCode::Enter => { self.show_settings=false; }
                _=>{}
            }
            return;
        }

        match key.code {
            KeyCode::Char('q') if key.modifiers.contains(KeyModifiers::CONTROL) => { self.should_quit=true; }
            KeyCode::Char('q') | KeyCode::Esc => { self.should_quit=true; }
            KeyCode::Char('1') => self.mode=AppMode::Clock,
            KeyCode::Char('2') => self.mode=AppMode::Pomodoro,
            KeyCode::Char('3') => self.mode=AppMode::Timer,
            KeyCode::Char('4') => self.mode=AppMode::Stopwatch,
            KeyCode::Tab => {
                if key.modifiers.contains(KeyModifiers::SHIFT){ self.mode=self.mode.prev(); } else { self.mode=self.mode.next(); }
            }
            KeyCode::BackTab => { self.mode=self.mode.prev(); }
            KeyCode::Char(' ') => self.do_action(ClickAction::StartPause),
            KeyCode::Char('r') | KeyCode::Char('R') => self.do_action(ClickAction::Reset),
            KeyCode::Char('s') if self.mode==AppMode::Pomodoro => self.do_action(ClickAction::Skip),
            KeyCode::Char('s') => { if self.mode!=AppMode::Pomodoro { self.show_settings=true; } }
            KeyCode::Char('S') => { self.show_settings=!self.show_settings; }
            KeyCode::Char('l') => {
                if self.mode==AppMode::Stopwatch { self.do_action(ClickAction::Lap); }
            }
            KeyCode::Char('L') => {
                if self.mode==AppMode::Stopwatch { self.do_action(ClickAction::Lap); }
            }
            // lowercase t = 12/24h, uppercase T = theme cycle
            KeyCode::Char('t') => { if self.mode==AppMode::Clock { self.use_24h=!self.use_24h; } }
            KeyCode::Char('T') => { self.theme_idx=(self.theme_idx+1)%THEMES.len(); }
            KeyCode::Char('[') => { self.theme_idx=(self.theme_idx+THEMES.len()-1)%THEMES.len(); }
            KeyCode::Char(']') => { self.theme_idx=(self.theme_idx+1)%THEMES.len(); }
            KeyCode::Char('m') | KeyCode::Char('M') => { self.mouse_enabled=!self.mouse_enabled; }
            KeyCode::Char('b') | KeyCode::Char('B') => { self.transparent_bg=!self.transparent_bg; }
            KeyCode::Char('n') => {
                if self.mode==AppMode::Pomodoro {
                    self.pomodoro.advance(); self.pomodoro.running=false;
                    send_notification("Pomodoro skipped", &format!("Now: {} ({})", self.pomodoro.phase.label(), format_duration_hms(self.pomodoro.phase_duration())));
                }
            }
            KeyCode::Char('e') | KeyCode::Char('E') => {
                if self.mode==AppMode::Timer && !self.timer.running { self.enter_timer_edit(); }
            }
            KeyCode::Up => {
                if self.mode==AppMode::Pomodoro { self.show_settings=true; }
            }
            KeyCode::Char('?') => { self.show_settings=!self.show_settings; }
            KeyCode::F(1) => { self.show_settings=!self.show_settings; }
            _=>{}
        }
    }

    fn adjust_setting(&mut self, dir:i32){
        match self.settings_field {
            SettingsField::Work => {
                let v=((self.pomodoro.work_secs as i32/60)+dir).clamp(5,60);
                self.pomodoro.work_secs=(v as u32)*60;
            }
            SettingsField::Short => {
                let v=((self.pomodoro.short_secs as i32/60)+dir).clamp(1,30);
                self.pomodoro.short_secs=(v as u32)*60;
            }
            SettingsField::Long => {
                let v=((self.pomodoro.long_secs as i32/60)+dir).clamp(5,45);
                self.pomodoro.long_secs=(v as u32)*60;
            }
            SettingsField::Cycles => {
                let v=(self.pomodoro.sessions_before_long as i32+dir).clamp(2,8);
                self.pomodoro.sessions_before_long=v as u32;
            }
            SettingsField::AutoBreak => self.pomodoro.auto_start_breaks=!self.pomodoro.auto_start_breaks,
            SettingsField::AutoWork => self.pomodoro.auto_start_work=!self.pomodoro.auto_start_work,
            SettingsField::Transparent => self.transparent_bg=!self.transparent_bg,
            SettingsField::Theme => {
                if dir>0 { self.theme_idx=(self.theme_idx+1)%THEMES.len(); }
                else { self.theme_idx=(self.theme_idx+THEMES.len()-1)%THEMES.len(); }
            }
        }
        if !self.pomodoro.running { self.pomodoro.remaining=self.pomodoro.phase_duration(); }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Helpers: time formatting
// ─────────────────────────────────────────────────────────────────────────────

fn digits_for_clock(use_24h:bool, show_seconds:bool)->Vec<char>{
    let now=chrono::Local::now();
    let (h,m,s)=(now.hour(), now.minute(), now.second());
    let h_disp = if use_24h { h } else { let h12=h%12; if h12==0{12}else{h12} };
    let mut v=Vec::new();
    v.extend(format!("{h_disp:02}").chars());
    v.extend(format!("{m:02}").chars());
    if show_seconds { v.extend(format!("{s:02}").chars()); }
    v
}
fn digits_for_duration(d:Duration, with_hours:bool)->Vec<char>{
    let secs=d.as_secs();
    if with_hours {
        let h=secs/3600; let m=(secs%3600)/60; let s=secs%60;
        let mut v=Vec::new();
        v.extend(format!("{h:02}").chars());
        v.extend(format!("{m:02}").chars());
        v.extend(format!("{s:02}").chars());
        v
    } else {
        let total_m=secs/60; let s2=secs%60;
        let mm=total_m.min(99);
        let mut v=Vec::new();
        v.extend(format!("{mm:02}").chars());
        v.extend(format!("{s2:02}").chars());
        v
    }
}
fn format_duration_hms(d:Duration)->String{
    let s=d.as_secs(); let h=s/3600; let m=(s%3600)/60; let sec=s%60;
    if h>0 { format!("{h:02}:{m:02}:{sec:02}") } else { format!("{m:02}:{sec:02}") }
}
fn format_stopwatch(d:Duration)->String{
    let ms=d.as_millis() as u64;
    let m=(ms/60000)%100; let s=(ms/1000)%60; let hund=(ms/10)%100;
    format!("{m:02}:{s:02}.{hund:02}")
}

/// Fire-and-forget desktop notification (start / pause / end states).
/// Errors are ignored so headless environments never break the TUI.
fn send_notification(summary: &str, body: &str) {
    let _ = notify_rust::Notification::new()
        .appname("FlipClock")
        .summary(summary)
        .body(body)
        .timeout(notify_rust::Timeout::Milliseconds(5000))
        .show();
}

/// Parse typed timer input. Accepts:
/// `MM:SS`, `HH:MM:SS`, `MM`, `25m`, `90s`, `2h`, plain `MMSS` runs
/// (e.g. `530` → 5:30, `125` → 1:25). Returns seconds, clamped 5s..24h.
fn parse_timer_input(raw: &str) -> Option<u32> {
    let s = raw.trim().to_lowercase().replace(' ', "");
    if s.is_empty() { return None; }
    // suffix forms: 25m / 90s / 2h
    if let Some(num) = s.strip_suffix('h') {
        let h: u32 = num.parse().ok()?;
        return Some((h * 3600).clamp(5, 24 * 3600));
    }
    if let Some(num) = s.strip_suffix('m') {
        // allow 5m30s? keep simple: integer minutes
        if num.contains('s') || num.contains(':') { return None; }
        let m: u32 = num.parse().ok()?;
        return Some((m * 60).clamp(5, 24 * 3600));
    }
    if let Some(num) = s.strip_suffix('s') {
        if num.contains(':') { return None; }
        let secs: u32 = num.parse().ok()?;
        return Some(secs.clamp(5, 24 * 3600));
    }
    if s.contains(':') {
        let parts: Vec<&str> = s.split(':').collect();
        if parts.len() > 3 || parts.iter().any(|p| p.is_empty() || p.len() > 3 || !p.chars().all(|c| c.is_ascii_digit())) {
            return None;
        }
        let nums: Vec<u32> = parts.iter().map(|p| p.parse().unwrap_or(0)).collect();
        let total = match nums.len() {
            2 => nums[0] * 60 + nums[1],
            3 => nums[0] * 3600 + nums[1] * 60 + nums[2],
            _ => return None,
        };
        if nums.len() == 3 && nums[1] >= 60 { return None; }
        if nums[nums.len() - 1] >= 60 { return None; }
        if total < 5 { return None; }
        return Some(total.min(24 * 3600));
    }
    // plain digits
    if !s.chars().all(|c| c.is_ascii_digit()) || s.len() > 6 { return None; }
    let total = if s.len() <= 2 {
        // "25" → 25 minutes
        s.parse::<u32>().ok()? * 60
    } else if s.len() <= 4 {
        // "530" → 5:30, "2500" → 25:00
        let (m, sec) = s.split_at(s.len() - 2);
        let m: u32 = m.parse().ok()?;
        let sec: u32 = sec.parse().ok()?;
        if sec >= 60 { return None; }
        m * 60 + sec
    } else {
        // "10130" → 1:01:30
        let (h, rest) = s.split_at(s.len() - 4);
        let (m, sec) = rest.split_at(2);
        let h: u32 = h.parse().ok()?;
        let m: u32 = m.parse().ok()?;
        let sec: u32 = sec.parse().ok()?;
        if m >= 60 || sec >= 60 { return None; }
        h * 3600 + m * 60 + sec
    };
    if total < 5 { return None; }
    Some(total.min(24 * 3600))
}
fn current_target(app: &App) -> (Vec<char>, bool) {
    match app.mode {
        AppMode::Clock => (digits_for_clock(app.use_24h, app.show_seconds), false),
        AppMode::Pomodoro => {
            let d = digits_for_duration(app.pomodoro.remaining, false);
            let f = app.flash_timer>0.0 && ((app.flash_timer*6.0).sin()>0.0);
            (d, f)
        }
        AppMode::Timer => {
            let with_h = app.timer.set_secs>=3600 || app.timer.remaining.as_secs()>=3600;
            let d = digits_for_duration(app.timer.remaining, with_h);
            let f = app.timer.finished;
            (d, f)
        }
        AppMode::Stopwatch => {
            let total_ms = app.stopwatch.elapsed.as_millis() as u64;
            let mins=(total_ms/60000)%100;
            let secs=(total_ms/1000)%60;
            let hund=(total_ms/10)%100;
            let s=format!("{mins:02}{secs:02}{hund:02}");
            (s.chars().collect::<Vec<_>>(), false)
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Readable 7-row flip-friendly glyphs.
// Row 3 (middle) is blank so the hinge/divider never cuts through a stroke.
// ─────────────────────────────────────────────────────────────────────────────

fn glyph7(ch: char) -> [&'static str; 7] {
    match ch {
        '0' => [" ███ ", "█   █", "█   █", "     ", "█   █", "█   █", " ███ "],
        '1' => ["  ██ ", " ███ ", "  ██ ", "     ", "  ██ ", "  ██ ", " ████"],
        '2' => [" ███ ", "    █", " ███ ", "     ", "█    ", "█    ", "█████"],
        '3' => ["████ ", "    █", " ███ ", "     ", "    █", "    █", "████ "],
        '4' => ["█   █", "█   █", "█████", "     ", "    █", "    █", "    █"],
        '5' => ["█████", "█    ", "████ ", "     ", "    █", "    █", "████ "],
        '6' => [" ███ ", "█    ", "████ ", "     ", "█   █", "█   █", " ███ "],
        '7' => ["█████", "    █", "   █ ", "     ", "  █  ", "  █  ", "  █  "],
        '8' => [" ███ ", "█   █", " ███ ", "     ", "█   █", "█   █", " ███ "],
        '9' => [" ███ ", "█   █", " ████", "     ", "    █", "    █", " ███ "],
        _   => ["     ", "  ?  ", "     ", "     ", "     ", "  ?  ", "     "],
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Drawing helpers: flip card into Buffer
// ─────────────────────────────────────────────────────────────────────────────

fn draw_flip_card(
    buf: &mut Buffer,
    x: u16,
    y: u16,
    w: u16,
    h: u16,
    state: &FlipState,
    flash: bool,
    th: &Theme,
) {
    // solid digit card — no split, no flip animation, no divider.
    // Single uniform background so digits are never cut in the middle.
    if w < 7 || h < 9 { return; }
    if x + w > buf.area.width || y + h > buf.area.height { return; }

    let border_col = if flash { th.accent_yellow } else { th.card_border };
    // uniform card background (use top color for consistency across themes)
    let card_bg = if flash { th.accent_yellow } else { th.card_top_bg };
    let text_fg = if flash {
        Color::Rgb(16,16,18)
    } else { th.text_main };

    for dy in 0..h {
        for dx in 0..w {
            let cx = x + dx;
            let cy = y + dy;
            if cx >= buf.area.width || cy >= buf.area.height { continue; }
            let cell = &mut buf[(cx, cy)];

            let is_top_border = dy == 0;
            let is_bottom_border = dy == h - 1;
            let is_left_border = dx == 0;
            let is_right_border = dx == w - 1;

            let (ch, style) = if is_top_border && is_left_border {
                ('┌', Style::default().fg(border_col).bg(th.bg))
            } else if is_top_border && is_right_border {
                ('┐', Style::default().fg(border_col).bg(th.bg))
            } else if is_bottom_border && is_left_border {
                ('└', Style::default().fg(border_col).bg(th.bg))
            } else if is_bottom_border && is_right_border {
                ('┘', Style::default().fg(border_col).bg(th.bg))
            } else if is_top_border || is_bottom_border {
                ('─', Style::default().fg(border_col).bg(th.bg))
            } else if is_left_border || is_right_border {
                ('│', Style::default().fg(border_col).bg(th.bg))
            } else {
                // solid interior — 5x7 glyph centered, no split
                let interior_w = (w - 2) as i16;
                let interior_h = (h - 2) as i16;
                let interior_x = (dx - 1) as i16;
                let interior_y = (dy - 1) as i16;

                let off_x = (interior_w - 5) / 2;
                let off_y = (interior_h - 7) / 2;

                let gx = interior_x - off_x;
                let gy = interior_y - off_y;
                if gx >= 0 && gx < 5 && gy >= 0 && gy < 7 {
                    let g = glyph7(state.current);
                    let row_str = g[gy as usize];
                    let ch_at = row_str.chars().nth(gx as usize).unwrap_or(' ');
                    if ch_at == '█' {
                        ('█', Style::default().fg(text_fg).bg(card_bg).add_modifier(Modifier::BOLD))
                    } else {
                        (' ', Style::default().bg(card_bg))
                    }
                } else {
                    (' ', Style::default().bg(card_bg))
                }
            };

            cell.set_char(ch);
            cell.set_style(style);
        }
    }
}

fn draw_colon(buf: &mut Buffer, x: u16, y: u16, w: u16, h: u16, on: bool, flash: bool, th: &Theme) {
    if x + w > buf.area.width || y + h > buf.area.height { return; }
    let col = if flash { th.accent_yellow } else if on { th.text_main } else { th.text_muted };
    let mid = h / 2;
    let dot_y1 = y + mid.saturating_sub(2);
    let dot_y2 = y + mid + 1;
    let cx = x + w / 2;
    for dy in 0..h {
        for dx in 0..w {
            if x+dx >= buf.area.width || y+dy >= buf.area.height { continue; }
            let cell = &mut buf[(x+dx, y+dy)];
            cell.set_char(' ');
            cell.set_style(Style::default().bg(th.slab_bg));
        }
    }
    if dot_y1 < buf.area.height && cx < buf.area.width {
        let c = &mut buf[(cx, dot_y1)];
        c.set_char('●');
        c.set_style(Style::default().fg(col).bg(th.slab_bg).add_modifier(Modifier::BOLD));
    }
    if dot_y2 < buf.area.height && cx < buf.area.width {
        let c = &mut buf[(cx, dot_y2)];
        c.set_char('●');
        c.set_style(Style::default().fg(col).bg(th.slab_bg).add_modifier(Modifier::BOLD));
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Responsive card chooser
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Clone, Copy)]
struct CardGeom { w: u16, h: u16, gap: u16, colon_w: u16 }

fn choose_card_size(avail_w: u16, avail_h: u16, num_digits: usize, num_colons: usize) -> Option<CardGeom> {
    // candidates large -> tiny
    let candidates = [
        CardGeom { w: 13, h: 11, gap: 1, colon_w: 4 },
        CardGeom { w: 11, h: 11, gap: 1, colon_w: 3 },
        CardGeom { w: 10, h: 10, gap: 1, colon_w: 3 },
        CardGeom { w: 9,  h: 10, gap: 1, colon_w: 2 },
    ];
    for c in candidates {
        let total_w = num_digits as u16 * c.w
            + (num_digits as u16).saturating_sub(1) * c.gap
            + num_colons as u16 * c.colon_w
            + num_colons as u16 * c.gap
            + 4; // slab pad + border
        let total_h = c.h + 4; // slab pad + border + spacing
        if total_w <= avail_w && total_h <= avail_h {
            return Some(c);
        }
    }
    None
}

// ─────────────────────────────────────────────────────────────────────────────
// UI rendering
// ─────────────────────────────────────────────────────────────────────────────

fn ui(frame: &mut Frame, app: &mut App) {
    let area = frame.area();
    let th = app.theme();
    app.clickables.clear();

    // fill bg (Reset = transparent, inherits Ghostty + opacity)
    for y in area.y..area.y+area.height {
        for x in area.x..area.x+area.width {
            frame.buffer_mut()[(x,y)].set_style(Style::default().bg(th.bg).fg(th.text_main));
            // don't overwrite char, just style
        }
    }

    // tiny terminal fallback: text only
    if area.width < 50 || area.height < 14 {
        let (digits, _) = current_target(app);
        // build time string
        let s: String = digits.iter().collect();
        // insert colons for readability
        let msg = format!("{}  [{}]  {}  (resize to enlarge, Q quit)", s, th.name, match app.mode {
            AppMode::Clock => "CLOCK",
            AppMode::Pomodoro => "POMODORO",
            AppMode::Timer => "TIMER",
            AppMode::Stopwatch => "STOPWATCH",
        });
        let p = Paragraph::new(vec![
            Line::from(Span::styled("FLIPCLOCK — terminal too small", Style::default().fg(th.accent_yellow).bg(th.bg).add_modifier(Modifier::BOLD))),
            Line::from(Span::styled(msg, Style::default().fg(th.text_main).bg(th.bg))),
            Line::from(Span::styled("1 Clock  2 Pomodoro  3 Timer  4 Stopwatch  Space Start  Q Quit", Style::default().fg(th.text_dim).bg(th.bg))),
        ])
        .alignment(Alignment::Center)
        .style(Style::default().bg(th.bg))
        .wrap(Wrap{trim:true});
        frame.render_widget(p, area);
        return;
    }

    // minimal layout: header / status / big numbers / progress / buttons / footer
    let header_h = if area.height < 24 { 2 } else { 3 };
    let mode_h = 1u16;
    let progress_h = 2u16;
    let controls_h = 2u16;
    // flip area gets the rest, but at least card_h+2
    // Use Layout with Min for flip to absorb extra space (centers vertically via inner centering)
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(header_h),
            Constraint::Length(mode_h),
            Constraint::Min(12),
            Constraint::Length(progress_h),
            Constraint::Length(controls_h),
            Constraint::Length(1), // footer
        ])
        .split(area);

    draw_header(frame, chunks[0], app);
    draw_mode_info(frame, chunks[1], app);
    let used_text_fallback = draw_flip_display(frame, chunks[2], app);
    if !used_text_fallback {
        draw_progress(frame, chunks[3], app);
    } else {
        // clear progress area if fallback used
        let buf = frame.buffer_mut();
        for y in chunks[3].y..chunks[3].y+chunks[3].height {
            for x in chunks[3].x..chunks[3].x+chunks[3].width {
                if x<buf.area.width && y<buf.area.height {
                    buf[(x,y)].set_char(' ');
                    buf[(x,y)].set_style(Style::default().bg(th.bg));
                }
            }
        }
    }
    draw_controls(frame, chunks[4], app);
    draw_footer_laps(frame, chunks[5], app);

    if app.show_settings {
        draw_settings(frame, app);
    }
    if let Some((msg,_)) = &app.info_msg.clone() {
        let w = (msg.len() as u16 + 6).min(area.width.saturating_sub(4)).max(10);
        let h = 3u16;
        let x = area.x + area.width.saturating_sub(w)/2;
        let y = area.y + area.height.saturating_sub(6);
        if y+ h < area.y+area.height {
            let popup = Rect::new(x, y, w, h);
            frame.render_widget(Clear, popup);
            let block = Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(th.accent_yellow))
                .style(Style::default().bg(th.popup_bg).fg(th.text_main));
            let p = Paragraph::new(Line::from(Span::styled(msg.clone(), Style::default().fg(th.accent_yellow).bg(th.popup_bg).add_modifier(Modifier::BOLD))))
                .block(block)
                .alignment(Alignment::Center)
                .wrap(Wrap{trim:true});
            frame.render_widget(p, popup);
        }
    }
}

fn draw_header(frame: &mut Frame, area: Rect, app: &mut App) {
    if area.width < 20 || area.height == 0 { return; }
    let th = app.theme();
    let buf = frame.buffer_mut();
    let title = " FLIPCLOCK ";
    let tabs = [
        (AppMode::Clock, " 1 CLOCK "),
        (AppMode::Pomodoro, " 2 POMODORO "),
        (AppMode::Timer, " 3 TIMER "),
        (AppMode::Stopwatch, " 4 STOPWATCH "),
    ];
    let y = area.y + area.height.saturating_sub(2).min(area.y+1).max(area.y);
    // actually center vertically: use area.y if height==2 else area.y+1
    let y = if area.height >= 3 { area.y + 1 } else { area.y };
    let mut x = area.x + 1;
    for ch in title.chars() {
        if x >= area.x+area.width { break; }
        if y >= buf.area.height { break; }
        let cell=&mut buf[(x,y)];
        cell.set_char(ch);
        cell.set_style(Style::default().fg(th.text_main).bg(th.bg).add_modifier(Modifier::BOLD));
        x+=1;
    }
    // minimal: tabs + settings icon only. No theme/24h spam (those live in Settings).
    let mut tab_x = area.x + area.width;
    // settings gear at far right
    let gear = " ⚙ ";
    if tab_x >= area.x + gear.len() as u16 + 1 {
        tab_x = tab_x.saturating_sub(gear.len() as u16 + 1);
        let rect = Rect::new(tab_x, y, gear.len() as u16, 1);
        app.clickables.push((rect, ClickAction::ToggleSettings));
        for (i,ch) in gear.chars().enumerate() {
            let cx = tab_x + i as u16;
            if cx >= area.x+area.width { continue; }
            let cell=&mut buf[(cx,y)];
            cell.set_char(ch);
            cell.set_style(Style::default().fg(th.text_dim).bg(th.bg));
        }
        if tab_x>0 { tab_x-=1; }
    }
    for (mode, label) in tabs.iter().rev() {
        // always full names: CLOCK / POMODORO / TIMER / STOPWATCH
        let lbl = *label;
        let len = lbl.len() as u16;
        if tab_x < area.x + len + 1 { break; }
        tab_x = tab_x.saturating_sub(len + 1);
        let selected = *mode == app.mode;
        // selected tab uses accent_blue (never yellow — yellow is reserved for flashes)
        let bg = if selected { th.accent_blue } else { th.bg };
        let fg = if selected {
            Color::Rgb(16,16,18)
        } else { th.text_dim };
        let rect = Rect::new(tab_x, y, lbl.len() as u16, 1);
        app.clickables.push((rect, ClickAction::Mode(*mode)));
        for (i,ch) in lbl.chars().enumerate() {
            let cx = tab_x + i as u16;
            if cx >= area.x+area.width { continue; }
            let cell=&mut buf[(cx,y)];
            cell.set_char(ch);
            cell.set_style(Style::default().fg(fg).bg(bg).add_modifier(if selected { Modifier::BOLD } else { Modifier::empty() }));
        }
        if tab_x>0 { tab_x-=1; }
    }
    if area.height >= 3 {
        let line_y = area.y.saturating_add(area.height.saturating_sub(1));
        for x in area.x..area.x+area.width {
            if line_y >= buf.area.height { break; }
            let cell=&mut buf[(x,line_y)];
            cell.set_char('─');
            cell.set_style(Style::default().fg(th.card_border_dim).bg(th.bg));
        }
    }
}

fn draw_mode_info(frame: &mut Frame, area: Rect, app: &mut App) {
    if area.height==0 { return; }
    let th = app.theme();
    let now = chrono::Local::now();
    let mut spans = Vec::new();
    match app.mode {
        AppMode::Clock => {
            let date = now.format("%A, %d %B %Y").to_string().to_uppercase();
            let ampm = if !app.use_24h { if now.hour()>=12 {" PM"} else {" AM"} } else {""};
            spans.push(Span::styled(format!("{date}{ampm}"), Style::default().fg(th.text_dim).bg(th.bg)));
        }
        AppMode::Pomodoro => {
            let phase = app.pomodoro.phase;
            let pc = app.pomodoro.phase_color(&th);
            spans.push(Span::styled(format!(" {} ", phase.label()), Style::default().fg(Color::Rgb(16,16,18)).bg(pc).add_modifier(Modifier::BOLD)));
            spans.push(Span::raw("  "));
            // hide session counts on narrow
            if area.width > 60 {
                spans.push(Span::styled(format!("SESSION {}/{}  •  TOTAL {}", app.pomodoro.session_in_cycle, app.pomodoro.sessions_before_long, app.pomodoro.completed), Style::default().fg(th.text_dim).bg(th.bg)));
                spans.push(Span::raw("  "));
            }
            spans.push(Span::styled(if app.pomodoro.running {"● RUNNING"} else {"○ PAUSED"}, Style::default().fg(th.text_muted).bg(th.bg)));
        }
        AppMode::Timer => {
            if app.timer_editing {
                spans.push(Span::styled(
                    format!("SET: {}_  (Enter save • Esc cancel)", app.timer_buffer),
                    Style::default().fg(th.accent_yellow).bg(th.bg).add_modifier(Modifier::BOLD),
                ));
            } else if app.timer.finished {
                spans.push(Span::styled("TIME'S UP — R TO RESET", Style::default().fg(th.accent_red).bg(th.bg).add_modifier(Modifier::BOLD)));
            }
            else if app.timer.running { spans.push(Span::styled("COUNTDOWN", Style::default().fg(th.text_dim).bg(th.bg))); }
            else {
                spans.push(Span::styled("SET TIMER  •  e to type time", Style::default().fg(th.text_dim).bg(th.bg)));
            }
        }
        AppMode::Stopwatch => {
            let s = if app.stopwatch.running {"● RUNNING"} else {"○ PAUSED"};
            spans.push(Span::styled(format!("{} LAPS  •  {s}", app.stopwatch.laps.len()), Style::default().fg(th.text_dim).bg(th.bg)));
        }
    }
    // no theme spam here — themes live in Settings only
    let p = Paragraph::new(Line::from(spans)).alignment(Alignment::Center).style(Style::default().bg(th.bg));
    frame.render_widget(p, area);
}

/// Normal clock display using the `tui-big-text` font library.
/// Solid big digits, steady colon (no blink, no split, no flip animation).
/// Returns true if plain-text fallback was used.
fn draw_flip_display(frame: &mut Frame, area: Rect, app: &mut App) -> bool {
    let th = app.theme();
    let (target_digits, _) = current_target(app);
    let num_digits = target_digits.len();
    let s: String = target_digits.iter().collect();
    // steady colon, never blinking. While typing a timer length, show the
    // edit buffer directly so what you type is what you get.
    if app.mode == AppMode::Timer && app.timer_editing {
        let buf = if app.timer_buffer.is_empty() { s.clone() } else { app.timer_buffer.clone() };
        let fg = th.accent_yellow;
        if area.width < 30 || area.height < 8 {
            let p = Paragraph::new(Line::from(Span::styled(
                buf,
                Style::default().fg(fg).bg(th.bg).add_modifier(Modifier::BOLD),
            )))
            .alignment(Alignment::Center)
            .style(Style::default().bg(th.bg));
            let h = 1u16.min(area.height);
            let y = area.y + area.height.saturating_sub(h) / 2;
            frame.render_widget(p, Rect::new(area.x, y, area.width, h.max(1)));
            return true;
        }
        let pixel = if area.height >= 10 { PixelSize::Quadrant } else { PixelSize::Sextant };
        let big = BigText::builder()
            .pixel_size(pixel)
            .style(Style::default().fg(fg).bg(th.bg))
            .lines(vec![buf.into()])
            .centered()
            .build();
        let want_h: u16 = match pixel {
            PixelSize::Full => 8,
            PixelSize::HalfHeight | PixelSize::HalfWidth | PixelSize::Quadrant => 4,
            _ => 3,
        };
        let render_h = want_h.min(area.height);
        let render_y = area.y + area.height.saturating_sub(render_h) / 2;
        frame.render_widget(big, Rect::new(area.x, render_y, area.width, render_h));
        return false;
    }
    let pretty: String = match app.mode {
        AppMode::Clock => {
            if app.show_seconds {
                format!("{}:{}:{}", &s[0..2], &s[2..4], &s[4..6])
            } else {
                format!("{}:{}", &s[0..2], &s[2..4])
            }
        }
        AppMode::Pomodoro | AppMode::Timer => {
            if num_digits == 6 {
                format!("{}:{}:{}", &s[0..2], &s[2..4], &s[4..6])
            } else {
                format!("{}:{}", &s[0..2], &s[2..4])
            }
        }
        AppMode::Stopwatch => {
            // MM:SS.hh — BigText font supports ':' and '.'; colon steady
            format!("{}:{}.{}", &s[0..2], &s[2..4], &s[4..6])
        }
    };

    // flash card tint when timer/pomodoro finishes (colon stays steady)
    let flashing = (app.mode == AppMode::Timer && app.timer.finished)
        || (app.mode == AppMode::Pomodoro && app.flash_timer > 0.0);
    let fg = if flashing { th.accent_yellow } else { th.text_main };

    // tiny area → plain text fallback
    if area.width < 30 || area.height < 8 {
        let p = Paragraph::new(Line::from(Span::styled(
            pretty,
            Style::default().fg(fg).bg(th.bg).add_modifier(Modifier::BOLD),
        )))
        .alignment(Alignment::Center)
        .style(Style::default().bg(th.bg));
        let h = 1u16.min(area.height);
        let y = area.y + area.height.saturating_sub(h) / 2;
        frame.render_widget(p, Rect::new(area.x, y, area.width, h.max(1)));
        return true;
    }

    // slimmer font: Quadrant is much less chunky than Full.
    // Sextant is smallest for short terminals.
    let pixel = if area.height >= 10 {
        PixelSize::Quadrant
    } else {
        PixelSize::Sextant
    };
    let big = BigText::builder()
        .pixel_size(pixel)
        .style(Style::default().fg(fg).bg(th.bg))
        .lines(vec![pretty.into()])
        .centered()
        .build();
    // vertically center: BigText is 1 line → ~4 rows (Quadrant) / ~3 rows (Sextant)
    let want_h: u16 = match pixel {
        PixelSize::Full => 8,
        PixelSize::HalfHeight | PixelSize::HalfWidth | PixelSize::Quadrant => 4,
        _ => 3,
    };
    let render_h = want_h.min(area.height);
    let render_y = area.y + area.height.saturating_sub(render_h) / 2;
    let centered = Rect::new(area.x, render_y, area.width, render_h);
    frame.render_widget(big, centered);
    false
}

fn draw_progress(frame: &mut Frame, area: Rect, app: &App) {
    if area.height==0 { return; }
    let th = app.theme();
    let buf = frame.buffer_mut();
    for y in area.y..area.y+area.height {
        for x in area.x..area.x+area.width {
            if x>=buf.area.width || y>=buf.area.height { continue; }
            buf[(x,y)].set_style(Style::default().bg(th.bg));
            buf[(x,y)].set_char(' ');
        }
    }
    match app.mode {
        AppMode::Pomodoro => {
            let total = app.pomodoro.phase_duration().as_secs_f32().max(1.0);
            let remain = app.pomodoro.remaining.as_secs_f32();
            let prog = 1.0 - (remain/total);
            let bar_w = (area.width.saturating_mul(6)/10).min(60).max(20).min(area.width);
            if bar_w==0 { return; }
            let bar_x = area.x + area.width.saturating_sub(bar_w)/2;
            let bar_y = area.y;
            for x in bar_x..bar_x+bar_w {
                if x>=buf.area.width { break; }
                buf[(x,bar_y)].set_char('─');
                buf[(x,bar_y)].set_style(Style::default().fg(th.card_border_dim).bg(th.bg));
            }
            let fill_w = (bar_w as f32 * prog.clamp(0.0,1.0)) as u16;
            let pc = app.pomodoro.phase_color(&th);
            for x in bar_x..bar_x+fill_w {
                if x>=buf.area.width { break; }
                buf[(x,bar_y)].set_char('━');
                buf[(x,bar_y)].set_style(Style::default().fg(pc).bg(th.bg).add_modifier(Modifier::BOLD));
            }
            if fill_w>0 && fill_w<bar_w && bar_x+fill_w < buf.area.width {
                buf[(bar_x+fill_w, bar_y)].set_char('●');
                buf[(bar_x+fill_w, bar_y)].set_style(Style::default().fg(pc).bg(th.bg));
            }
            if area.height>1 {
                let dot_y = bar_y+1;
                let cycles = app.pomodoro.sessions_before_long as u16;
                let dot_gap = 2u16;
                let total_dot_w = cycles + (cycles.saturating_sub(1))*dot_gap;
                if total_dot_w < area.width {
                    let dot_x = area.x + area.width.saturating_sub(total_dot_w)/2;
                    for i in 0..cycles {
                        let cx = dot_x + i*(1+dot_gap);
                        if cx>=buf.area.width { break; }
                        let filled = (i+1) < app.pomodoro.session_in_cycle as u16
                            || (i+1==app.pomodoro.session_in_cycle as u16 && app.pomodoro.phase!=PomodoroPhase::Work && app.pomodoro.completed>0);
                        let active = i+1==app.pomodoro.session_in_cycle as u16 && app.pomodoro.phase==PomodoroPhase::Work;
                        let ch = if filled {'●'} else if active {'○'} else {'·'};
                        let col = if filled {th.accent_red} else if active {th.text_main} else {th.text_muted};
                        let mut style=Style::default().fg(col).bg(th.bg);
                        if active && app.pomodoro.running { style=style.add_modifier(Modifier::BOLD); }
                        buf[(cx,dot_y)].set_char(ch);
                        buf[(cx,dot_y)].set_style(style);
                    }
                }
            }
        }
        AppMode::Timer => {
            let total = app.timer.set_secs as f32;
            let remain = app.timer.remaining.as_secs_f32();
            let prog = if total>0.0 {1.0 - (remain/total)} else {0.0};
            let bar_w = (area.width.saturating_mul(6)/10).min(60).max(20).min(area.width);
            if bar_w==0 { return; }
            let bar_x = area.x + area.width.saturating_sub(bar_w)/2;
            let bar_y = area.y;
            for x in bar_x..bar_x+bar_w {
                if x>=buf.area.width { break; }
                buf[(x,bar_y)].set_char('─');
                buf[(x,bar_y)].set_style(Style::default().fg(th.card_border_dim).bg(th.bg));
            }
            let fill_w = (bar_w as f32 * prog.clamp(0.0,1.0)) as u16;
            let col = if app.timer.finished {th.accent_red} else {th.accent_blue};
            for x in bar_x..bar_x+fill_w {
                if x>=buf.area.width { break; }
                buf[(x,bar_y)].set_char('━');
                buf[(x,bar_y)].set_style(Style::default().fg(col).bg(th.bg).add_modifier(Modifier::BOLD));
            }
            if fill_w>0 && fill_w<bar_w && bar_x+fill_w < buf.area.width {
                buf[(bar_x+fill_w, bar_y)].set_char('●');
                buf[(bar_x+fill_w, bar_y)].set_style(Style::default().fg(col).bg(th.bg));
            }
            if area.height>1 && app.timer.finished {
                let txt = "  TIME'S UP  ";
                if (txt.len() as u16) < area.width {
                    let tx = area.x + area.width.saturating_sub(txt.len() as u16)/2;
                    for (i,ch) in txt.chars().enumerate(){
                        if tx+i as u16 >= buf.area.width { break; }
                        buf[(tx+i as u16, bar_y+1)].set_char(ch);
                        buf[(tx+i as u16, bar_y+1)].set_style(Style::default().fg(th.accent_red).bg(th.bg).add_modifier(Modifier::BOLD));
                    }
                }
            }
        }
        AppMode::Stopwatch => {
            let txt = format!("{}  •  {}", format_stopwatch(app.stopwatch.elapsed), if app.stopwatch.running {"● RUNNING"} else {"○ PAUSED"});
            if (txt.len() as u16) < area.width {
                let tx = area.x + area.width.saturating_sub(txt.len() as u16)/2;
                for (i,ch) in txt.chars().enumerate(){
                    if tx+i as u16 >= area.x+area.width { break; }
                    buf[(tx+i as u16, area.y)].set_char(ch);
                    buf[(tx+i as u16, area.y)].set_style(Style::default().fg(th.text_dim).bg(th.bg));
                }
            }
        }
        AppMode::Clock => {
            let txt = "LOCAL TIME  •  CLOCK MODE";
            if (txt.len() as u16) < area.width {
                let tx = area.x + area.width.saturating_sub(txt.len() as u16)/2;
                for (i,ch) in txt.chars().enumerate(){
                    buf[(tx+i as u16, area.y)].set_char(ch);
                    buf[(tx+i as u16, area.y)].set_style(Style::default().fg(th.text_muted).bg(th.bg));
                }
            }
        }
    }
}

fn draw_controls(frame: &mut Frame, area: Rect, app: &mut App) {
    if area.height==0 || area.width < 30 { return; }
    let th = app.theme();
    // minimal: contextual buttons only (mode switching lives in top bar).
    // Clock has no buttons.
    let controls: Vec<(String, Color, Color, ClickAction)> = match app.mode {
        AppMode::Clock => vec![],
        AppMode::Pomodoro => {
            let running = app.pomodoro.running;
            let pc = app.pomodoro.phase_color(&th);
            vec![
                (if running {" ❚❚ PAUSE ".into()} else {" ▶ START ".into()}, if running {th.slab_bg} else {pc}, if running {th.text_main} else {Color::Rgb(16,16,18)}, ClickAction::StartPause),
                (" ↺ RESET ".into(), th.slab_bg, th.text_main, ClickAction::Reset),
                (" SKIP → ".into(), th.slab_bg, th.text_main, ClickAction::Skip),
            ]
        }
        AppMode::Timer => {
            // while typing a length, offer save/cancel instead of start
            if app.timer_editing {
                vec![
                    (" ✓ SAVE ".into(), th.accent_green, Color::Rgb(16,16,18), ClickAction::TimerSave),
                    (" ✗ CANCEL ".into(), th.slab_bg, th.text_main, ClickAction::TimerCancel),
                ]
            } else {
                let running = app.timer.running;
                let finished = app.timer.finished;
                let start_label: String = if finished {" ↺ RESTART ".into()} else if running {" ❚❚ PAUSE ".into()} else {" ▶ START ".into()};
                let col = if finished || !running {th.accent_blue} else {th.slab_bg};
                let fg = if finished || !running {Color::Rgb(16,16,18)} else {th.text_main};
                let mut v: Vec<(String, Color, Color, ClickAction)> = Vec::new();
                if !running && !finished {
                    v.push((" ✎ EDIT ".into(), th.slab_bg, th.text_main, ClickAction::TimerEdit));
                }
                v.push((start_label, col, fg, ClickAction::StartPause));
                v.push((" ↺ RESET ".into(), th.slab_bg, th.text_main, ClickAction::Reset));
                v
            }
        }
        AppMode::Stopwatch => {
            let running = app.stopwatch.running;
            vec![
                (if running {" ❚❚ PAUSE ".into()} else {" ▶ START ".into()}, if running {th.slab_bg} else {th.accent_green}, if running {th.text_main} else {Color::Rgb(16,16,18)}, ClickAction::StartPause),
                (" ◎ LAP ".into(), th.slab_bg, th.text_main, ClickAction::Lap),
                (" ↺ RESET ".into(), th.slab_bg, th.text_main, ClickAction::Reset),
            ]
        }
    };

    let gap = 1u16;
    let total_w: u16 = controls.iter().map(|(s,_,_,_)| s.len() as u16 + 2).sum::<u16>() + gap*(controls.len() as u16).saturating_sub(1);
    if total_w > area.width { return; }
    let start_x = area.x + area.width.saturating_sub(total_w)/2;
    let y = area.y;
    let buf = frame.buffer_mut();
    for yy in area.y..area.y+area.height {
        for xx in area.x..area.x+area.width {
            if xx>=buf.area.width || yy>=buf.area.height { continue; }
            buf[(xx,yy)].set_char(' ');
            buf[(xx,yy)].set_style(Style::default().bg(th.bg).fg(th.text_main));
        }
    }
    let empty = controls.is_empty();
    let mut cx = start_x;
    for (label, bg, fg, action) in &controls {
        let w = label.len() as u16 + 2;
        if cx + w > area.x+area.width { break; }
        if y >= buf.area.height { break; }
        let rect = Rect::new(cx, y, w, 1);
        app.clickables.push((rect, *action));
        for dx in 0..w {
            if cx+dx >= buf.area.width { continue; }
            let cell=&mut buf[(cx+dx, y)];
            let ch = if dx==0 {' '} else if dx==w-1 {' '} else { label.chars().nth((dx-1) as usize).unwrap_or(' ') };
            cell.set_char(ch);
            cell.set_style(Style::default().fg(*fg).bg(*bg).add_modifier(Modifier::BOLD));
        }
        cx += w + gap;
    }

    // single subtle hint line — no theme/mouse spam (those live in Settings)
    if area.height >= 2 && !empty {
        let hint_y = y+1;
        let hint = match app.mode {
            AppMode::Clock => "",
            AppMode::Pomodoro => "space start  •  r reset  •  s skip",
            AppMode::Timer if app.timer_editing => "type MM:SS  •  enter save  •  esc cancel",
            AppMode::Timer => "space start  •  r reset  •  e edit time",
            AppMode::Stopwatch => "space start  •  l lap  •  r reset",
        };
        if !hint.is_empty() && (hint.len() as u16) < area.width {
            let hx = area.x + area.width.saturating_sub(hint.len() as u16)/2;
            for (i,ch) in hint.chars().enumerate(){
                let cxx=hx+i as u16;
                if cxx>=area.x+area.width || cxx>=buf.area.width { break; }
                buf[(cxx,hint_y)].set_char(ch);
                buf[(cxx,hint_y)].set_style(Style::default().fg(th.text_muted).bg(th.bg));
            }
        }
    }
    // empty controls (Clock mode): clear area only
    if empty {
        return;
    }
}

fn draw_footer_laps(frame: &mut Frame, area: Rect, app: &mut App) {
    if area.height==0 { return; }
    let th = app.theme();
    if app.mode==AppMode::Stopwatch && !app.stopwatch.laps.is_empty() && area.height >= 3 {
        let title = format!(" LAPS ({} total) — newest first ", app.stopwatch.laps.len());
        let block = Block::default()
            .title(title)
            .title_style(Style::default().fg(th.text_dim).bg(th.bg))
            .borders(Borders::ALL)
            .border_style(Style::default().fg(th.card_border_dim))
            .style(Style::default().bg(th.slab_bg));
        let inner = block.inner(area);
        frame.render_widget(block, area);
        let mut y = inner.y;
        for (rev_idx, lap) in app.stopwatch.laps.iter().rev().enumerate() {
            if y >= inner.y + inner.height { break; }
            let idx = app.stopwatch.laps.len() - rev_idx;
            let prev = if idx>=2 { app.stopwatch.laps[idx-2] } else { Duration::ZERO };
            let split = lap.checked_sub(prev).unwrap_or(*lap);
            let mut spans = vec![
                Span::styled(format!(" LAP {:02} ", idx), Style::default().fg(th.text_muted).bg(th.slab_bg)),
                Span::styled("│ ", Style::default().fg(th.card_border_dim).bg(th.slab_bg)),
                Span::styled(format_duration_hms(*lap), Style::default().fg(th.text_main).bg(th.slab_bg).add_modifier(Modifier::BOLD)),
                Span::styled("  │  +", Style::default().fg(th.card_border_dim).bg(th.slab_bg)),
                Span::styled(format_duration_hms(split), Style::default().fg(th.text_dim).bg(th.slab_bg)),
            ];
            if rev_idx==0 {
                spans.push(Span::styled("  ◀ latest", Style::default().fg(th.accent_green).bg(th.slab_bg)));
            }
            let p = Paragraph::new(Line::from(spans)).style(Style::default().bg(th.slab_bg));
            let row_area = Rect::new(inner.x, y, inner.width, 1);
            frame.render_widget(p, row_area);
            y+=1;
            if rev_idx < app.stopwatch.laps.len()-1 && y < inner.y+inner.height {
                for x in inner.x..inner.x+inner.width {
                    if x>=frame.buffer_mut().area.width { break; }
                    frame.buffer_mut()[(x,y)].set_char('·');
                    frame.buffer_mut()[(x,y)].set_style(Style::default().fg(th.card_border_dim).bg(th.slab_bg));
                }
                y+=1;
            }
        }
    } else {
        // minimal footer — no theme/mouse spam (in Settings only)
        let p = Paragraph::new(Line::from(Span::styled(
            "s settings  •  q quit",
            Style::default().fg(th.text_muted).bg(th.bg),
        )))
        .alignment(Alignment::Center)
        .style(Style::default().bg(th.bg));
        let footer_area = Rect::new(area.x, area.y + area.height.saturating_sub(1), area.width, 1);
        frame.render_widget(p, footer_area);
    }
}

fn draw_settings(frame: &mut Frame, app: &mut App) {
    let area = frame.area();
    let th = app.theme();
    if area.width < 30 || area.height < 12 { return; }
    let w = 58u16.min(area.width.saturating_sub(4)).max(30);
    let h = 24u16.min(area.height.saturating_sub(4)).max(14);
    let x = area.x + area.width.saturating_sub(w)/2;
    let y = area.y + area.height.saturating_sub(h)/2;
    let popup = Rect::new(x,y,w,h);
    frame.render_widget(Clear, popup);
    let block = Block::default()
        .title(" SETTINGS — ↑↓ Navigate  ←→ Adjust  Enter/Esc Close ")
        .title_style(Style::default().fg(th.text_main).bg(th.popup_bg).add_modifier(Modifier::BOLD))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(th.accent_yellow))
        .style(Style::default().bg(th.popup_bg));
    frame.render_widget(block, popup);
    let inner = Rect::new(x+2, y+2, w.saturating_sub(4), h.saturating_sub(4));
    if inner.width < 10 || inner.height < 6 { return; }

    let mut list_y = inner.y;

    // helper to render a subheading row (non-selectable, non-clickable)
    let mut heading = |frame: &mut Frame, app: &mut App, list_y: &mut u16, text: &str| {
        if *list_y >= inner.y + inner.height { return; }
        let th = app.theme();
        let line = format!("── {} ──", text);
        let p = Paragraph::new(Line::from(Span::styled(
            line,
            Style::default().fg(th.text_muted).bg(th.popup_bg).add_modifier(Modifier::BOLD),
        )))
        .style(Style::default().bg(th.popup_bg));
        frame.render_widget(p, Rect::new(inner.x, *list_y, inner.width, 1));
        *list_y += 1;
    };

    // helper to render one selectable field row
    let mut field_row = |frame: &mut Frame, app: &mut App, list_y: &mut u16, field: SettingsField, label: String, value: String| {
        if *list_y >= inner.y + inner.height { return; }
        let th = app.theme();
        let selected = field == app.settings_field;
        let bg = if selected { th.card_border_dim } else { th.popup_bg };
        let bg = match bg {
            Color::Reset if selected => Color::DarkGray,
            other => other,
        };
        let fg = if selected { th.text_main } else { th.text_dim };
        let fg = match (fg, selected) {
            (Color::Reset, true) => Color::White,
            (other, _) => other,
        };
        let prefix = if selected {"▶ "} else {"  "};
        let suffix = if field==SettingsField::Theme {"  < >"} else if selected {"  ◀▶"} else {""};
        let line = format!("{prefix}{label:<20} {value:>14}{suffix}");
        let line: String = line.chars().take(inner.width as usize).collect();
        let style = Style::default().fg(fg).bg(bg).add_modifier(if selected {Modifier::BOLD} else {Modifier::empty()});
        let p = Paragraph::new(Line::from(Span::styled(line, style))).style(Style::default().bg(bg));
        let row = Rect::new(inner.x, *list_y, inner.width, 1);
        app.clickables.push((row, ClickAction::SettingsField(field)));
        frame.render_widget(p, row);
        *list_y += 1;
    };

    // ── Display (applies everywhere) ──
    heading(frame, app, &mut list_y, "DISPLAY");
    field_row(frame, app, &mut list_y, SettingsField::Theme, "Theme".into(), th.name.to_string());
    field_row(frame, app, &mut list_y, SettingsField::Transparent, "Transparent BG".into(), if app.transparent_bg {"ON"} else {"OFF"}.into());

    // ── Pomodoro-specific (only affects Pomodoro mode) ──
    heading(frame, app, &mut list_y, "POMODORO-SPECIFIC");
    field_row(frame, app, &mut list_y, SettingsField::Work, "Focus".into(), format!("{:02} min", app.pomodoro.work_secs/60));
    field_row(frame, app, &mut list_y, SettingsField::Short, "Short Break".into(), format!("{:02} min", app.pomodoro.short_secs/60));
    field_row(frame, app, &mut list_y, SettingsField::Long, "Long Break".into(), format!("{:02} min", app.pomodoro.long_secs/60));
    field_row(frame, app, &mut list_y, SettingsField::Cycles, "Sessions before Long".into(), format!("{}", app.pomodoro.sessions_before_long));
    field_row(frame, app, &mut list_y, SettingsField::AutoBreak, "Auto-start breaks".into(), if app.pomodoro.auto_start_breaks {"ON"} else {"OFF"}.into());
    field_row(frame, app, &mut list_y, SettingsField::AutoWork, "Auto-start focus".into(), if app.pomodoro.auto_start_work {"ON"} else {"OFF"}.into());

    list_y+=1;
    if list_y >= inner.y+inner.height { return; }
    let gen: Vec<String> = vec![
        format!("Display: 24H [{}] SEC [{}] Mouse [{}]", if app.use_24h {"ON"} else {"OFF"}, if app.show_seconds {"ON"} else {"OFF"}, if app.mouse_enabled {"ON"} else {"OFF"}),
        "Theme: [ / ] cycle, T cycles, B transparent BG".into(),
        "".into(),
        "Keys: 1-4 Mode • TAB Cycle • SPACE Start • Q Quit".into(),
        "Timer: e type time (MM:SS, 25m, 90s) • Enter save".into(),
        "Click: tabs, buttons, settings rows".into(),
    ];
    for line in gen {
        if list_y >= inner.y+inner.height { break; }
        let line: String = line.chars().take(inner.width as usize).collect();
        let p = Paragraph::new(Line::from(Span::styled(line, Style::default().fg(th.text_muted).bg(th.popup_bg)))).style(Style::default().bg(th.popup_bg));
        let row = Rect::new(inner.x, list_y, inner.width, 1);
        frame.render_widget(p, row);
        list_y+=1;
    }
    if inner.height >= 2 {
        let footer = " [↑↓] Move  [←→/+-] Change  [Enter/Esc] Close ";
        let p = Paragraph::new(Line::from(Span::styled(footer, Style::default().fg(th.accent_green).bg(th.popup_bg)))).alignment(Alignment::Center);
        let row = Rect::new(inner.x, inner.y+inner.height-1, inner.width, 1);
        app.clickables.push((row, ClickAction::SettingsClose));
        frame.render_widget(p, row);
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Main loop
// ─────────────────────────────────────────────────────────────────────────────

fn run_app<B: ratatui::backend::Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    let tick_rate = Duration::from_millis(33);
    let mut last = Instant::now();
    loop {
        terminal.draw(|f| ui(f, &mut app))?;
        let timeout = tick_rate.saturating_sub(last.elapsed());
        if event::poll(timeout)? {
            match event::read()? {
                Event::Key(key) => {
                    app.handle_key(key);
                    if app.should_quit { return Ok(()); }
                }
                Event::Mouse(m) => {
                    if !app.mouse_enabled { continue; }
                    match m.kind {
                        MouseEventKind::Down(MouseButton::Left) => {
                            app.handle_click(m.column, m.row);
                            if app.should_quit { return Ok(()); }
                        }
                        _ => {}
                    }
                }
                Event::Resize(_, _) => {}
                _ => {}
            }
        }
        let now = Instant::now();
        let dt = (now - last).as_secs_f32().min(0.1);
        last = now;
        app.tick(dt);
        app.tick_flips(dt);
    }
}

fn main() -> io::Result<()> {
    use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
    let orig_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        let _ = disable_raw_mode();
        let _ = execute!(io::stdout(), LeaveAlternateScreen, DisableMouseCapture);
        orig_hook(info);
    }));

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = ratatui::backend::CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    let app = App::default();
    let res = run_app(&mut terminal, app);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    terminal.show_cursor()?;

    if let Err(e) = res { eprintln!("App error: {e}"); }
    Ok(())
}
