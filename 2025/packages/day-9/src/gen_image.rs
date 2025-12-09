use crate::types::Point;
use std::fs::File;
use std::io::Write;

pub fn draw_polygon_to_file(polygon: &Vec<Point>, largest_tile: [Point; 4], filename: &str) -> std::io::Result<()> {
    if polygon.is_empty() {
        return Ok(());
    }

    // Find bounding box
    let min_x = polygon.iter().map(|p| p.0).min().unwrap();
    let max_x = polygon.iter().map(|p| p.0).max().unwrap();
    let min_y = polygon.iter().map(|p| p.1).min().unwrap();
    let max_y = polygon.iter().map(|p| p.1).max().unwrap();

    let orig_width = (max_x - min_x + 1) as f64;
    let orig_height = (max_y - min_y + 1) as f64;

    // Scale down by fixed factor
    const SCALE: f64 = 100.0;

    let width = (orig_width / SCALE).ceil() as usize;
    let height = (orig_height / SCALE).ceil() as usize;

    // Colors: (R, G, B)
    const WHITE: (u8, u8, u8) = (255, 255, 255);
    const BLACK: (u8, u8, u8) = (0, 0, 0);
    const RED: (u8, u8, u8) = (255, 0, 0);

    // Create pixel buffer (RGB)
    let mut pixels = vec![vec![WHITE; width]; height];

    // Helper to scale a point
    let scale_point = |x: i64, y: i64| -> (usize, usize) {
        let col = ((x - min_x) as f64 / SCALE) as usize;
        let row = ((y - min_y) as f64 / SCALE) as usize;
        (col.min(width - 1), row.min(height - 1))
    };

    // Mark each vertex with black
    for &(x, y) in polygon {
        let (col, row) = scale_point(x, y);
        pixels[row][col] = BLACK;
    }

    // Draw lines between rectangle vertices (p1 -> p3 -> p2 -> p4 -> p1)
    // largest_tile = [(x1,y1), (x2,y2), (x3,y3), (x4,y4)] where p3=(x2,y1), p4=(x1,y2)
    let [p1, p2, p3, p4] = largest_tile;
    let corners = [p1, p3, p2, p4]; // ordered to form rectangle
    for i in 0..4 {
        let start = scale_point(corners[i].0, corners[i].1);
        let end = scale_point(corners[(i + 1) % 4].0, corners[(i + 1) % 4].1);
        draw_line(&mut pixels, start, end, RED, width, height);
    }

    // Write BMP file
    write_bmp(filename, &pixels, width, height)
}

fn draw_line(
    pixels: &mut [Vec<(u8, u8, u8)>],
    start: (usize, usize),
    end: (usize, usize),
    color: (u8, u8, u8),
    width: usize,
    height: usize,
) {
    let (x0, y0) = (start.0 as isize, start.1 as isize);
    let (x1, y1) = (end.0 as isize, end.1 as isize);

    let dx = (x1 - x0).abs();
    let dy = -(y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx + dy;

    let mut x = x0;
    let mut y = y0;

    loop {
        if (x as usize) < width && (y as usize) < height {
            pixels[y as usize][x as usize] = color;
        }

        if x == x1 && y == y1 {
            break;
        }

        let e2 = 2 * err;
        if e2 >= dy {
            err += dy;
            x += sx;
        }
        if e2 <= dx {
            err += dx;
            y += sy;
        }
    }
}

fn write_bmp(filename: &str, pixels: &[Vec<(u8, u8, u8)>], width: usize, height: usize) -> std::io::Result<()> {
    let mut file = File::create(filename)?;

    // BMP rows must be padded to 4-byte boundaries
    let row_size = (width * 3 + 3) & !3;
    let pixel_data_size = row_size * height;
    let file_size = 54 + pixel_data_size;

    // BMP File Header (14 bytes)
    file.write_all(b"BM")?;                          // Signature
    file.write_all(&(file_size as u32).to_le_bytes())?; // File size
    file.write_all(&0u16.to_le_bytes())?;            // Reserved
    file.write_all(&0u16.to_le_bytes())?;            // Reserved
    file.write_all(&54u32.to_le_bytes())?;           // Pixel data offset

    // DIB Header (BITMAPINFOHEADER - 40 bytes)
    file.write_all(&40u32.to_le_bytes())?;           // Header size
    file.write_all(&(width as i32).to_le_bytes())?;  // Width
    file.write_all(&(height as i32).to_le_bytes())?; // Height (positive = bottom-up)
    file.write_all(&1u16.to_le_bytes())?;            // Color planes
    file.write_all(&24u16.to_le_bytes())?;           // Bits per pixel
    file.write_all(&0u32.to_le_bytes())?;            // Compression (none)
    file.write_all(&(pixel_data_size as u32).to_le_bytes())?; // Image size
    file.write_all(&2835u32.to_le_bytes())?;         // Horizontal resolution (72 DPI)
    file.write_all(&2835u32.to_le_bytes())?;         // Vertical resolution (72 DPI)
    file.write_all(&0u32.to_le_bytes())?;            // Colors in palette
    file.write_all(&0u32.to_le_bytes())?;            // Important colors

    // Pixel data (bottom-up, BGR format)
    let padding = vec![0u8; row_size - width * 3];
    for row in pixels.iter().rev() {
        for &(r, g, b) in row {
            file.write_all(&[b, g, r])?; // BGR order
        }
        file.write_all(&padding)?;
    }

    Ok(())
}
