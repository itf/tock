
use kernel::common::cells::{TakeCell, OptionalCell};
use kernel::hil;
use kernel::hil::time::Frequency;

pub static mut UP_BUFFER: [u8; 1024] = [0; 1024];
pub static mut DOWN_BUFFER: [u8; 32] = [0; 32];
// pub static mut SEGGER_RTT_MEMORY: segger_rtt_memory =

#[repr(C)]
pub struct SeggerRttMemory {
	id: [u8; 16],
	number_up_buffers: u32,
	number_down_buffers: u32,
	up_buffer: SeggerRttBuffer,
	down_buffer: SeggerRttBuffer,
}

#[repr(C)]
pub struct SeggerRttBuffer {
	name: u32,
	buffer: u32,
	length: u32,
	write_position: u32,
	read_position: u32,
	flags: u32,
}

impl SeggerRttMemory {
	pub fn new(up_buffer_name: &'static [u8],
		up_buffer: &'static [u8],
		down_buffer_name: &'static [u8],
		down_buffer: &'static [u8]) -> SeggerRttMemory {
		SeggerRttMemory {
			id: [0x53, 0x45, 0x47, 0x47, 0x45, 0x52, 0x20, 0x52, 0x54, 0x54, 0, 0, 0, 0, 0, 0],
			number_up_buffers: 1,
			number_down_buffers: 1,
			up_buffer: SeggerRttBuffer {
				name: up_buffer_name as *const [u8] as *const () as u32,
				buffer: up_buffer as *const [u8] as *const () as u32,
				length: 1024,
				write_position: 0,
				read_position: 0,
				flags: 0
			},
			down_buffer: SeggerRttBuffer {
				name: down_buffer_name as *const [u8] as *const () as u32,
				buffer: down_buffer as *const [u8] as *const () as u32,
				length: 32,
				write_position: 0,
				read_position: 0,
				flags: 0
			}
		}
	}
}


pub struct SeggerRtt<'a, A: hil::time::Alarm + 'a> {
    // state: Cell<State>,
    alarm: &'a A,
    config: TakeCell<'static, SeggerRttMemory>,
    up_buffer: TakeCell<'static, [u8]>,
    down_buffer: TakeCell<'static, [u8]>,
    client: OptionalCell<&'static hil::uart::Client>,
    client_buffer: TakeCell<'static, [u8]>,
}

impl<'a, A: hil::time::Alarm + 'a> SeggerRtt<'a, A> {
	pub fn new(alarm: &'a A, config: &'static mut SeggerRttMemory, up_buffer: &'static mut [u8],
        down_buffer: &'static mut [u8] ) -> SeggerRtt<'a, A> {
		SeggerRtt {
			alarm: alarm,
			config: TakeCell::new(config),
			up_buffer: TakeCell::new(up_buffer),
			down_buffer: TakeCell::new(down_buffer),
			client: OptionalCell::empty(),
			client_buffer: TakeCell::empty(),
		}
	}

	// pub fn say (&self) {
	// 	self.up_buffer.map(|buffer| {
	// 		self.config.map(|config| {
	// 			// debug_gpio!(0, toggle);
	// 			let mut index = config.up_buffer.write_position;

	// 			buffer[index as usize] = 55;
	// 			buffer[index as usize +1] = 10;

	// 			index += 2;
	// 			if index == config.up_buffer.length {
	// 				index = 0;
	// 			}
	// 			config.up_buffer.write_position = index;
	// 		});
	// 	});



	// 		// config.a = 0x33;
	// 		// config.d = config.a  +config.b;
	// 	// });
	// }
}

impl<'a, A: hil::time::Alarm + 'a> hil::uart::UART for SeggerRtt<'a, A> {
    fn set_client(&self, client: &'static hil::uart::Client) {
        self.client.set(client);
    }

    fn init(&self, params: hil::uart::UARTParams) {

    }

    fn transmit(&self, tx_data: &'static mut [u8], tx_len: usize) {
    	self.up_buffer.map(|buffer| {
    		self.config.map(|config| {
    			let mut index = config.up_buffer.write_position as usize;
    			let buffer_len = config.up_buffer.length as usize;

    			for i in 0..tx_len {
    				buffer[(i+index) % buffer_len] = tx_data[i];
    			}

    			// buffer[index as usize] = 55;
    			// buffer[index as usize +1] = 10;

    			index = (index + tx_len) % buffer_len;
    			// if index >= config.up_buffer.length {
    			// 	index = 0;
    			// }
    			config.up_buffer.write_position = index as u32;
    		});
    	});

    	self.client_buffer.replace(tx_data);

    	let interval = (100 as u32) * <A::Frequency>::frequency() / 1000000;
    	let tics = self.alarm.now().wrapping_add(interval);
    	self.alarm.set_alarm(tics);



    }

    fn receive(&self, rx_buf: &'static mut [u8], rx_len: usize) {
    }

    fn abort_receive(&self) {
    }
}

impl<'a, A: hil::time::Alarm + 'a> hil::time::Client for SeggerRtt<'a, A> {
    fn fired(&self) {
        // self.buffer.take().map(|buffer| {
        //     // Turn on i2c to send commands.
        //     self.i2c.enable();

        //     buffer[0] = 0x02 as u8;
        //     self.i2c.write_read(buffer, 1, 2);
        //     self.state.set(State::ReadingLI);
        // });
        self.client.map(|client| {
	        self.client_buffer.take().map(|buffer| {
	            client.transmit_complete(
	                buffer,
	                hil::uart::Error::CommandComplete,
	            );
	        });
        });
    }
}
