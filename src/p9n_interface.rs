use crate::ps5_dualsense::AXES_DUALSENSE;
use crate::ps5_dualsense::BUTTONS_DUALSENSE;
use safe_drive::msg::common_interfaces::sensor_msgs;

pub struct PlaystationInterface<'a> {
    msg: &'a sensor_msgs::msg::Joy,
}

#[allow(dead_code)]
impl<'a> PlaystationInterface<'a> {
    pub fn new(_msg: &'a sensor_msgs::msg::Joy) -> Self {
        PlaystationInterface { msg: _msg }
    }
    pub fn set_joy_msg(&mut self, _msg: &'a sensor_msgs::msg::Joy) {
        self.msg = _msg;
    }
    fn is_vaild_button(&self, button: usize) -> bool {
        button < self.msg.buttons.as_slice().len()
    }
    fn is_vaild_axes(&self, axes: usize) -> bool {
        axes < self.msg.buttons.as_slice().len()
    }
    pub fn pressed_start(&self) -> bool {
        if self.is_vaild_button(BUTTONS_DUALSENSE::START) {
            self.msg.buttons.as_slice()[BUTTONS_DUALSENSE::START] == 1
        } else {
            false
        }
    }
    pub fn pressed_select(&self) -> bool {
        if self.is_vaild_button(BUTTONS_DUALSENSE::SELECT) {
            self.msg.buttons.as_slice()[BUTTONS_DUALSENSE::SELECT] == 1
        } else {
            false
        }
    }
    pub fn pressed_ps(&self) -> bool {
        if self.is_vaild_button(BUTTONS_DUALSENSE::PS) {
            self.msg.buttons.as_slice()[BUTTONS_DUALSENSE::PS] == 1
        } else {
            false
        }
    }
    pub fn pressed_l1(&self) -> bool {
        if self.is_vaild_button(BUTTONS_DUALSENSE::L1) {
            self.msg.buttons.as_slice()[BUTTONS_DUALSENSE::L1] == 1
        } else {
            false
        }
    }
    pub fn pressed_r1(&self) -> bool {
        if self.is_vaild_button(BUTTONS_DUALSENSE::R1) {
            self.msg.buttons.as_slice()[BUTTONS_DUALSENSE::R1] == 1
        } else {
            false
        }
    }
    pub fn pressed_r2(&self) -> bool {
        if self.is_vaild_button(BUTTONS_DUALSENSE::R2) {
            self.msg.buttons.as_slice()[BUTTONS_DUALSENSE::R2] == 1
        } else {
            false
        }
    }
    pub fn pressed_l2(&self) -> bool {
        if self.is_vaild_button(BUTTONS_DUALSENSE::L2) {
            self.msg.buttons.as_slice()[BUTTONS_DUALSENSE::L2] == 1
        } else {
            false
        }
    }
    pub fn pressed_cross(&self) -> bool {
        if self.is_vaild_button(BUTTONS_DUALSENSE::CROSS) {
            self.msg.buttons.as_slice()[BUTTONS_DUALSENSE::CROSS] == 1
        } else {
            false
        }
    }
    pub fn pressed_circle(&self) -> bool {
        if self.is_vaild_button(BUTTONS_DUALSENSE::CIRCLE) {
            self.msg.buttons.as_slice()[BUTTONS_DUALSENSE::CIRCLE] == 1
        } else {
            false
        }
    }
    pub fn pressed_triangle(&self) -> bool {
        if self.is_vaild_button(BUTTONS_DUALSENSE::TRIANGLE) {
            self.msg.buttons.as_slice()[BUTTONS_DUALSENSE::TRIANGLE] == 1
        } else {
            false
        }
    }
    pub fn pressed_square(&self) -> bool {
        if self.is_vaild_button(BUTTONS_DUALSENSE::SQUARE) {
            self.msg.buttons.as_slice()[BUTTONS_DUALSENSE::SQUARE] == 1
        } else {
            false
        }
    }
    pub fn pressed_dpad_left(&self) -> bool {
        if self.is_vaild_axes(AXES_DUALSENSE::DPAD_X) {
            self.msg.axes.as_slice()[AXES_DUALSENSE::DPAD_X] > 0.0
        } else {
            false
        }
    }
    pub fn pressed_dpad_right(&self) -> bool {
        if self.is_vaild_axes(AXES_DUALSENSE::DPAD_X) {
            self.msg.axes.as_slice()[AXES_DUALSENSE::DPAD_X] < 0.0
        } else {
            false
        }
    }
    pub fn pressed_dpad_up(&self) -> bool {
        if self.is_vaild_axes(AXES_DUALSENSE::DPAD_Y) {
            self.msg.axes.as_slice()[AXES_DUALSENSE::DPAD_Y] > 0.0
        } else {
            false
        }
    }
    pub fn pressed_dpad_down(&self) -> bool {
        if self.is_vaild_axes(AXES_DUALSENSE::DPAD_Y) {
            self.msg.axes.as_slice()[AXES_DUALSENSE::DPAD_Y] < 0.0
        } else {
            false
        }
    }
}
