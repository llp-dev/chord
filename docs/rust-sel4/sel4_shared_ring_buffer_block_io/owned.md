**sel4_shared_ring_buffer_block_io > owned**

# Module: owned

## Contents

**Structs**

- [`OwnedSharedRingBufferBlockIO`](#ownedsharedringbufferblockio)

**Enums**

- [`IssueRequestBuf`](#issuerequestbuf)
- [`PollRequestBuf`](#pollrequestbuf)

---

## sel4_shared_ring_buffer_block_io::owned::IssueRequestBuf

*Enum*

**Generic Parameters:**
- 'a

**Variants:**
- `Read{ len: usize }`
- `Write{ buf: &'a [u8] }`

**Methods:**

- `fn new<A>(operation: &'a Operation<'a, A>) -> Self`



## sel4_shared_ring_buffer_block_io::owned::OwnedSharedRingBufferBlockIO

*Struct*

**Generic Parameters:**
- S
- A
- F

**Methods:**

- `fn new(dma_region: SharedMemoryRef<'static, [u8]>, bounce_buffer_allocator: A, ring_buffers: RingBuffers<'static, Provide, F, BlockIORequest>) -> Self`
- `fn slot_set_semaphore(self: &Self) -> &SlotSetSemaphoreHandle<S, NUM_SLOT_POOLS>`
- `fn issue_read_request(self: & mut Self, reservation: & mut SlotSetReservation<S, NUM_SLOT_POOLS>, start_block_idx: u64, num_bytes: usize) -> Result<usize, ErrorOrUserError>`
- `fn issue_write_request(self: & mut Self, reservation: & mut SlotSetReservation<S, NUM_SLOT_POOLS>, start_block_idx: u64, buf: &[u8]) -> Result<usize, ErrorOrUserError>`
- `fn issue_request(self: & mut Self, reservation: & mut SlotSetReservation<S, NUM_SLOT_POOLS>, start_block_idx: u64, buf: & mut IssueRequestBuf) -> Result<usize, ErrorOrUserError>`
- `fn cancel_request(self: & mut Self, request_index: usize) -> Result<(), ErrorOrUserError>`
- `fn poll_read_request(self: & mut Self, request_index: usize, buf: & mut [u8], waker: Option<Waker>) -> Result<Poll<Result<(), IOError>>, ErrorOrUserError>`
- `fn poll_write_request(self: & mut Self, request_index: usize, waker: Option<Waker>) -> Result<Poll<Result<(), IOError>>, ErrorOrUserError>`
- `fn poll_request(self: & mut Self, request_index: usize, buf: & mut PollRequestBuf, waker: Option<Waker>) -> Result<Poll<Result<(), IOError>>, ErrorOrUserError>`
- `fn poll(self: & mut Self) -> Result<bool, ErrorOrUserError>`



## sel4_shared_ring_buffer_block_io::owned::PollRequestBuf

*Enum*

**Generic Parameters:**
- 'a

**Variants:**
- `Read{ buf: &'a  mut [u8] }`
- `Write`

**Methods:**

- `fn new<'b, A>(operation: &'a  mut Operation<'b, A>) -> Self`



