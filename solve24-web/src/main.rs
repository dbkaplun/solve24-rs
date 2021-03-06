#![feature(match_default_bindings)]
#![recursion_limit = "128"]

extern crate solve24;
extern crate stdweb;
#[macro_use]
extern crate yew;

use std::error::Error;
use solve24::{solve24, BoundOp, Val};
use stdweb::web::{document, IElement};
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
    fn solve<'a>(&'a self) -> Box<Iterator<Item = BoundOp> + 'a> {
        solve24(vec![
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
        let solution_views = self.solve()
            .enumerate()
            .map(solution_view)
            .collect::<Vec<_>>();
        let solution_views_len = solution_views.len();
        html! {
            <div class="row",>
                <div class="col-lg-5",>
                    <div class="solve24-card",>
                        <input
                            name="top",
                            value=self.val_top,
                            class="form-control",
                            oninput=|e: InputData| Msg::SetTop(e.value),
                        />
                        <input
                            name="right",
                            value=self.val_right,
                            class="form-control",
                            oninput=|e: InputData| Msg::SetRight(e.value),
                        />
                        <input
                            name="bottom",
                            value=self.val_bottom,
                            class="form-control",
                            oninput=|e: InputData| Msg::SetBottom(e.value),
                        />
                        <input
                            name="left",
                            value=self.val_left,
                            class="form-control",
                            oninput=|e: InputData| Msg::SetLeft(e.value),
                        />
                    </div>
                </div>
                <div class="col-lg-7",>
                    {
                        match solution_views_len {
                            0 => html! { <h3 class="my-3",>{"No solutions."}</h3> },
                            _ => html! {
                                <table class="table",>
                                    <thead>
                                        <th scope="col",>{"#"}</th>
                                        <th scope="col",>{"Solution"}</th>
                                        <th scope="col",>{"Explanation"}</th>
                                    </thead>
                                    { for solution_views }
                                </table>
                            },
                        }
                    }
                </div>
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
                <ol>
                    { for bop.explain().1.into_iter().map(|s| {
                        html! {
                            <li>{s}</li>
                        }
                    }) }
                </ol>
            </td>
        </tr>
    }
}

fn start() -> Result<(), Box<Error>> {
    yew::initialize();

    let app: App<_, Model> = App::new(Context::default());
    app.mount(try!(
        document()
            .query_selector("#app")
            .ok_or("couldn't find #app")
    ));

    let body = try!(
        document()
            .query_selector("body")
            .ok_or("couldn't find body")
    );
    body.class_list().add("has-webassembly");

    yew::run_loop();
    Ok(())
}

fn main() {
    if let Err(err) = start() {
        warn!("{}", err);
    }
}
