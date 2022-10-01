use fltk::{app, prelude::*, window::Window};
use hex::FromHex;
use pixels::{Pixels, SurfaceTexture};

// used to log some useful debug information
const DEBUG_MODE: bool = false;

// default world settings
const DEFAULT_WORLD_WIDTH: i32 = 600;
const DEFAULT_WORLD_HEIGHT: i32 = 400;
const DEFAULT_WORLD_COLOR: &str = "ffffffff";

// default entity settings
const DEFAULT_ENTITY_COUNT: i32 = 27;
const DEFAULT_ENTITY_WIDTH: i32 = 50;
const DEFAULT_ENTITY_HEIGHT: i32 = 50;
const DEFAULT_ENTITY_VELOCITY: i32 = 5;
const DEFAULT_ENTITY_COLOR: &str = "000000ff";

fn main() {
    // initialize app and window
    let app = app::App::default();
    let mut window = Window::default()
        .with_size(DEFAULT_WORLD_WIDTH, DEFAULT_WORLD_HEIGHT)
        .with_label("RUSTY-PIXELS-POC");
    window.end();

    // show window
    window.show();

    // initialize world and load it with entities
    let mut world = World::new();

    // // one orange entity in the back (for lil)
    // world.entities.push(Entity::new_custom(
    //     333,
    //     333,
    //     10,
    //     10,
    //     -1,
    //     -1,
    //     "ff9d00ff".to_string(),
    // ));

    // many default entities based on default settings
    for _ in (0..DEFAULT_ENTITY_COUNT).into_iter() {
        world.entities.push(Entity::new());
    }

    // red, blue and green entities for rrggbb
    world.entities.push(Entity::new_custom(
        DEFAULT_ENTITY_WIDTH * 2,
        DEFAULT_ENTITY_HEIGHT * 2,
        10,
        10,
        DEFAULT_ENTITY_VELOCITY,
        DEFAULT_ENTITY_VELOCITY,
        "ff0000ff".to_string(),
    ));
    world.entities.push(Entity::new_custom(
        DEFAULT_ENTITY_WIDTH * 2,
        DEFAULT_ENTITY_HEIGHT * 2,
        world.width / 3,
        world.height / 3,
        -DEFAULT_ENTITY_VELOCITY,
        -DEFAULT_ENTITY_VELOCITY,
        "00ff00ff".to_string(),
    ));
    world.entities.push(Entity::new_custom(
        DEFAULT_ENTITY_WIDTH * 2,
        DEFAULT_ENTITY_HEIGHT * 2,
        world.width / 3 * 2,
        world.height / 3 * 2,
        DEFAULT_ENTITY_VELOCITY,
        DEFAULT_ENTITY_VELOCITY,
        "0000ffff".to_string(),
    ));

    // print world object as json if in debug mode
    if DEBUG_MODE {
        println!("the whole world: {:?}", world);
    }

    // initialize pixels
    let mut pixels = {
        let pixel_width = window.pixel_w() as u32;
        let pixel_height = window.pixel_h() as u32;
        let surface_texture = SurfaceTexture::new(pixel_width, pixel_height, &window);
        Pixels::new(
            DEFAULT_WORLD_WIDTH as u32,
            DEFAULT_WORLD_HEIGHT as u32,
            surface_texture,
        )
        .expect("pixels failed to initialize")
    };

    // app loop
    while app.wait() {
        // handle events (if any)
        // there isn't any for this POC...

        // Update internal state
        world.update();

        // Draw the current frame
        world.draw(pixels.get_frame());
        if pixels
            .render()
            .map_err(|e| panic!("pixels.render() failed: {}", e))
            .is_err()
        {
            app.quit();
        }
        app::flush();
        app::awake();
    }
}

// World object containing various world states
// derive Debug for debug console information
#[derive(Debug)]
struct World {
    width: i32,
    height: i32,
    entities: Vec<Entity>,
    rgba_hex_str: String,
}

impl World {
    // initialize a new world with default settings
    fn new() -> Self {
        Self {
            width: DEFAULT_WORLD_WIDTH,
            height: DEFAULT_WORLD_HEIGHT,
            entities: vec![],
            rgba_hex_str: DEFAULT_WORLD_COLOR.to_string(),
        }
    }

    // // initialize a new world with custom settings
    // fn new_custom(width: i32, height: i32, rgba_hex_str: String) -> Self {
    //     // TODO: handle more gracefully
    //     if rgba_hex_str.len() != 9 {
    //         panic!("rgba_hex_str.len() != 9, expected format: rrggbbaa");
    //     }

    //     Self {
    //         width,
    //         height,
    //         entities: vec![],
    //         rgba_hex_str,
    //     }
    // }

    // update world every frame
    fn update(&mut self) {
        // update every entity inside world
        for entity in &mut self.entities {
            entity.update(self.width as i32, self.height as i32);
        }
    }

    // draw world every frame
    fn draw(&self, frame: &mut [u8]) {
        // loop through each pixel (frame split in four due to rrggbbaa format)
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            // calculate pixels x and y positions on frame
            let pixel_x_position = (i % DEFAULT_WORLD_WIDTH as usize) as i32;
            let pixel_y_position = (i / DEFAULT_WORLD_WIDTH as usize) as i32;

            // set the color based on if pixel is within an entity
            let mut rgba_hex_str = &self.rgba_hex_str;
            for entity in &self.entities {
                if pixel_x_position >= entity.x_position
                    && pixel_x_position <= entity.x_position + entity.width
                    && pixel_y_position >= entity.y_position
                    && pixel_y_position <= entity.y_position + entity.height
                {
                    // use entity's rbga if within
                    rgba_hex_str = &entity.rgba_hex_str;
                }
            }

            // set pixel rgba
            pixel.copy_from_slice(
                &<[u8; 4]>::from_hex(rgba_hex_str).expect(
                    "issue converting rgba_str to rgba u8 slice, expected format: rrggbbaa",
                ),
            );
        }
    }
}

// Entity object containing various entity states
// used to have many entities in the world
// derive Debug for debug console information
#[derive(Debug)]
struct Entity {
    width: i32,
    height: i32,
    x_position: i32,
    y_position: i32,
    x_velocity: i32,
    y_velocity: i32,
    rgba_hex_str: String,
}

impl Entity {
    // initialize a new entity with some randomness
    fn new() -> Self {
        Self {
            width: DEFAULT_ENTITY_WIDTH,
            height: DEFAULT_ENTITY_HEIGHT,
            x_position: (rand::random::<u32>()
                % ((DEFAULT_WORLD_WIDTH - DEFAULT_ENTITY_HEIGHT) as u32))
                as i32,
            y_position: (rand::random::<u32>()
                % ((DEFAULT_WORLD_HEIGHT - DEFAULT_ENTITY_HEIGHT) as u32))
                as i32,
            x_velocity: DEFAULT_ENTITY_VELOCITY * if rand::random() { 1 } else { -1 },
            y_velocity: DEFAULT_ENTITY_VELOCITY * if rand::random() { 1 } else { -1 },
            rgba_hex_str: DEFAULT_ENTITY_COLOR.to_string(),
        }
    }

    // initialize a new entity with custom settings
    fn new_custom(
        width: i32,
        height: i32,
        x_position: i32,
        y_position: i32,
        x_velocity: i32,
        y_velocity: i32,
        rgba_hex_str: String,
    ) -> Self {
        // TODO: handle more gracefully
        if rgba_hex_str.len() != 8 {
            panic!("rgba_hex_str.len() != 8, should be in the following format: rrggbbaa");
        }

        Self {
            width,
            height,
            x_position,
            y_position,
            x_velocity,
            y_velocity,
            rgba_hex_str,
        }
    }

    // update entity every frame
    fn update(&mut self, world_width: i32, world_height: i32) {
        if self.x_position <= 0 || self.x_position + self.width > world_width {
            self.x_velocity *= -1;
        }

        if self.y_position <= 0 || self.y_position + self.height > world_height {
            self.y_velocity *= -1;
        }

        self.x_position += self.x_velocity;
        self.y_position += self.y_velocity;
    }

    // draw entity every frame
    // fn draw(&self, frame: &mut [u8]) {
    // draw handled in world object
    // }
}
