use nix::sys::ptrace::{self, AddressType};
use nix::unistd::Pid;

pub struct Breakpoint {
    pid: Pid,
    address: AddressType,
    enabled: bool,
    saved_data: u8,
}

impl Breakpoint {
    pub fn new(pid: Pid, address: usize) -> Self {
        let address = address as AddressType;
        Self {
            pid,
            address,
            enabled: false,
            saved_data: 0,
        }
    }

    pub fn enable(&mut self) -> nix::Result<()> {
        let data = ptrace::read(self.pid, self.address)? as u64;

        // Save the first byte of the instruction at the breakpoint address.
        self.saved_data = (data & 0xff) as u8;

        // Replace the first byte of the instruction with the INT3 opcode.
        const INT3: u64 = 0xcc;
        let data = (data & !0xff) | INT3;

        ptrace::write(self.pid, self.address, data as i64)?;
        self.enabled = true;
        Ok(())
    }

    pub fn _disable(&mut self) -> nix::Result<()> {
        let data = ptrace::read(self.pid, self.address)? as u64;

        // Restore the first byte of the instruction at the breakpoint address.
        let restored_data = (data & !0xff) | self.saved_data as u64;

        ptrace::write(self.pid, self.address, restored_data as i64)?;
        self.enabled = false;
        Ok(())
    }

    pub fn _is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn _get_address(&self) -> usize {
        self.address as usize
    }
}
