use druid::widget::{Button, Flex, Label};
use druid::{AppLauncher, LocalizedString, PlatformError, Widget, WidgetExt, WindowDesc, Point, Size, AppDelegate, Application};

struct Delegate;

impl AppDelegate<i32> for Delegate {
    fn window_added(
        &mut self,
        _id: druid::WindowId,
        _data: &mut i32,
        _env: &druid::Env,
        _ctx: &mut druid::DelegateCtx,
    ) {}

    fn window_removed(
        &mut self,
        _id: druid::WindowId,
        _data: &mut i32,
        _env: &druid::Env,
        _ctx: &mut druid::DelegateCtx,
    ) {
        Application::global().quit();
    }
}


fn main() -> Result<(), PlatformError> {
    let main_window = WindowDesc::new(ui_builder)
        .set_position(Point { x: 500.0, y: 200.0 })
        .window_size(Size { width: 500.0, height: 500.0 }).title("hello!");
    let data = 0;
    AppLauncher::with_window(main_window).delegate(Delegate)
        .use_simple_logger()
        .launch(data)
}

fn ui_builder() -> impl Widget<i32> {
    // The label text will be computed dynamically based on the current locale and count
    let text =
        LocalizedString::new("hello-counter").with_arg("count", |data: &i32, _env| (*data).into());
    let label = Label::new(text).padding(5.0).center();
    let button = Button::new("increment")
        .on_click(|_ctx, data, _env| *data += 1)
        .padding(5.0);

    let button2 = Button::new("decrement")
        .on_click(|_ctx, data, _env| *data -= 1)
        .padding(5.0);

    Flex::column().with_child(label).with_child(button).with_child(button2)
}