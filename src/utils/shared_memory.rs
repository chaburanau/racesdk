use std::fmt;

pub trait SharedMemory {
    fn as_slice(&self) -> &[u8];
}

#[derive(Debug)]
pub enum SharedMemoryError {
    SharedMemoryNotFound(String),
    ConnectionFailed(String),
}

impl std::error::Error for SharedMemoryError {}

impl fmt::Display for SharedMemoryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SharedMemoryError::SharedMemoryNotFound(msg) => write!(f, "Shared memory not found: {}", msg),
            SharedMemoryError::ConnectionFailed(msg) => write!(f, "Connection failed: {}", msg),
        }
    }
}

#[cfg(target_os = "windows")]
pub use windows_implemetation::open_shared_memory;

#[cfg(target_os = "windows")]
mod windows_implemetation {
    use windows::Win32::Foundation::CloseHandle;
    use windows::Win32::Foundation::GetLastError;
    use windows::Win32::Foundation::HANDLE;
    use windows::Win32::Foundation::INVALID_HANDLE_VALUE;
    use windows::Win32::System::Memory::FILE_MAP_READ;
    use windows::Win32::System::Memory::MEMORY_MAPPED_VIEW_ADDRESS;
    use windows::Win32::System::Memory::MapViewOfFile;
    use windows::Win32::System::Memory::OpenFileMappingW;
    use windows::Win32::System::Memory::UnmapViewOfFile;
    use windows::core::HSTRING;
    use windows::core::PCWSTR;

    use super::SharedMemory;
    use super::SharedMemoryError;

    pub struct WindowsSharedMemory {
        size: usize,
        handle: HANDLE,
        memory: MEMORY_MAPPED_VIEW_ADDRESS,
    }

    impl WindowsSharedMemory {
        pub fn open(name: &str, size: usize) -> Result<Self, SharedMemoryError> {
            let h_name = HSTRING::from(name);
            let wide_name = PCWSTR::from_raw(h_name.as_ptr());

            let handle = match unsafe { OpenFileMappingW(FILE_MAP_READ.0, false, wide_name) } {
                Ok(handle) => handle,
                Err(error) => return Err(SharedMemoryError::SharedMemoryNotFound(format!("{} (Error code: {})", name, error))),
            };

            let memory = unsafe { MapViewOfFile(handle, FILE_MAP_READ, 0, 0, size) };

            if memory.Value.is_null() {
                let error = unsafe { GetLastError() };
                unsafe {
                    let _ = CloseHandle(handle);
                };
                return Err(SharedMemoryError::ConnectionFailed(format!("Failed to map view (Error code: {})", error.0)));
            }

            Ok(Self {
                size: size,
                handle: handle,
                memory: memory,
            })
        }

        pub fn as_slice(&self) -> &[u8] {
            unsafe { std::slice::from_raw_parts(self.memory.Value as *const u8, self.size) }
        }
    }

    impl Drop for WindowsSharedMemory {
        fn drop(&mut self) {
            if !self.memory.Value.is_null() {
                unsafe {
                    let _ = UnmapViewOfFile(self.memory);
                };
            }
            if !self.handle.0.is_null() && self.handle != INVALID_HANDLE_VALUE {
                unsafe {
                    let _ = CloseHandle(self.handle);
                };
            }
        }
    }

    impl SharedMemory for WindowsSharedMemory {
        fn as_slice(&self) -> &[u8] {
            self.as_slice()
        }
    }

    pub fn open_shared_memory(name: &str, size: usize) -> Result<WindowsSharedMemory, SharedMemoryError> {
        WindowsSharedMemory::open(name, size)
    }
}
