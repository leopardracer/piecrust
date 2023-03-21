// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// Copyright (c) DUSK NETWORK. All rights reserved.

use blake3::hash;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use wasmer::Module;

use crate::error::Error;
use crate::instance::Store;
use crate::store::COMPILED_DIR;
use crate::Error::ModuleCacheError;

#[derive(Clone)]
pub struct WrappedModule {
    serialized: Arc<Vec<u8>>,
}

impl WrappedModule {
    pub fn new<B: AsRef<[u8]>, C: AsRef<[u8]>>(
        bytecode: B,
        objectcode: Option<C>,
    ) -> Result<Self, Error> {
        let store = Store::new_store();
        let serialized = match objectcode {
            Some(obj) => obj.as_ref().to_vec(),
            _ => {
                let module = Module::new(&store, bytecode.as_ref())?;
                module.serialize()?.to_vec()
            }
        };

        Ok(WrappedModule {
            serialized: Arc::new(serialized),
        })
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.serialized
    }
}
