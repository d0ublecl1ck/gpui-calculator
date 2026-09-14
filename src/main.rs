use gpui_kit::component::{button::{Button, ButtonVariants}, Root, Sizable};
use gpui_kit::{div, prelude::*, px, size, App, Context, IntoElement, Render, Window, WindowBounds, WindowOptions};

#[derive(Clone, Copy, Debug, PartialEq)]
enum Op { Add, Subtract, Multiply, Divide }

#[derive(Default)]
struct Calculator {
    display: String,
    stored: Option<f64>,
    pending: Option<Op>,
    replace_display: bool,
}

impl Calculator {
    fn press_digit(&mut self, digit: char) {
        if self.replace_display || self.display == "0" {
            self.display.clear();
            self.replace_display = false;
        }
        self.display.push(digit);
    }

    fn press_decimal(&mut self) {
        if self.replace_display {
            self.display = "0".into();
            self.replace_display = false;
        }
        if !self.display.contains('.') { self.display.push('.'); }
    }

    fn press_operator(&mut self, op: Op) {
        let value = self.value();
        self.stored = Some(match (self.stored, self.pending) {
            (Some(left), Some(previous)) => calculate(left, value, previous),
            _ => value,
        });
        self.pending = Some(op);
        self.replace_display = true;
    }

    fn press_equals(&mut self) {
        if let (Some(left), Some(op)) = (self.stored, self.pending) {
            self.display = format_number(calculate(left, self.value(), op));
            self.stored = None;
            self.pending = None;
            self.replace_display = true;
        }
    }

    fn clear(&mut self) { *self = Self::default(); }
    fn toggle_sign(&mut self) { if self.display != "0" { self.display = format_number(-self.value()); } }
    fn percent(&mut self) { self.display = format_number(self.value() / 100.0); }
    fn value(&self) -> f64 { self.display.parse().unwrap_or(0.0) }
}

fn calculate(left: f64, right: f64, op: Op) -> f64 {
    match op {
        Op::Add => left + right,
        Op::Subtract => left - right,
        Op::Multiply => left * right,
        Op::Divide => if right == 0.0 { 0.0 } else { left / right },
    }
}

fn format_number(value: f64) -> String {
    if value.fract() == 0.0 { format!("{value:.0}") } else { format!("{value:.8}").trim_end_matches('0').trim_end_matches('.').into() }
}

impl Render for Calculator {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let display = self.display.clone();
        let button = |id: &'static str, label: &'static str| {
            Button::new(id).label(label).large().w(px(68.)).h(px(52.))
        };
        div().size_full().flex().flex_col().justify_center().items_center().gap_3().p_6()
            .child(div().w(px(300.)).text_3xl().text_right().px_3().py_4().child(display))
            .child(div().flex().gap_2().children([
                button("clear", "C").on_click(cx.listener(|this, _, _, cx| { this.clear(); cx.notify(); })),
                button("sign", "±").on_click(cx.listener(|this, _, _, cx| { this.toggle_sign(); cx.notify(); })),
                button("percent", "%").on_click(cx.listener(|this, _, _, cx| { this.percent(); cx.notify(); })),
                button("divide", "÷").on_click(cx.listener(|this, _, _, cx| { this.press_operator(Op::Divide); cx.notify(); })),
            ]))
            .child(self.row(cx, button, [('7', "7"), ('8', "8"), ('9', "9"), ('×', "×")]))
            .child(self.row(cx, button, [('4', "4"), ('5', "5"), ('6', "6"), ('−', "−")]))
            .child(self.row(cx, button, [('1', "1"), ('2', "2"), ('3', "3"), ('+', "+")]))
            .child(div().flex().gap_2()
                .child(button("digit-0", "0").w(px(138.)).on_click(cx.listener(|this, _, _, cx| { this.press_digit('0'); cx.notify(); })))
                .child(button("decimal", ".").on_click(cx.listener(|this, _, _, cx| { this.press_decimal(); cx.notify(); })))
                .child(button("equals", "=").primary().on_click(cx.listener(|this, _, _, cx| { this.press_equals(); cx.notify(); }))))
    }
}

impl Calculator {
    fn row<F>(&self, cx: &mut Context<Self>, button: F, keys: [(char, &'static str); 4]) -> impl IntoElement
    where F: Fn(&'static str, &'static str) -> Button + Copy {
        let mut row = div().flex().gap_2();
        for (key, label) in keys {
            let id = match key { '×' => "multiply", '−' => "subtract", '+' => "add", _ => match key { '7'=>"digit-7",'8'=>"digit-8",'9'=>"digit-9",'4'=>"digit-4",'5'=>"digit-5",'6'=>"digit-6",'1'=>"digit-1",'2'=>"digit-2",_=>"digit-3" } };
            let child = match key {
                '×' => button(id, label).on_click(cx.listener(|this, _, _, cx| { this.press_operator(Op::Multiply); cx.notify(); })),
                '−' => button(id, label).on_click(cx.listener(|this, _, _, cx| { this.press_operator(Op::Subtract); cx.notify(); })),
                '+' => button(id, label).on_click(cx.listener(|this, _, _, cx| { this.press_operator(Op::Add); cx.notify(); })),
                digit => button(id, label).on_click(cx.listener(move |this, _, _, cx| { this.press_digit(digit); cx.notify(); })),
            };
            row = row.child(child);
        }
        row
    }
}

fn main() {
    gpui_kit::application().run(|cx: &mut App| {
        gpui_kit::init(cx);
        cx.open_window(WindowOptions { window_bounds: Some(WindowBounds::centered(size(px(360.), px(520.)), cx)), ..Default::default() }, |window, cx| {
            let view = cx.new(|_| Calculator { display: "0".into(), ..Default::default() });
            cx.new(|cx| Root::new(view, window, cx))
        }).expect("failed to open calculator window");
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn calculator_flow() {
        let mut calc = Calculator { display: "2".into(), ..Default::default() };
        calc.press_operator(Op::Add); calc.press_digit('3'); calc.press_equals();
        assert_eq!(calc.display, "5");
    }
}
