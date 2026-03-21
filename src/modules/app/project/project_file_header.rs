
use bytemuck::{Pod, Zeroable};
use bytemuck::PodCastError;

use crate::modules::app::project::project_id::ProjectID;

use crate::modules::consts::MAGIC_BYTES;

#[repr(C)]
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(Pod, Zeroable)]
pub struct ProjectFileHeader {
    pub project_id: [u8; 16],
    pub magic: [u8; 6],
    pub reserv: [u8; 2],
    pub version: u32,
}

impl ProjectFileHeader {
    pub fn new(version: u32, project_id: &ProjectID) -> Self {
        Self { 
            project_id: *project_id.as_bytes(),
            magic: MAGIC_BYTES, 
            reserv: [0; 2], 
            version,
        }
    }

    pub fn as_bytes(&self) -> &[u8] {
        bytemuck::bytes_of(self)
    }

    pub fn try_from_bytes(bytes: &[u8]) -> Result<&Self, PodCastError>  {
        bytemuck::try_from_bytes(bytes)
    } 
}
