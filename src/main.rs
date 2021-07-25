use seed::prelude::*;
use seed::*;

enum Msg {}
struct Model {}

fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model {}
}

fn update(_msg: Msg, _model: &mut Model, _: &mut impl Orders<Msg>) {
    // do nothing
}

fn view(_model: &Model) -> impl IntoNodes<Msg> {
    div![
        C![
            "flex",
            "items-center",
            "w-screen",
            "h-screen",
            "mx-auto",
            "object-center",
            "justify-items-center"
        ],
        div![
            C![
                "w-1/3",
                "min-w-max",
                "mx-auto",
                "px-16",
                "bg-green-300",
                "rounded-sm",
                "shadow-md",
                "text-3xl",
                "text-gray-600",
                "text-center",
                "font-light",
                "font-mono",
                "leading-loose",
                "animate-fade-in"
            ],
            p!["老的网站因故意外下线",],
            p!["无心恢复老版设计",],
            p!["建设新版中，敬请谅解",]
        ]
    ]
}

fn main() {
    App::start("app", init, update, view);
}
