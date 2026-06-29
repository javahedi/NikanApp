
// src/errors.rs

#[derive(Debug)]
pub enum NikanError {
    DonorNotFound(u32),
    InvalidAmount,
}