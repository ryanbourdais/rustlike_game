use tcod::colors::*;
use tcod::console::*;

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;

const LIMIT_FPS: i32 = 20;

struct Tcod {
    root: Root,
    con: Offscreen,
}

fn handle_keys(tcod: &mut Tcod, player_x: &mut i32, player_y: &mut i32) -> bool {
    use tcod::input::Key;
    // use tcod::input::KeyCode::*;

    let key = tcod.root.wait_for_keypress(true);
    match key {
        Key { code: tcod::input::KeyCode::Up, .. } => *player_y -= 1,
        Key { code: tcod::input::KeyCode::Down, .. } => *player_y += 1,
        Key { code: tcod::input::KeyCode::Left, .. } => *player_x -= 1,
        Key { code: tcod::input::KeyCode::Right, .. } => *player_x += 1,
        Key { code: tcod::input::KeyCode::Enter, alt: true, .. } => {
            let fullscreen = tcod.root.is_fullscreen();
            tcod.root.set_fullscreen(!fullscreen);
        },
        Key { code: tcod::input::KeyCode::Escape, .. } => return true,
        _ => {}
    }
    false
}

fn main() {
    tcod::system::set_fps(LIMIT_FPS);

    let root = Root::initializer()
        .font("arial10x10.png", FontLayout::Tcod)
        .font_type(FontType::Greyscale)
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("RustLike Game")
        .init();

    let con = Offscreen::new(SCREEN_WIDTH, SCREEN_HEIGHT);

    let mut tcod = Tcod { root, con };

    let mut player_x = SCREEN_WIDTH / 2;
    let mut player_y = SCREEN_WIDTH / 2;

    while !tcod.root.window_closed() {
        tcod.con.set_default_foreground(WHITE);
        tcod.con.clear();
        tcod.con.put_char(player_x, player_y, '@', BackgroundFlag::None);
        tcod.root.flush();
        tcod.root.wait_for_keypress(true);

        blit(
            &tcod.con,
            (0,0),
            (SCREEN_WIDTH, SCREEN_HEIGHT),
            &mut tcod.root,
            (0,0),
            1.0,
            1.0,
        );

        let exit = handle_keys(&mut tcod, &mut player_x, &mut player_y);

        if exit {
            break;
        }
    }
}
