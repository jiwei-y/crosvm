// Copyright 2023 The ChromiumOS Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod descriptor;
pub mod event;
pub mod memory_mapping;
pub mod pipe;
pub mod shm;
pub mod tube;
pub mod wait_context;

pub use memory_mapping::MemoryMapping;
pub use shm::SharedMemory;
