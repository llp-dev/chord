**embedded_fat > volume**

# Module: volume

## Contents

**Structs**

- [`Volume`](#volume) - A `VolumeManager` wraps a block device and gives access to the FAT-formatted

---

## embedded_fat::volume::Volume

*Struct*

A `VolumeManager` wraps a block device and gives access to the FAT-formatted
volumes within it.

**Generic Parameters:**
- D
- T
- const MAX_DIRS
- const MAX_FILES

**Methods:**

- `fn new(block_device: D, time_source: T) -> Result<Volume<D, T, 4, 4>, Error<<D as >::Error>>` - Create a new Volume Manager using a generic `BlockDevice`. From this
- `fn new_with_limits(block_device: D, time_source: T, id_offset: u32) -> Result<Volume<D, T, MAX_DIRS, MAX_FILES>, Error<<D as >::Error>>` - Create a new Volume Manager using a generic `BlockDevice`. From this
- `fn device(self: & mut Self) -> & mut D` - Temporarily get access to the underlying block device.
- `fn open_root_dir(self: & mut Self) -> Result<Directory, Error<<D as >::Error>>` - Open the volume's root directory.
- `fn open_dir<N>(self: & mut Self, parent_dir: Directory, name: N) -> Result<Directory, Error<<D as >::Error>>` - Open a directory.
- `fn close_dir(self: & mut Self, directory: Directory) -> Result<(), Error<<D as >::Error>>` - Close a directory. You cannot perform operations on an open directory
- `fn find_directory_entry<N>(self: & mut Self, directory: Directory, name: N) -> Result<DirEntry, Error<<D as >::Error>>` - Look in a directory for a named file.
- `fn find_lfn_directory_entry(self: & mut Self, directory: Directory, name: &str) -> Result<DirEntry, Error<<D as >::Error>>`
- `fn iterate_dir<F, U>(self: & mut Self, directory: Directory, func: F) -> Result<Option<U>, Error<<D as >::Error>>` - Call a callback function for each directory entry in a directory.
- `fn iterate_lfn_dir<F, U>(self: & mut Self, directory: Directory, func: F) -> Result<Option<U>, Error<<D as >::Error>>` - Call a callback function for each directory entry in a directory, with its LFN if it has one.
- `fn open_file_in_dir<N>(self: & mut Self, directory: Directory, name: N, mode: Mode) -> Result<File, Error<<D as >::Error>>` - Open a file with the given full path. A file can only be opened once.
- `fn delete_file_in_dir<N>(self: & mut Self, directory: Directory, name: N) -> Result<(), Error<<D as >::Error>>` - Delete a closed file with the given filename, if it exists.
- `fn read(self: & mut Self, file: File, buffer: & mut [u8]) -> Result<usize, Error<<D as >::Error>>` - Read from an open file.
- `fn write(self: & mut Self, file: File, buffer: &[u8]) -> Result<usize, Error<<D as >::Error>>` - Write to a open file.
- `fn close_file(self: & mut Self, file: File) -> Result<(), Error<<D as >::Error>>` - Close a file with the given full path.
- `fn has_open_handles(self: &Self) -> bool` - Check if any files or folders are open.
- `fn free(self: Self) -> (D, T)` - Consume self and return BlockDevice and TimeSource
- `fn file_eof(self: &Self, file: File) -> Result<bool, Error<<D as >::Error>>` - Check if a file is at End Of File.
- `fn file_seek_from_start(self: & mut Self, file: File, offset: u32) -> Result<(), Error<<D as >::Error>>` - Seek a file with an offset from the start of the file.
- `fn file_seek_from_current(self: & mut Self, file: File, offset: i32) -> Result<(), Error<<D as >::Error>>` - Seek a file with an offset from the current position.
- `fn file_seek_from_end(self: & mut Self, file: File, offset: u32) -> Result<(), Error<<D as >::Error>>` - Seek a file with an offset back from the end of the file.
- `fn file_length(self: &Self, file: File) -> Result<u32, Error<<D as >::Error>>` - Get the length of a file
- `fn file_offset(self: &Self, file: File) -> Result<u32, Error<<D as >::Error>>` - Get the current offset of a file



