

use std::sync::{Arc, Mutex};

use druid::piet::InterpolationMode;
use druid::text::ParseFormatter;
use druid::widget::{Button, Controller, FillStrat, Flex, Image, TextBox};
use druid::{AppLauncher, Data, Env, Event, EventCtx, ImageBuf, Lens, MouseButton, Widget, WidgetExt, WindowDesc};

pub mod mandelbrot;
//use druid::image::{DynamicImage, RgbImage};
use mandelbrot::{image, Coordinates, Mandelbrot, Pos};

#[derive(Clone, Debug, Data, Lens)]
struct ApplicationState {
    mandelbrot: Arc<Mutex<Mandelbrot>>,
    coordinates: Arc<Mutex<Coordinates>>,
    iterations: usize,
}

impl ApplicationState {
    fn calc_mandelbrot_buffer(m: &Mandelbrot, c: &Coordinates) -> ImageBuf {
        // let m = self.mandelbrot.lock().unwrap();
        // let c = self.coordinates.lock().unwrap();
        let buffer = m.calculate2(&c);
        let i: image::RgbImage = m.render_mandelbrot_image(&buffer);
        let d = image::DynamicImage::ImageRgb8(i);
        let img_buf = ImageBuf::from_raw(d.as_bytes(), druid::piet::ImageFormat::Rgb, m.image_size.0, m.image_size.1);
        img_buf
    }
}
struct ImageController;

//impl<W: Widget<()>> Controller<(), W> for ImageController {
impl Controller<ApplicationState, Image> for ImageController {
    fn event(&mut self, _child: &mut Image, _ctx: &mut EventCtx, event: &Event, _data: &mut ApplicationState, _env: &Env) {
        match event {
            Event::MouseDown(mouse_event) => {
                
                println!("image mouse pos   : {:?}", mouse_event.pos);
                println!("image mouse button: {:?}", mouse_event.button);
               
                let mut c = _data.coordinates.lock().unwrap();
                let mut m = _data.mandelbrot.lock().unwrap();

                let mut zoom_in = true;

                match mouse_event.button {
                    MouseButton::Right => {
                        if m.max_iterations >= 1 {
                            //m.max_iterations -= 1;
                            zoom_in = false;
                        } else {
                            return;
                        }
                            
                        
                    },
                    MouseButton::Left => {
                        //m.max_iterations += 1;
                    },
                    _ => { return; }
                }

                _data.iterations = m.max_iterations;
                // let mut c = _data.coordinates.lock().unwrap();
                // let mut m = _data.mandelbrot.lock().unwrap();
                // m.max_iterations += 100;
                // _data.iterations = m.max_iterations;
                let image_size = Pos {x: m.image_size.0 as f64, y: m.image_size.1 as f64};
                c.calc_zoom_on_pos(Pos {x: mouse_event.pos.x as f64, y: mouse_event.pos.y as f64}, image_size, zoom_in);
                let img_buf = ApplicationState::calc_mandelbrot_buffer(&m, &c);

                _child.set_image_data(img_buf);
               _ctx.request_paint();

            },

            Event::KeyDown(key_event) => {
                let c = _data.coordinates.lock().unwrap();
                println!("key event: {}", key_event.code);
                println!("key event: {:?}", c);

            },

            Event::Wheel(wheel_event) => {
                let c = _data.coordinates.lock().unwrap();
                println!("key event: {}", wheel_event.wheel_delta);
                println!("key event: {:?}", c);
            },
            _ => (),
        }
    }
    
}
fn build_ui(state: &ApplicationState) -> impl Widget<ApplicationState> {
    // let c = Coordinates::default();
    // let m = Mandelbrot {
    //     max_iterations: 50,
    //     image_size: (1200, 760),
    //     threshold: None,
    //     ..Mandelbrot::default()
    // };

    let m = &state.mandelbrot.lock().unwrap();
    let c = &state.coordinates.lock().unwrap();
    // let buffer = m.calculate2(&c);
    // let i: image::RgbImage = m.render_mandelbrot_image(&buffer);
    // let d = image::DynamicImage::ImageRgb8(i);
    // let img_buf = ImageBuf::from_raw(d.as_bytes(), druid::piet::ImageFormat::Rgb, m.image_size.0, m.image_size.1);
    let img_buf = ApplicationState::calc_mandelbrot_buffer(m, c);
    let m_image = Image::new(img_buf.clone()).fill_mode(FillStrat::None).interpolation_mode(InterpolationMode::Bilinear);
    
    let map_buf = ImageBuf::from_raw(m.color_map.as_bytes(), druid::piet::ImageFormat::RgbaPremul, m.color_map.width() as usize, m.color_map.height() as usize);
    let map_image = Image::new(map_buf);
    let wrapped = m_image.controller(ImageController);

    let top_row = Flex::row().with_child(Button::new("Label"))
        .with_child(TextBox::new().with_formatter(ParseFormatter::new()).lens(ApplicationState::iterations)).fix_height(100.0);
    let root= Flex::column().with_child(top_row).with_child(wrapped).with_child(map_image);
    
    // m_image.on_click(|ctx, _data, _env| {
    //     _env.
    // });
    // m_image.event(ctx: &mut EventCtx, event: &Event, data: (), env: &Env) {
    //     match event {
    //         Event::MouseDown(mouse_event) => println!("image mouse pos: {:?}", mouse_event.pos),
    //         _ => (),
    //     }
    // });
    root
    
}

fn main() {

    let c = Coordinates::default();
    let m = Mandelbrot {
        max_iterations: 50,
        image_size: (1200, 760),
        threshold: None,
        ..Mandelbrot::default()
    };

    let state = ApplicationState {
        mandelbrot: Arc::new(Mutex::new(m)),
        coordinates: Arc::new(Mutex::new(c)),
        iterations: 50,
    };

    let main_window = WindowDesc::new(build_ui(&state))
        .window_size((1200.0, 862.0))
        .title("Mandelbrot Viewer");
    //let initial_data = ();

    AppLauncher::with_window(main_window)
        .launch(state)
        .expect("Failed to launch application");
}
// fn zoom_on_pos(&mut self, pos: Pos2, zoom_in: bool) {
//     let mut m_pos =  Pos2::new(0.0, 0.0);
//     m_pos.x = map(pos.x,  0.0, self.canvas_size.x, self.coordinates.re_min, self.coordinates.re_max);
//     m_pos.y = map(pos.y,  0.0, self.canvas_size.y, self.coordinates.im_min, self.coordinates.im_max);
//     let dx = self.coordinates.re_max - self.coordinates.re_min;
//     let dy = self.coordinates.im_max - self.coordinates.im_min;

//     let mut new = Coordinates::default();
//     if zoom_in {
//         new = Coordinates {
//             re_min: m_pos.x - dx / 4.0,
//             re_max: m_pos.x + dx / 4.0,
//             im_min: m_pos.y - dy / 4.0,
//             im_max: m_pos.y + dy / 4.0,

//         };
//         self.mandelbrot.max_iterations += 150;
//     } else {
//         new = Coordinates {
//             re_min: m_pos.x - dx,
//             re_max: m_pos.x + dx,
//             im_min: m_pos.y - dy,
//             im_max: m_pos.y + dy,

//         };
//         self.mandelbrot.max_iterations -= 150;
//     }

//     println!("pos  : {}", pos);
//     println!("m_pos: {}", m_pos);
//     println!("m_pos: {:?}", new);

    
//     self.coordinates = new;
    

// }



