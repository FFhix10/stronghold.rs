// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{
    crypto_utils::crypto_box::BoxProvider,
    locked_memory::{
        LockedConfiguration::{self, *},
        LockedMemory,
        MemoryError::{self, *},
        ProtectedConfiguration::*,
        ProtectedMemory,
    },
    memories::{buffer::Buffer, file_memory::FileMemory},
    types::Bytes,
};
use core::fmt::{self, Debug, Formatter};
use crypto::hashes::sha;
use zeroize::Zeroize;

static IMPOSSIBLE_CASE: &'static str = "NonContiguousMemory: this case should not happen if allocated properly";

// Currently we only support data of 32 bytes in noncontiguous memory
const NC_DATA_SIZE: usize = 32;

// NONCONTIGUOUS MEMORY
/// Shards of memory which composes a non contiguous memory
enum MemoryShard<T: Zeroize + Bytes, P: BoxProvider> {
    // EncryptedFileShard(FileMemory<P>),
    // EncryptedRamShard(EncryptedRam<P>),
    Zeroed(),
    FileShard(FileMemory<P>),
    RamShardedType(Buffer<T>),
}
use MemoryShard::*;

/// NonContiguousMemory only works on data which size corresponds to the hash primitive we use. In our case we use it to
/// store keys hence the size of the data depends on the chosen box provider
pub struct NonContiguousMemory<P: BoxProvider> {
    shard1: MemoryShard<u8, P>,
    shard2: MemoryShard<u8, P>,
    config: LockedConfiguration<P>,
}

impl<P: BoxProvider> LockedMemory<u8, P> for NonContiguousMemory<P> {
    /// Writes the payload into a LockedMemory then locks it
    fn alloc(payload: &[u8], config: LockedConfiguration<P>) -> Result<Self, MemoryError> {
        NonContiguousMemory::check_config(&config)?;
        let random = P::random_vec(NC_DATA_SIZE).expect("Failed to generate random vec");
        let mut digest = [0u8; NC_DATA_SIZE];

        sha::SHA256(&random, &mut digest);
        digest = xor(&digest, payload);

        let buf1 = Buffer::alloc(&random, BufferConfig(NC_DATA_SIZE))?;
        let shard1 = RamShardedType(buf1);
        let shard2 = match config {
            NCRamAndFileConfig(_) => {
                let fmem = FileMemory::alloc(&digest, FileConfig(Some(NC_DATA_SIZE)))?;
                FileShard(fmem)
            }
            NCRamConfig(_) => {
                let buf2 = Buffer::alloc(&digest, BufferConfig(NC_DATA_SIZE))?;
                RamShardedType(buf2)
            }
            _ => panic!("{}", IMPOSSIBLE_CASE),
        };
        Ok(NonContiguousMemory { shard1, shard2, config })
    }

    /// Locks the memory and possibly reallocates
    fn lock(mut self, payload: Buffer<u8>, config: LockedConfiguration<P>) -> Result<Self, MemoryError> {
        self.dealloc()?;
        NonContiguousMemory::alloc(&payload.borrow(), config)
    }

    /// Unlocks the memory and returns an unlocked Buffer
    // To retrieve secret value you xor the hash contained in shard1 with value in shard2
    fn unlock(&self, _config: LockedConfiguration<P>) -> Result<Buffer<u8>, MemoryError> {
        let mut data1 = [0u8; NC_DATA_SIZE];
        sha::SHA256(&self.get_buffer_from_shard1().borrow(), &mut data1);

        let data = match &self.shard2 {
            RamShardedType(b2) => xor(&data1, &b2.borrow()),
            FileShard(fm) => {
                let buf = fm.unlock(FileConfig(None)).expect("Failed to unlock file memory");
                let x = xor(&data1, &buf.borrow());
                x
            }
            _ => panic!("{}", IMPOSSIBLE_CASE),
        };
        Buffer::alloc(&data, BufferConfig(NC_DATA_SIZE))
    }
}

impl<P: BoxProvider> NonContiguousMemory<P> {
    // Check that the given configurations are correct for allocation
    fn check_config(config: &LockedConfiguration<P>) -> Result<(), MemoryError> {
        match config {
            NCRamAndFileConfig(Some(size)) => {
                if *size != NC_DATA_SIZE {
                    Err(NCSizeNotAllowed)
                } else {
                    Ok(())
                }
            }
            NCRamConfig(Some(size)) => {
                if *size != NC_DATA_SIZE {
                    Err(NCSizeNotAllowed)
                } else {
                    Ok(())
                }
            }
            _ => Err(ConfigurationNotAllowed),
        }
    }

    fn get_buffer_from_shard1(&self) -> &Buffer<u8> {
        match &self.shard1 {
            RamShardedType(buf) => buf,
            _ => panic!("{}", IMPOSSIBLE_CASE),
        }
    }

    // Refresh the shards to increase security, may be called every _n_ seconds or
    // punctually
    fn refresh(self) -> Result<Self, MemoryError> {
        NonContiguousMemory::check_config(&self.config)?;
        let random = P::random_vec(NC_DATA_SIZE).expect("Failed to generate random vec");
        // let hash_of_random = [0u8; NC_DATA_SIZE];

        // Refresh shard1
        let data_of_old_shard1: &[u8] = &self.get_buffer_from_shard1().borrow();
        let new_data1 = xor(data_of_old_shard1, &random);
        let new_shard1 = RamShardedType(Buffer::alloc(&new_data1, BufferConfig(NC_DATA_SIZE))?);

        let new_shard2;
        let new_config;
        let mut hash_of_old_shard1 = [0u8; NC_DATA_SIZE];
        let mut hash_of_new_shard1 = [0u8; NC_DATA_SIZE];
        sha::SHA256(data_of_old_shard1, &mut hash_of_old_shard1);
        sha::SHA256(&new_data1, &mut hash_of_new_shard1);

        match &self.shard2 {
            RamShardedType(b2) => {
                let new_data2 = xor(&b2.borrow(), &hash_of_old_shard1);
                let new_data2 = xor(&new_data2, &hash_of_new_shard1);
                new_shard2 = RamShardedType(Buffer::alloc(&new_data2, BufferConfig(NC_DATA_SIZE))?);
                new_config = NCRamConfig(Some(NC_DATA_SIZE));
            }
            FileShard(fm) => {
                let buf = fm.unlock(FileConfig(None)).expect("Failed to unlock file memory");
                let new_data2 = xor(&buf.borrow(), &hash_of_old_shard1);
                let new_data2 = xor(&new_data2, &hash_of_new_shard1);
                let new_fm = FileMemory::alloc(&new_data2, FileConfig(Some(NC_DATA_SIZE)))?;
                new_shard2 = FileShard(new_fm);
                // new_shard2 = FileShard(fm.lock(new_data2, FileConfig(Some(NC_DATA_SIZE)))?);
                new_config = NCRamAndFileConfig(Some(NC_DATA_SIZE));
            }
            _ => panic!("{}", IMPOSSIBLE_CASE),
        };

        Ok(NonContiguousMemory {
            shard1: new_shard1,
            shard2: new_shard2,
            config: new_config,
        })
    }
}

fn xor(a: &[u8], b: &[u8]) -> [u8; NC_DATA_SIZE] {
    let mut ouput = [0u8; NC_DATA_SIZE];
    for i in 0..NC_DATA_SIZE {
        ouput[i] = a[i] ^ b[i]
    }
    ouput
}

impl<P: BoxProvider> Debug for NonContiguousMemory<P> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "hidden")
    }
}

//##### Zeroize
impl<T: Zeroize + Bytes, P: BoxProvider> Zeroize for MemoryShard<T, P> {
    fn zeroize(&mut self) {
        match self {
            Zeroed() => (),
            FileShard(fm) => fm.zeroize(),
            RamShardedType(buf) => buf.zeroize(),
        }
    }
}

impl<P: BoxProvider> Zeroize for NonContiguousMemory<P> {
    fn zeroize(&mut self) {
        self.shard1.zeroize();
        self.shard2.zeroize();
        self.config = LockedConfiguration::ZeroedConfig;
    }
}

impl<P: BoxProvider> Drop for NonContiguousMemory<P> {
    fn drop(&mut self) {
        self.zeroize();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto_utils::provider::Provider;

    #[test]
    fn test_functionality_full_ram() {
        // Check alloc
        let data = Provider::random_vec(NC_DATA_SIZE).unwrap();
        let ncm = NonContiguousMemory::<Provider>::alloc(&data, NCRamConfig(Some(NC_DATA_SIZE)));
        assert!(ncm.is_ok());
        let ncm = ncm.unwrap();
        let buf = ncm.unlock(NCRamConfig(None));
        assert!(buf.is_ok());
        let buf = buf.unwrap();
        assert_eq!((&*buf.borrow()), &data);

        // Check locking
        let data = Provider::random_vec(NC_DATA_SIZE).unwrap();
        let buf = Buffer::alloc(&data, BufferConfig(NC_DATA_SIZE));
        assert!(buf.is_ok());
        let ncm = ncm.lock(buf.unwrap(), NCRamConfig(Some(NC_DATA_SIZE)));
        assert!(ncm.is_ok());
        let ncm = ncm.unwrap();
        let buf = ncm.unlock(NCRamConfig(None));
        assert!(buf.is_ok());
        let buf = buf.unwrap();
        assert_eq!((&*buf.borrow()), &data);
    }

    #[test]
    fn test_functionality_ram_file() {
        // Check alloc
        let data = Provider::random_vec(NC_DATA_SIZE).unwrap();
        let ncm = NonContiguousMemory::<Provider>::alloc(&data, NCRamAndFileConfig(Some(NC_DATA_SIZE)));
        assert!(ncm.is_ok());
        let ncm = ncm.unwrap();
        let buf = ncm.unlock(NCRamAndFileConfig(None));
        assert!(buf.is_ok());
        let buf = buf.unwrap();
        assert_eq!((&*buf.borrow()), &data);

        // Check locking
        let data = Provider::random_vec(NC_DATA_SIZE).unwrap();
        let buf = Buffer::alloc(&data, BufferConfig(NC_DATA_SIZE));
        assert!(buf.is_ok());
        let ncm = ncm.lock(buf.unwrap(), NCRamAndFileConfig(Some(NC_DATA_SIZE)));
        assert!(ncm.is_ok());
        let ncm = ncm.unwrap();
        let buf = ncm.unlock(NCRamAndFileConfig(None));
        assert!(buf.is_ok());
        let buf = buf.unwrap();
        assert_eq!((&*buf.borrow()), &data);
    }

    #[test]
    fn test_refresh() {
        let data = Provider::random_vec(NC_DATA_SIZE).unwrap();
        let ncm = NonContiguousMemory::<Provider>::alloc(&data, NCRamAndFileConfig(Some(NC_DATA_SIZE)));
        assert!(ncm.is_ok());
        let ncm = ncm.unwrap();

        let shard1_before_refresh = ncm.get_buffer_from_shard1().clone();
        let shard2_before_refresh = if let FileShard(fm) = &ncm.shard2 {
            fm.unlock(FileConfig(None)).unwrap()
        } else {
            panic!("{}", IMPOSSIBLE_CASE)
        };

        let ncm = ncm.refresh();
        assert!(ncm.is_ok());
        let ncm = ncm.unwrap();

        let shard1_after_refresh = ncm.get_buffer_from_shard1();
        let shard2_after_refresh = if let FileShard(fm) = &ncm.shard2 {
            fm.unlock(FileConfig(None)).unwrap()
        } else {
            panic!("{}", IMPOSSIBLE_CASE)
        };

        // Check that secrets is still ok after refresh
        let buf = ncm.unlock(NCRamConfig(None));
        assert!(buf.is_ok());
        let buf = buf.unwrap();
        assert_eq!((&*buf.borrow()), &data);

        // Check that refresh change the shards
        assert_ne!(&*shard1_before_refresh.borrow(), &*shard1_after_refresh.borrow());
        assert_ne!(&*shard2_before_refresh.borrow(), &*shard2_after_refresh.borrow());
    }

    #[test]
    // Checking that the shards don't contain the data
    fn test_lock_security() {
        // With full Ram
        let data = Provider::random_vec(NC_DATA_SIZE).unwrap();
        let ncm = NonContiguousMemory::<Provider>::alloc(&data, NCRamConfig(Some(NC_DATA_SIZE)));
        assert!(ncm.is_ok());
        let ncm = ncm.unwrap();

        if let RamShardedType(buf) = &ncm.shard1 {
            assert_ne!(&*buf.borrow(), &data);
        }
        if let RamShardedType(buf) = &ncm.shard2 {
            assert_ne!(&*buf.borrow(), &data);
        }

        // With Ram and File
        let data = Provider::random_vec(NC_DATA_SIZE).unwrap();
        let ncm = NonContiguousMemory::<Provider>::alloc(&data, NCRamAndFileConfig(Some(NC_DATA_SIZE)));
        assert!(ncm.is_ok());
        let ncm = ncm.unwrap();

        if let RamShardedType(buf) = &ncm.shard1 {
            assert_ne!(&*buf.borrow(), &data);
        }
        if let FileShard(fm) = &ncm.shard2 {
            let buf = fm.unlock(FileConfig(None)).unwrap();
            assert_ne!(&*buf.borrow(), &data);
        }
    }

    #[test]
    fn test_zeroize() {
        // Check alloc
        let data = Provider::random_vec(NC_DATA_SIZE).unwrap();
        let ncm = NonContiguousMemory::<Provider>::alloc(&data, NCRamAndFileConfig(Some(NC_DATA_SIZE)));
        assert!(ncm.is_ok());
        let mut ncm = ncm.unwrap();
        ncm.zeroize();

        if let RamShardedType(buf) = &ncm.shard1 {
            assert_eq!(*buf.borrow(), []);
        }

        if let FileShard(fm) = &ncm.shard2 {
            let buf = fm.unlock(FileConfig(None));
            // We can't unlock a zeroized filememory
            assert!(buf.is_err());
        }
    }

    #[test]
    // Check that file content cannot be read directly
    fn test_security() {}
}
