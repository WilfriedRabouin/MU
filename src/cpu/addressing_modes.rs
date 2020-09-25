use super::*;

pub(super) struct Immediate;
pub(super) struct ZeroPage;
pub(super) struct ZeroPageX;
pub(super) struct ZeroPageY;
pub(super) struct Absolute;
pub(super) struct AbsoluteX;
pub(super) struct AbsoluteY;
pub(super) struct IndirectX;
pub(super) struct IndirectY;

pub(super) trait AddressingMode {
	fn get_address(emulator: &mut Emulator) -> u16;
}

impl AddressingMode for Immediate {
	fn get_address(emulator: &mut Emulator) -> u16 {
		let address = emulator.cpu.pc;
		emulator.cpu.increment_pc(1);
		address
	}
}

impl AddressingMode for ZeroPage {
	fn get_address(emulator: &mut Emulator) -> u16 {
		get_next8(emulator) as _
	}
}

impl AddressingMode for ZeroPageX {
	fn get_address(emulator: &mut Emulator) -> u16 {
		get_next8(emulator).wrapping_add(emulator.cpu.x) as _
	}
}

impl AddressingMode for ZeroPageY {
	fn get_address(emulator: &mut Emulator) -> u16 {
		get_next8(emulator).wrapping_add(emulator.cpu.y) as _
	}
}

impl AddressingMode for Absolute {
	fn get_address(emulator: &mut Emulator) -> u16 {
		get_next16(emulator)
	}
}

impl AddressingMode for AbsoluteX {
	fn get_address(emulator: &mut Emulator) -> u16 {
		get_next16(emulator).wrapping_add(emulator.cpu.x as _)
	}
}

impl AddressingMode for AbsoluteY {
	fn get_address(emulator: &mut Emulator) -> u16 {
		get_next16(emulator).wrapping_add(emulator.cpu.y as _)
	}
}

impl AddressingMode for IndirectX {
	fn get_address(emulator: &mut Emulator) -> u16 {
		let address = get_next8(emulator).wrapping_add(emulator.cpu.x);
		let low_byte = read8(emulator, address as _) as u16;
		let high_byte = read8(emulator, address.wrapping_add(1) as _) as u16;
		(high_byte << 8) | low_byte
	}
}

impl AddressingMode for IndirectY {
	fn get_address(emulator: &mut Emulator) -> u16 {
		let address = get_next8(emulator);
		let low_byte = read8(emulator, address as _) as u16;
		let high_byte = read8(emulator, address.wrapping_add(1) as _) as u16;
		let value = (high_byte << 8) | low_byte;
		value.wrapping_add(emulator.cpu.y as _)
	}
}
