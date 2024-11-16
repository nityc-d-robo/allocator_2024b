mod p9n_interface;
mod ps5_dualsense;
#[allow(unused_imports)]
use safe_drive::{
    context::Context,
    error::DynError,
    logger::Logger,
    msg::common_interfaces::{sensor_msgs, sensor_msgs::msg::Joy},
    pr_info,
    topic::publisher::Publisher,
    topic::subscriber::TakenMsg,
};

use core::cell::RefCell;

fn main() -> Result<(), DynError> {
    let _logger = Logger::new("allocator_2024b");
    let ctx = Context::new()?;
    let mut selector = ctx.create_selector()?;
    let node = ctx.create_node("allocator_2024b", None, Default::default())?;
    let mut select_button_state = false;
    let mut ps_button_state = false;

    let c_joy = node.create_subscriber::<sensor_msgs::msg::Joy>("joy", None)?;

    let mut r_joys = RefCell::new((
        [
            node.create_publisher::<Joy>("rjoy1", None)?,
            node.create_publisher::<Joy>("rjoy2_3", None)?,
        ],
        0,
    ));
    selector.add_subscriber(
        c_joy,
        Box::new(move |msg| {
            worker(msg, &mut r_joys, &mut select_button_state, &mut ps_button_state);
        }),
    );

    loop {
        selector.wait()?;
    }
}

fn worker(joy_msg: TakenMsg<Joy>, _robocons: &mut RefCell<([Publisher<Joy>; 2], usize)>, select_button_state: &mut bool, ps_button_state: &mut bool) {
    let binding = sensor_msgs::msg::Joy::new().unwrap();
    let mut joy_c = p9n_interface::PlaystationInterface::new(&binding);
    joy_c.set_joy_msg(&joy_msg);

    if joy_c.pressed_ps() && !*ps_button_state{
        pr_info!(Logger::new("p9n_interface_2024"), "ps");
        *ps_button_state = true;
        let robocons = _robocons.get_mut();
        robocons.1 = 0;
    }
    if !joy_c.pressed_ps() && *ps_button_state{
        *ps_button_state = false;
    }
    if joy_c.pressed_select() && !*select_button_state{
        pr_info!(Logger::new("p9n_interface_2024"), "select");
        *select_button_state = true;
        let robocons = _robocons.get_mut();
        robocons.1 = 1;
    }
    if !joy_c.pressed_select() && *select_button_state{
        *select_button_state = false;
    }
    /*if joy_c.pressed_circle() {
        let robocons = _robocons.get_mut();
        robocons.1 = 0;
    }
    if joy_c.pressed_square() {
        let robocons = _robocons.get_mut();
        robocons.1 = 1;
    }*/

    let pointer = _robocons.borrow().1;
    let unpointer = (pointer + 1) % _robocons.borrow().0.len();
    let robocons = _robocons.get_mut();

    robocons.0[pointer].send(&joy_msg).unwrap();
    robocons.0[unpointer].send(&Joy::new().unwrap()).unwrap()
}
