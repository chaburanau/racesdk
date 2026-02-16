use crate::ace::{AssettoCorsaEvoError, Graphics, Physics, StaticInfo};
use crate::utils::{open_shared_memory, SharedMemory};
use std::mem;
use std::ptr;

pub struct AssettoCorsaEvoClient {
    physics_memory: Option<Box<dyn SharedMemory>>,
    graphics_memory: Option<Box<dyn SharedMemory>>,
    static_memory: Option<Box<dyn SharedMemory>>,
}

impl AssettoCorsaEvoClient {
    pub fn new() -> Self {
        Self {
            physics_memory: None,
            graphics_memory: None,
            static_memory: None,
        }
    }

    pub fn connect(&mut self) -> Result<(), AssettoCorsaEvoError> {
        match (
            open_shared_memory("Local\\acpmf_physics", mem::size_of::<Physics>()),
            open_shared_memory("Local\\acpmf_graphics", mem::size_of::<Graphics>()),
            open_shared_memory("Local\\acpmf_static", mem::size_of::<StaticInfo>()),
        ) {
            (Ok(physics), Ok(graphics), Ok(static_mem)) => {
                self.physics_memory = Some(Box::new(physics));
                self.graphics_memory = Some(Box::new(graphics));
                self.static_memory = Some(Box::new(static_mem));
                Ok(())
            }
            (Err(e), _, _) | (_, Err(e), _) | (_, _, Err(e)) => {
                self.physics_memory = None;
                self.graphics_memory = None;
                self.static_memory = None;
                Err(map_shared_memory_error(e))
            }
        }
    }

    pub fn is_connected(&self) -> bool {
        self.physics_memory.is_some() && self.graphics_memory.is_some() && self.static_memory.is_some()
    }

    pub fn get_physics(&self) -> Option<Physics> {
        if let Some(ref memory) = self.physics_memory {
            let physics: Physics = unsafe { ptr::read(memory.as_slice().as_ptr() as *const Physics) };
            Some(physics)
        } else {
            None
        }
    }

    pub fn get_graphics(&self) -> Option<Graphics> {
        if let Some(ref memory) = self.graphics_memory {
            let graphics: Graphics = unsafe { ptr::read(memory.as_slice().as_ptr() as *const Graphics) };
            Some(graphics)
        } else {
            None
        }
    }

    pub fn get_static_info(&self) -> Option<StaticInfo> {
        if let Some(ref memory) = self.static_memory {
            let static_info: StaticInfo = unsafe { ptr::read(memory.as_slice().as_ptr() as *const StaticInfo) };
            Some(static_info)
        } else {
            None
        }
    }
}

fn map_shared_memory_error(err: crate::utils::SharedMemoryError) -> AssettoCorsaEvoError {
    match err {
        crate::utils::SharedMemoryError::SharedMemoryNotFound(msg) => AssettoCorsaEvoError::SharedMemoryNotFound(msg),
        crate::utils::SharedMemoryError::ConnectionFailed(msg) => AssettoCorsaEvoError::ConnectionFailed(msg),
    }
}
