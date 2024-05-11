use std::collections::VecDeque;
use image::{GenericImageView, RgbaImage, Rgba, DynamicImage};
use std::path::{Path, PathBuf};
use std::fs;

const EDGE_THRESHOLD: i32 = 255;

fn bfs(source_node: usize, sink_node: usize, parent: &mut Vec<i32>, graph: &Vec<Vec<usize>>, capacity: &Vec<Vec<i32>>) -> i32 {
    parent.fill(-1);
    parent[source_node] = -2;

    let mut q: VecDeque<(usize, i32)> = VecDeque::new();
    q.push_back((source_node, i32::MAX));

    while !q.is_empty() {
        let (current_node, flow) = q.pop_front().unwrap();

        for next in graph[current_node].iter() {
            if parent[*next] == -1 && capacity[current_node][*next] != 0 {
                parent[*next] = current_node as i32;

                let new_flow = flow.min(capacity[current_node][*next]);
                if *next == sink_node {
                    return new_flow;
                }

                q.push_back((*next, new_flow));
            }
        }
    }

    0
}

fn calculate_max_flow(node_count: usize, source_node: usize, sink_node: usize, graph: &Vec<Vec<usize>>, capacity: &mut Vec<Vec<i32>>) -> Vec<bool> {
    let mut parent: Vec<i32> = vec![0; node_count + 2];

    // let mut flow = 0;

    loop {
        let new_flow = bfs(source_node, sink_node, &mut parent, graph, capacity);
        if new_flow == 0 { break; }

        // flow += new_flow;

        let mut curr = sink_node;
        while curr != source_node {
            let prev = parent[curr];
            capacity[prev as usize][curr] -= new_flow;
            capacity[curr][prev as usize] += new_flow;
            curr = prev as usize;
        }
    }

    // Find nodes reachable from the source after the flow is calculated
    let mut visited = vec![false; node_count + 2];
    let mut q = VecDeque::new();

    visited[source_node] = true;
    q.push_back(source_node);

    while let Some(node) = q.pop_front() {
        for &next in &graph[node] {
            if !visited[next] && capacity[node][next] > 0 {
                visited[next] = true;
                q.push_back(next);
            }
        }
    }

    visited
}

fn capacity_function(first: u8, second: u8) -> i32 {
    if first == second { return EDGE_THRESHOLD; }
    (EDGE_THRESHOLD / (first as i32 - second as i32)).pow(2)
}

fn assemble_edges_from_image(grayscale: &DynamicImage,
                             graph: &mut Vec<Vec<usize>>,
                             capacity: &mut Vec<Vec<i32>>,
                             source_node: usize, sink_node: usize) {
    let (width, height) = grayscale.dimensions();

    // Construct graph based on the differences between grayscale values
    let mut current_node = 1;
    for row in 0..height {
        for col in 0..width {
            // Process node up
            if row != 0 {
                let (first, second) = (grayscale.get_pixel(col, row)[0], grayscale.get_pixel(col, row - 1)[0]);

                graph[current_node].push(current_node - width as usize);
                capacity[current_node][current_node - width as usize] = capacity_function(first, second);
            }

            // Process node up-right diagonal
            if col + 1 != width && row != 0 {
                let (first, second) = (grayscale.get_pixel(col, row)[0], grayscale.get_pixel(col + 1, row - 1)[0]);

                graph[current_node].push(current_node - width as usize + 1);
                capacity[current_node][current_node - width as usize + 1] = capacity_function(first, second);
            }

            // Process node to the right
            if col + 1 != width {
                let (first, second) = (grayscale.get_pixel(col, row)[0], grayscale.get_pixel(col + 1, row)[0]);

                graph[current_node].push(current_node + 1);
                capacity[current_node][current_node + 1] = capacity_function(first, second);
            }

            // Process node down-right diagonal
            if col + 1 != width && row + 1 != height {
                let (first, second) = (grayscale.get_pixel(col, row)[0], grayscale.get_pixel(col + 1, row + 1)[0]);

                graph[current_node].push(current_node + width as usize + 1);
                capacity[current_node][current_node + width as usize + 1] = capacity_function(first, second);
            }

            // Process node down
            if row + 1 != height {
                let (first, second) = (grayscale.get_pixel(col, row)[0], grayscale.get_pixel(col, row + 1)[0]);

                graph[current_node].push(current_node + width as usize);
                capacity[current_node][current_node + width as usize] = capacity_function(first, second);
            }

            // Process node down-left diagonal
            if col != 0 && row + 1 != height {
                let (first, second) = (grayscale.get_pixel(col, row)[0], grayscale.get_pixel(col - 1, row + 1)[0]);

                graph[current_node].push(current_node + width as usize - 1);
                capacity[current_node][current_node + width as usize - 1] = capacity_function(first, second);
            }

            // Process node to the left
            if col != 0 {
                let (first, second) = (grayscale.get_pixel(col, row)[0], grayscale.get_pixel(col - 1, row)[0]);

                graph[current_node].push(current_node - 1);
                capacity[current_node][current_node - 1] = capacity_function(first, second);
            }

            // Process node up-left diagonal
            if col != 0 && row != 0 {
                let (first, second) = (grayscale.get_pixel(col, row)[0], grayscale.get_pixel(col - 1, row - 1)[0]);

                graph[current_node].push(current_node - width as usize - 1);
                capacity[current_node][current_node - width as usize - 1] = capacity_function(first, second);
            }

            current_node += 1;
        }
    }

    // Wire in source and sink node connections
    for i in 0..width {
        let to = (i + 1) as usize;
        graph[source_node].push(to);
        capacity[source_node][to] = EDGE_THRESHOLD;

        let s = ((height - 1) * width + i) as usize;
        graph[s].push(sink_node);
        capacity[s][sink_node] = EDGE_THRESHOLD;
    }
}

fn determine_foreground(input_path: &PathBuf) {
    let mut image = image::open(input_path).unwrap();
    let grayscale = image.grayscale();

    let (width, height) = grayscale.dimensions();

    let node_count = (width * height) as usize;
    let mut graph: Vec<Vec<usize>> = vec![vec![]; node_count + 2];
    let mut capacity: Vec<Vec<i32>> = vec![vec![0; node_count + 2]; node_count + 2];
    let (source_node, sink_node) = (0usize, node_count + 1);

    assemble_edges_from_image(&grayscale, &mut graph, &mut capacity, source_node, sink_node);

    let visited_nodes = calculate_max_flow(node_count, source_node, sink_node, &graph, &mut capacity);

    // Let's save the image
    let mut foreground = RgbaImage::new(width, height);
    for row in 0..height {
        for col in 0..width {
            let pixel_value = visited_nodes[(height * row + col) as usize];

            let pixel_color = if pixel_value { Rgba([255, 0, 0, 60]) } else { Rgba([255, 255, 255, 60]) };
            foreground.put_pixel(col, row, pixel_color);
        }
    }

    image::imageops::overlay(&mut image, &mut foreground, 0, 0);

    let img_name = input_path.file_stem().unwrap().to_str().unwrap();
    let new_name = format!("{}_detection", img_name);

    let output_path = Path::new(&new_name).with_extension("png");
    image.save(output_path).unwrap();
}

fn main() {
    let paths = fs::read_dir("./test_images").unwrap();
    paths.for_each(|path| determine_foreground(&path.unwrap().path()));
}