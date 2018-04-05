mod channels;
mod clock_times;


// pub enum Msg {
// 	Hello, HowAreYou, Fine,
// }
// impl channels::Message for Msg {}

fn main() {
    channels::test();

    // let mut a: channels::Channel<clock_times::Trivial, Msg>
    // 	= channels::Channel::new();

    // a.single_send(Msg::Hello);
    // {
    // 	let handle = a.group_handle();
    // 	handle.group_send(Msg::Hello);
    // 	handle.group_send(Msg::HowAreYou);
    // }
}
