use minifb::{Key, Scale, Window, WindowOptions};

const WIDTH: usize = 500;
const HEIGHT: usize = 300;

fn main() -> anyhow::Result<()> {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut window = Window::new(
        "Code Flags",
        WIDTH,
        HEIGHT,
        WindowOptions {
            scale: Scale::X2,
            ..Default::default()
        },
    )?;

    // top to bottom
    let mut stripe_colors = [
        rgb(228, 3, 3),
        rgb(255, 140, 0),
        rgb(255, 237, 0),
        rgb(0, 128, 38),
        rgb(0, 77, 255),
        rgb(117, 7, 135),
    ];

    // left to right
    let mut tri_colors = [
        rgb(255, 255, 255),
        rgb(255, 175, 200),
        rgb(116, 215, 238),
        rgb(97, 57, 21),
        rgb(0, 0, 0),
    ];

    let mut cycle_colors = false;

    render_flag(&mut buffer, &stripe_colors, &tri_colors);
    window.limit_update_rate(Some(std::time::Duration::from_nanos(50_000_000)));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        if window.is_key_released(Key::Space) {
            cycle_colors = !cycle_colors;
        }

        if cycle_colors {
            stripe_colors.rotate_right(1);
            tri_colors.rotate_right(1);
            render_flag(&mut buffer, &stripe_colors, &tri_colors);
        }

        window.update_with_buffer(&buffer, WIDTH, HEIGHT)?;
    }

    anyhow::Ok(())
}

fn rgb(r: u8, g: u8, b: u8) -> u32 {
    let (r, g, b) = (u32::from(r), u32::from(g), u32::from(b));
    (255 << 24) | (r << 16) | (g << 8) | b
}

fn render_flag(buffer: &mut [u32], stripe_colors: &[u32; 6], tri_colors: &[u32; 5]) {
    let mut render_span = |y: usize, x_start: i32, x_len: i32, color: u32| {
        let mut x_start = x_start;
        let mut x_len = x_len;
        if x_start < 0 {
            x_len += x_start;
            x_start = 0;
        }
        if x_len <= 0 {
            return;
        }

        let buffer_idx = y * WIDTH + (x_start as usize);
        buffer[buffer_idx..(buffer_idx + (x_len as usize))].fill(color);
    };

    {
        // stripes
        for y in 0..HEIGHT {
            render_span(y, 0, WIDTH as i32, stripe_colors[y / (HEIGHT / 6)]);
        }
    }

    {
        // left-most triangle
        for y in 80..(HEIGHT / 2) {
            render_span(y, 0, (y - 80) as i32, tri_colors[0]);
        }
        for y in (HEIGHT / 2)..(HEIGHT - 80) {
            render_span(y, 0, ((HEIGHT - 80) - y) as i32, tri_colors[0]);
        }
    }

    let mut render_tristrip = |span_idx: usize, x_mul: i32, y_inset: usize| {
        const TRI_SPANS: [i32; 5] = [80, 40, 40, 40, 40];

        let mut x = TRI_SPANS[span_idx] * x_mul;
        for y in y_inset..(HEIGHT / 2) {
            render_span(y, x, TRI_SPANS[span_idx], tri_colors[span_idx]);
            x += 1;
        }
        for y in (HEIGHT / 2)..(HEIGHT - y_inset) {
            render_span(y, x, TRI_SPANS[span_idx], tri_colors[span_idx]);
            x -= 1;
        }
    };

    render_tristrip(1, -1, 40);
    render_tristrip(2, -1, 0);
    render_tristrip(3, 0, 0);
    render_tristrip(4, 1, 0);
}
