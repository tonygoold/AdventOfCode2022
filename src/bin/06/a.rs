use aoc::comms::Datastream;

fn main() {
    let input = aoc::read_line(&aoc::input_arg());
    let stream = Datastream::new(&input);
    if let Some(i) = stream.start_packet_offset() {
        println!("The packet starts at offset {}", i);
    } else {
        println!("No packet start found");
    }
}