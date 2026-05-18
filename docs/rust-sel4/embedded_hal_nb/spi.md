**embedded_hal_nb > spi**

# Module: spi

## Contents

**Traits**

- [`FullDuplex`](#fullduplex) - Full duplex SPI (master mode).

---

## embedded_hal_nb::spi::FullDuplex

*Trait*

Full duplex SPI (master mode).

# Notes

- It's the task of the user of this interface to manage the slave select lines.

- Due to how full duplex SPI works each `read` call must be preceded by a `write` call.

- `read` calls only return the data received with the last `write` call.
Previously received data is discarded

- Data is only guaranteed to be clocked out when the `read` call succeeds.
The slave select line shouldn't be released before that.

- Some SPIs can work with 8-bit *and* 16-bit words. You can overload this trait with different
`Word` types to allow operation in both modes.

**Methods:**

- `read`: Reads the word stored in the shift register
- `write`: Writes a word to the slave



