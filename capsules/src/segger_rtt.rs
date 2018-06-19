
use kernel::common::cells::{TakeCell, OptionalCell};
use kernel::hil;

#[repr(C)]
pub struct segger_rtt_memory {
	id: [u8; 16],
	number_up_buffers: u32,
	number_down_buffers: u32,
	up_buffer: segger_rtt_buffer,
	down_buffer: segger_rtt_buffer,
}

#[repr(C)]
pub struct segger_rtt_buffer {
	name: &'static [u8],
	buffer: &'static [u8],
	length: u32,
	write_position: u32,
	read_position: u32,
	flags: u32,
}


pub struct SeggerRtt {
    // state: Cell<State>,
    config: TakeCell<'static, segger_rtt_memory>,
    up_buffer: TakeCell<'static, [u8]>,
    down_buffer: TakeCell<'static, [u8]>,
    client: OptionalCell<&'static hil::uart::Client>,
}

impl SeggerRtt {
	pub fn new(config: &'static mut segger_rtt_memory, up_buffer: &'static mut [u8],
        down_buffer: &'static mut [u8] ) -> SeggerRtt {
		SeggerRtt {
			config: TakeCell::new(config),
			up_buffer: TakeCell::new(up_buffer),
			down_buffer: TakeCell::new(down_buffer),
			client: OptionalCell::empty(),
		}
	}
}
