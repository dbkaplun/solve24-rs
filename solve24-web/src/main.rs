#![feature(match_default_bindings)]

extern crate solve24;
#[macro_use]
extern crate yew;

use solve24::{BoundOp, Card, Val};
use yew::prelude::*;
use yew::services::console::ConsoleService;

pub struct Context {
    #[allow(dead_code)]
    console: ConsoleService,
}

impl Default for Context {
    fn default() -> Self {
        Self {
            console: ConsoleService,
        }
    }
}

pub struct Model {
    val_top: Val,
    val_right: Val,
    val_bottom: Val,
    val_left: Val,
}

impl Model {
    fn get_card(&self) -> Card {
        Card::new(vec![
            self.val_top,
            self.val_right,
            self.val_bottom,
            self.val_left,
        ])
    }
}

impl Default for Model {
    fn default() -> Self {
        Self {
            val_top: 1.,
            val_right: 3.,
            val_bottom: 4.,
            val_left: 6.,
        }
    }
}

pub enum Msg {
    SetTop(String),
    SetRight(String),
    SetBottom(String),
    SetLeft(String),
}

impl Component<Context> for Model {
    type Msg = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _ctx: &mut Env<Context, Self>) -> Self {
        Model::default()
    }

    fn update(&mut self, msg: Self::Msg, _ctx: &mut Env<Context, Self>) -> ShouldRender {
        match msg {
            Msg::SetTop(v) => {
                self.val_top = parse_val(&*v);
            }
            Msg::SetRight(v) => {
                self.val_right = parse_val(&*v);
            }
            Msg::SetBottom(v) => {
                self.val_bottom = parse_val(&*v);
            }
            Msg::SetLeft(v) => {
                self.val_left = parse_val(&*v);
            }
        }
        true
    }
}

pub fn parse_val(input: &str) -> Val {
    let input_n = input.matches(char::is_numeric).collect::<String>();
    Val::from(input_n.parse::<i32>().unwrap_or(0))
}

impl Renderable<Context, Model> for Model {
    fn view(&self) -> Html<Context, Self> {
        let card = self.get_card();
        let solution_views = card.solve()
            .enumerate()
            .map(solution_view)
            .collect::<Vec<_>>();
        let solution_views_len = solution_views.len();
        html! {
            <div>
                <div class="card",>
                    <input name="top", value=self.val_top, oninput=|e: InputData| Msg::SetTop(e.value), />
                    <input name="right", value=self.val_right, oninput=|e: InputData| Msg::SetRight(e.value), />
                    <input name="bottom", value=self.val_bottom, oninput=|e: InputData| Msg::SetBottom(e.value), />
                    <input name="left", value=self.val_left, oninput=|e: InputData| Msg::SetLeft(e.value), />
                </div>
                {
                    match solution_views_len {
                        0 => html! { <p>{"No solutions."}</p> },
                        _ => html! {
                            <table>
                                <thead>
                                    <th>{"#"}</th>
                                    <th>{"Solution"}</th>
                                    <th>{"Explanation"}</th>
                                </thead>
                                { for solution_views }
                            </table>
                        },
                    }
                }
            </div>
        }
    }
}

fn solution_view((idx, bop): (usize, BoundOp)) -> Html<Context, Model> {
    html! {
        <tr>
            <td>{idx + 1}</td>
            <td>
                {bop.to_infix_notation()}
                <span class="text-muted",>
                    {format!(" = {}", bop.eval())}
                </span>
            </td>
            <td>
                <ul>
                    { for explain(&bop).1.into_iter().map(|s| {
                        html! {
                            <li>{s}</li>
                        }
                    }) }
                </ul>
            </td>
        </tr>
    }
}

fn explain(bop: &BoundOp) -> (Val, Vec<String>) {
    let mut explanation = vec![];
    match bop {
        BoundOp::Val(val) => (*val, explanation),
        BoundOp::BoundOp { op, l, r } => {
            let (lv, le) = explain(l);
            explanation.extend(le);
            let (rv, re) = explain(r);
            explanation.extend(re);

            let flat_bop = BoundOp::BoundOp {
                op: op.clone(),
                l: Box::new(BoundOp::Val(lv)),
                r: Box::new(BoundOp::Val(rv)),
            };
            let val = flat_bop.eval();
            explanation.push(format!("{} = {}", flat_bop.to_string(), val));
            (val, explanation)
        }
    }
}

fn main() {
    yew::initialize();
    let app: App<_, Model> = App::new(Context::default());
    app.mount_to_body();
    yew::run_loop();
}
