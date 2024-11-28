use std::ffi::CString;
use std::ptr;

pub mod sdl;

const SCREEN_WIDTH: i32 = 640;
const SCREEN_HEIGHT: i32 = 480;
const BG_COLOR: u32 = 0x4583c3;

pub fn blit_text(
    surface: *mut sdl::SDL_Surface,
    text: &str,
    font: *mut sdl::TTF_Font,
    color: &sdl::SDL_Color,
    x: i16,
    y: i16,
) {
    let text = CString::new(text).unwrap();
    unsafe {
        let text_surface = sdl::TTF_RenderText_Blended(font, text.as_ptr(), *color);
        let mut pos = sdl::SDL_Rect { x, y, w: 0, h: 0 };
        sdl::SDL_UpperBlit(text_surface, ptr::null_mut(), surface, &mut pos);
        sdl::SDL_FreeSurface(text_surface);
    }
}

struct StayAwake {}

impl StayAwake {
    fn new() -> StayAwake {
        std::fs::File::create("/tmp/stay_awake").unwrap();
        StayAwake {}
    }
}
impl Drop for StayAwake {
    fn drop(&mut self) {
        std::fs::remove_file("/tmp/stay_awake").unwrap();
    }
}

fn main() {
    // OnionOS trick to prevent the device from sleeping.
    let _stay_awake = StayAwake::new();

    // Demo only -- unsafe calls because we are interfacing the the SDL C lib. Consider wrapping these unsafe calls in a safe, rusty API.
    unsafe {
        if sdl::SDL_Init(sdl::SDL_INIT_VIDEO) != 0 {
            panic!("SDL_Init: error");
        }

        if sdl::TTF_Init() != 0 {
            panic!("TTF_Init: error");
        }

        let font_path = CString::new("./assets/VT323-Regular.ttf").unwrap();
        let font: *mut sdl::_TTF_Font = sdl::TTF_OpenFont(font_path.as_ptr(), 24);
        if font.is_null() {
            panic!("Font is null");
        }

        // We can get the screen size from the video info. But since we also want to run on a host, I'm hardcoding the screen size.
        // let video_info = sdl::SDL_GetVideoInfo();
        // let width = (*video_info).current_w;
        // let height = (*video_info).current_h;
        let width = SCREEN_WIDTH;
        let height = SCREEN_HEIGHT;

        let screen: *mut sdl::SDL_Surface =
            sdl::SDL_SetVideoMode(width, height, 32, sdl::SDL_HWSURFACE);
        if screen.is_null() {
            panic!("Screen is null");
        }

        let surface: *mut sdl::SDL_Surface =
            sdl::SDL_CreateRGBSurface(sdl::SDL_HWSURFACE, width, height, 32, 0, 0, 0, 0);
        if surface.is_null() {
            panic!("Surface is null");
        }

        let logo_path = CString::new("./assets/rust-logo-256x256.png").unwrap();
        let logo: *mut sdl::SDL_Surface = sdl::IMG_Load(logo_path.as_ptr());

        let mut logo_pos = sdl::SDL_Rect {
            x: ((width - (*logo).w) / 2) as i16,
            y: ((height - (*logo).h) / 2) as i16,
            w: 0,
            h: 0,
        };
        if logo.is_null() {
            panic!("Logo is null");
        }

        let color_white = sdl::SDL_Color {
            r: 255,
            g: 255,
            b: 255,
            unused: 0,
        };

        let mut evt: sdl::SDL_Event = std::mem::zeroed();

        'main_: loop {
            // Poll events.
            while sdl::SDL_PollEvent(&mut evt) != 0 {
                match evt.type_ as u32 {
                    sdl::SDL_EventType_SDL_KEYDOWN => {
                        let key = evt.key.keysym.sym;
                        match key {
                            sdl::miyoo_btns::BTN_START => {
                                break 'main_;
                            }
                            sdl::miyoo_btns::BTN_LEFT => {
                                logo_pos.x -= 10;
                            }
                            sdl::miyoo_btns::BTN_RIGHT => {
                                logo_pos.x += 10;
                            }
                            sdl::miyoo_btns::BTN_UP => {
                                logo_pos.y -= 10;
                            }
                            sdl::miyoo_btns::BTN_DOWN => {
                                logo_pos.y += 10;
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }

            // Blit logo to surface.
            sdl::SDL_UpperBlit(logo, ptr::null_mut(), surface, &mut logo_pos);

            blit_text(
                surface,
                "ARROWS: move logo",
                font,
                &color_white,
                10,
                (height - 1 * 32) as i16,
            );

            blit_text(
                surface,
                "START / Enter: quit",
                font,
                &color_white,
                10,
                (height - 2 * 32) as i16,
            );

            blit_text(
                surface,
                format!("Logo y = {}, x = {}", logo_pos.y, logo_pos.x).as_str(),
                font,
                &color_white,
                width as i16 / 2,
                height as i16 - 1 * 32,
            );

            // Blit surface to screen.
            sdl::SDL_UpperBlit(surface, ptr::null_mut(), screen, ptr::null_mut());

            // Present screen.
            sdl::SDL_Flip(screen);

            // Hacky framerate cap. Use SDL_GetTicks() for a more accurate timer.
            sdl::SDL_Delay(16);

            // Clear screen.
            sdl::SDL_FillRect(surface, ptr::null_mut(), BG_COLOR);
        }

        sdl::SDL_FreeSurface(logo);
        sdl::SDL_FreeSurface(surface);
        sdl::SDL_FreeSurface(screen);
        sdl::SDL_Quit();
    }
    println!("Goodbye, world");
}
