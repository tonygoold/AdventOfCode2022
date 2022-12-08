use aoc::comms::Datastream;

fn main() {
    let input = aoc::read_line(&aoc::input_arg());
    let stream = Datastream::new(&input);
    if let Some(i) = stream.start_message_offset() {
        println!("The message starts at offset {}", i);
    } else {
        println!("No message start found");
    }
}