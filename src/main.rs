// Copyright 2019 The Druid Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! An example of a custom drawing widget.

use druid::kurbo::{BezPath,Circle};
use druid::piet::{FontFamily, ImageFormat, InterpolationMode, FontWeight, FontStyle,

};
use druid::widget::prelude::*;
use druid::{
    Affine, AppLauncher, ArcStr, Color, FontDescriptor, LocalizedString, Point, Rect, TextLayout,
    WindowDesc,
    Data,
};

use std::rc::Rc;
use std::cell::RefCell;

mod pga3d;

struct CustomWidget;

fn drawtext(ctx: &mut impl RenderContext) {
     let layout2 = ctx
        .text()
        // .new_text_layout("Helloo piet!")
;
}

#[derive(Clone,Data)]
struct State {
    min_x: f32,
    max_x: f32,
    min_y: f32,
    max_y: f32,

    points: Rc<Vec<Dot>>,
    // points: Rc<RefCell<Vec<Dot>>>,
    x: f32, y:f32,
}

trait GeometricConstruction {
    fn add_dot(&mut self, label: String, x: f32, y:f32);
}

impl GeometricConstruction for State {
    fn add_dot(&mut self, label: String, x: f32, y:f32) {

        // self.points.push(Dot{label: label, x:x, y:y});
        Rc::get_mut(&mut self.points).unwrap().push(Dot{label: label, x:x, y:y});
        // (Rc::get_mut(&mut self.points).unwrap()).borrow_mut().push(Dot{label: label, x:x, y:y});
    }
}

impl Default for State {
    fn default() -> State {
        State{min_x: -2., min_y: -2., max_x: 2., max_y: 2.,points: Rc::new(Vec::new())
        // State{min_x: -2., min_y: -2., max_x: 2., max_y: 2.,points: Rc::new(RefCell::new(Vec::new()))
            ,x:0.,y:0.
        }
    }
}

#[derive(Default,Clone,Data)]
struct Dot {
    label: String,
    x: f32, y: f32
}

impl Widget<State> for CustomWidget {
    fn event(&mut self, _ctx: &mut EventCtx, _event: &Event, _data: &mut State, _env: &Env) {

        println!("event {:?}", _event);
    _ctx.request_focus();
        }

    fn lifecycle(
        &mut self,
        _ctx: &mut LifeCycleCtx,
        _event: &LifeCycle,
        _data: &State,
        _env: &Env,
    ) {

    }

    fn update(&mut self, _ctx: &mut UpdateCtx, _old_data: &State, _data: &State, _env: &Env) {}

    fn layout(
        &mut self,
        _layout_ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        _data: &State,
        _env: &Env,
    ) -> Size {
        // BoxConstraints are passed by the parent widget.
        // This method can return any Size within those constraints:
        // bc.constrain(my_size)
        //
        // To check if a dimension is infinite or not (e.g. scrolling):
        // bc.is_width_bounded() / bc.is_height_bounded()
        bc.max()
    }


    // The paint method gets called last, after an event flow.
    // It goes event -> update -> layout -> paint, and each method can influence the next.
    // Basically, anything that changes the appearance of a widget causes a paint.
    fn paint(&mut self, ctx: &mut PaintCtx
        , data: &State, env: &Env) {
        // Let's draw a picture with Piet!

        // Clear the whole widget with the color of your choice
        // (ctx.size() returns the size of the layout rect we're painting in)
        let size = ctx.size();
        let rect = size.to_rect();
        ctx.fill(rect, &Color::BLACK);


        // for p in (&data.points).borrow().iter() {
        for p in (&data.points).iter() {
            println!("label: {}", p.label);
        }

        // Note: ctx also has a `clear` method, but that clears the whole context,
        // and we only want to clear this widget's area.

        // Create an arbitrary bezier path
        let mut path = BezPath::new();
        path.move_to(Point::ORIGIN);
        path.quad_to((80.0, 90.0), (size.width, size.height));
        // Create a color
        let stroke_color = Color::rgb8(0, 128, 0);
        // Stroke the path with thickness 1.0
        ctx.stroke(path, &stroke_color, 1.0);

        // Rectangles: the path for practical people
        let rect = Rect::from_origin_size((10., 10.), (100., 100.));
        // Note the Color:rgba8 which includes an alpha channel (7F in this case)
        let fill_color = Color::rgba8(0x73, 0x73, 0x73, 0xFF);
        ctx.fill(rect, &fill_color);


        ctx.stroke(
            Circle::new(Point::new(data.x as f64, data.y as f64), 15.0),
            &fill_color,
            5.0,
        );



        // Text is easy; in real use TextLayout should be stored in the widget
        // and reused.
        let mut layout = TextLayout::<ArcStr>::from_text("hello");//data.to_owned());
        layout.set_font(FontDescriptor::new(FontFamily::SANS_SERIF).
            with_size(24.0)//.with_weight(FontWeight::BOLD)
            .with_style(FontStyle::Italic));
        layout.set_text_color(fill_color);
//        layout.set_text_style(FontStyle::Italic);
        layout.rebuild_if_needed(ctx.text(), env);

        // Let's rotate our text slightly. First we save our current (default) context:
        ctx.with_save(|ctx| {
            // Now we can rotate the context (or set a clip path, for instance):
            ctx.transform(Affine::rotate(0.0));
            layout.draw(ctx, (80.0, 40.0));
        });
        // When we exit with_save, the original context's rotation is restored

//drawtext(ctx);
     // let layout2 = ctx
        // .text()

// let mut moo: () = layout2;
        // .new_text_layout("Helloo piet!");
        // .font(FontFamily::SYSTEM_UI, 24.0)
// .default_attribute(FontStyle::Italic)
// .default_attribute(FontWeight::BOLD)
        // .default_attribute(TextAttribute::TextColor(RED_ALPHA))
        // .build()?;

//     let w: f64 = layout2.size().width;
//     rc.draw_text(&layout2, (80.0, 10.0));

//     rc.stroke(Line::new((80.0, 12.0), (80.0 + w, 12.0)), &RED_ALPHA, 1.0);

//     rc.with_save(|rc| {
//         rc.transform(Affine::rotate(0.1));
//         rc.draw_text(&layout2, (80.0, 10.0));
//         Ok(())
//     })?;



        // Let's burn some CPU to make a (partially transparent) image buffer
        let image_data = make_image_data(256, 256);
        let image = ctx
            .make_image(256, 256, &image_data, ImageFormat::RgbaSeparate)
            .unwrap();
        // The image is automatically scaled to fit the rect you pass to draw_image
        ctx.draw_image(&image, size.to_rect(), InterpolationMode::Bilinear);
    }
}

pub fn main() {
    let window = WindowDesc::new(|| CustomWidget {}).title(
        LocalizedString::new("custom-widget-demo-window-title").with_placeholder("Fancy Colors"),
    );


    let mut s = State{x:200.,y:200.,..Default::default()};

    s.add_dot("A".to_string(), -1., -1.);
    s.add_dot("B".to_string(), -1., 1.);
    s.add_dot("C".to_string(), 1., 1.);

pga3d::moo();

    AppLauncher::with_window(window)
        .use_simple_logger()
        .launch(s)
        // .launch("Druid + Piet".to_string())
        .expect("launch failed");
}

fn make_image_data(width: usize, height: usize) -> Vec<u8> {
    let mut result = vec![0; width * height * 4];
    for y in 0..height {
        for x in 0..width {
            let ix = (y * width + x) * 4;
            result[ix] = x as u8;
            result[ix + 1] = y as u8;
            result[ix + 2] = !(x as u8);
            result[ix + 3] = 127;
        }
    }
    result
}
