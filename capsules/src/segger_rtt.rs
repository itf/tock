



#[repr(C)]
struct segger_rtt_memory {
	id: [u8; 16],
	number_up_buffers: u32,
	number_down_buffers: u32,
	up_buffer: segger_rtt_buffer,
	down_buffer: segger_rtt_buffer,
}

#[repr(C)]
struct segger_rtt_buffer {
	name: &'static [u8],
	buffer: &'static [u8],
	length: u32,
	write_position: u32,
	read_position: u32,
	flags: u32,
}
