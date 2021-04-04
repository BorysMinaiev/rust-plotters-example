use druid::widget::{Button, Flex, Label};
use druid::{AppLauncher, LocalizedString, PlatformError, Widget, WidgetExt, WindowDesc, Point, Size, AppDelegate, Application, Data, EventCtx, LifeCycle, PaintCtx, BoxConstraints, LifeCycleCtx, LayoutCtx, Event, Env, UpdateCtx, Color, RenderContext};

struct Delegate;

impl<T: Data> AppDelegate<T> for Delegate {
    fn window_added(
        &mut self,
        _id: druid::WindowId,
        _data: &mut T,
        _env: &druid::Env,
        _ctx: &mut druid::DelegateCtx,
    ) {}

    fn window_removed(
        &mut self,
        _id: druid::WindowId,
        _data: &mut T,
        _env: &druid::Env,
        _ctx: &mut druid::DelegateCtx,
    ) {
        Application::global().quit();
    }
}

#[derive(Data, Clone)]
struct WidgetData {
    value: i32
}

struct MyWidget;

impl Widget<WidgetData> for MyWidget {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut WidgetData, env: &Env) {}

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &WidgetData, env: &Env) {}

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &WidgetData, data: &WidgetData, env: &Env) {}

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &WidgetData, env: &Env) -> Size {
        if bc.is_width_bounded() || bc.is_height_bounded() {
            let size = Size::new(500.0, 100.0);
            bc.constrain(size)
        } else {
            bc.max()
        }
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &WidgetData, env: &Env) {
        // Clear the whole widget with the color of your choice
        // (ctx.size() returns the size of the layout rect we're painting in)
        // Note: ctx also has a `clear` method, but that clears the whole context,
        // and we only want to clear this widget's area.
        let size = ctx.size();
        let rect = size.to_rect();
        ctx.fill(rect, &Color::WHITE);
    }
}

fn main() -> Result<(), PlatformError> {
    let main_window = WindowDesc::new(ui_builder)
        .set_position(Point { x: 500.0, y: 200.0 })
        .window_size(Size { width: 500.0, height: 500.0 }).title("hello!");
    let data = WidgetData { value: 0 };
    AppLauncher::with_window(main_window).delegate(Delegate)
        .use_simple_logger()
        .launch(data)
}

fn ui_builder() -> impl Widget<WidgetData> {
    // The label text will be computed dynamically based on the current locale and count
    let text =
        LocalizedString::new("hello-counter").with_arg("count", |data: &WidgetData, _env| data.value.into());
    let label = Label::new(text).padding(5.0).center();
    let button = Button::new("increment")
        .on_click(|_ctx, data: &mut WidgetData, _env| data.value += 1)
        .padding(5.0);

    let button2 = Button::new("decrement")
        .on_click(|_ctx, data: &mut WidgetData, _env| data.value -= 1)
        .padding(5.0);

    Flex::column().with_child(label).with_child(button).with_child(button2).with_child(MyWidget)
}