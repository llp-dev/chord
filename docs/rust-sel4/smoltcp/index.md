# smoltcp

The _smoltcp_ library is built in a layered structure, with the layers corresponding
to the levels of API abstraction. Only the highest layers would be used by a typical
application; however, the goal of _smoltcp_ is not just to provide a simple interface
for writing applications but also to be a toolbox of networking primitives, so
every layer is fully exposed and documented.

When discussing networking stacks and layering, often the [OSI model][osi] is invoked.
_smoltcp_ makes no effort to conform to the OSI model as it is not applicable to TCP/IP.

# The socket layer
The socket layer APIs are provided in the module [socket](socket/index.html); currently,
raw, ICMP, TCP, and UDP sockets are provided. The socket API provides the usual primitives,
but necessarily differs in many from the [Berkeley socket API][berk], as the latter was
not designed to be used without heap allocation.

The socket layer provides the buffering, packet construction and validation, and (for
stateful sockets) the state machines, but it is interface-agnostic. An application must
use sockets together with a network interface.

# The interface layer
The interface layer APIs are provided in the module [iface](iface/index.html); currently,
Ethernet interface is provided.

The interface layer handles the control messages, physical addressing and neighbor discovery.
It routes packets to and from sockets.

# The physical layer
The physical layer APIs are provided in the module [phy](phy/index.html); currently,
raw socket and TAP interface are provided. In addition, two _middleware_ interfaces
are provided: the _tracer device_, which prints a human-readable representation of packets,
and the _fault injector device_, which randomly introduces errors into the transmitted
and received packet sequences.

The physical layer handles interaction with a platform-specific network device.

# The wire layers
Unlike the higher layers, the wire layer APIs will not be used by a typical application.
They however are the bedrock of _smoltcp_, and everything else is built on top of them.

The wire layer APIs are designed by the principle "make illegal states ir-representable".
If a wire layer object can be constructed, then it can also be parsed from or emitted to
a lower level.

The wire layer APIs also provide _tcpdump_-like pretty printing.

## The representation layer
The representation layer APIs are provided in the module [wire].

The representation layer exists to reduce the state space of raw packets. Raw packets
may be nonsensical in a multitude of ways: invalid checksums, impossible combinations of flags,
pointers to fields out of bounds, meaningless options... Representations shed all that,
as well as any features not supported by _smoltcp_.

## The packet layer
The packet layer APIs are also provided in the module [wire].

The packet layer exists to provide a more structured way to work with packets than
treating them as sequences of octets. It makes no judgement as to content of the packets,
except where necessary to provide safe access to fields, and strives to implement every
feature ever defined, to ensure that, when the representation layer is unable to make sense
of a packet, it is still logged correctly and in full.

# Minimum Supported Rust Version (MSRV)

This crate is guaranteed to compile on stable Rust 1.91 and up with any valid set of features.
It *might* compile on older versions but that may change in any new patch release.

The exception is when using the `defmt` feature, in which case `defmt`'s MSRV applies, which
is higher.

[wire]: wire/index.html
[osi]: https://en.wikipedia.org/wiki/OSI_model
[berk]: https://en.wikipedia.org/wiki/Berkeley_sockets

## Modules

### [`smoltcp`](smoltcp.md)

*7 modules*

### [`config`](config.md)

*16 constants*

### [`iface::interface`](iface/interface.md)

*2 enums, 3 structs*

### [`iface::route`](iface/route.md)

*3 structs*

### [`iface::socket_set`](iface/socket_set.md)

*3 structs*

### [`phy`](phy.md)

*2 enums, 3 structs, 3 traits*

### [`phy::fault_injector`](phy/fault_injector.md)

*1 struct*

### [`phy::fuzz_injector`](phy/fuzz_injector.md)

*1 struct, 1 trait*

### [`phy::loopback`](phy/loopback.md)

*1 struct*

### [`phy::pcap_writer`](phy/pcap_writer.md)

*1 struct, 1 trait, 2 enums*

### [`phy::tracer`](phy/tracer.md)

*1 enum, 2 structs*

### [`socket`](socket.md)

*1 enum, 1 trait, 3 modules*

### [`socket::dhcpv4`](socket/dhcpv4.md)

*1 enum, 4 structs*

### [`socket::dns`](socket/dns.md)

*3 enums, 3 structs*

### [`socket::tcp`](socket/tcp.md)

*1 struct, 1 type alias, 6 enums*

### [`storage`](storage.md)

*1 trait, 2 structs*

### [`storage::assembler`](storage/assembler.md)

*3 structs*

### [`storage::packet_buffer`](storage/packet_buffer.md)

*2 structs*

### [`storage::ring_buffer`](storage/ring_buffer.md)

*1 struct*

### [`time`](time.md)

*2 structs*

### [`wire`](wire.md)

*1 constant, 1 enum, 1 module, 1 type alias, 2 structs*

### [`wire::arp`](wire/arp.md)

*1 struct, 3 enums*

### [`wire::dhcpv4`](wire/dhcpv4.md)

*2 enums, 3 constants, 5 structs*

### [`wire::dns`](wire/dns.md)

*4 enums, 5 structs*

### [`wire::ethernet`](wire/ethernet.md)

*1 constant, 1 enum, 3 structs*

### [`wire::icmp`](wire/icmp.md)

*1 enum*

### [`wire::icmpv4`](wire/icmpv4.md)

*1 struct, 6 enums*

### [`wire::igmp`](wire/igmp.md)

*1 struct, 3 enums*

### [`wire::ip`](wire/ip.md)

*3 structs, 5 enums*

### [`wire::ipv4`](wire/ipv4.md)

*4 constants, 4 structs*

### [`wire::pretty_print`](wire/pretty_print.md)

*1 trait, 2 structs*

### [`wire::tcp`](wire/tcp.md)

*1 constant, 1 type alias, 2 enums, 4 structs*

### [`wire::udp`](wire/udp.md)

*1 constant, 2 structs*

