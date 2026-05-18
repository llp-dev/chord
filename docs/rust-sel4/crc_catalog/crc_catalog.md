**crc_catalog**

# Module: crc_catalog

## Contents

**Modules**

- [`algorithm`](#algorithm) - CRC algorithms as structs.
- [`poly`](#poly) - CRC polynomials and their aliases.

**Structs**

- [`Algorithm`](#algorithm) - This struct describes a CRC algorithm using the fields specified by the [Catalogue of

**Traits**

- [`Width`](#width)

---

## crc_catalog::Algorithm

*Struct*

This struct describes a CRC algorithm using the fields specified by the [Catalogue of
parametrised CRC algorithms](https://reveng.sourceforge.io/crc-catalogue/all.htm).

**Generic Parameters:**
- W

**Fields:**
- `width: u8` - The number of bit cells in the linear feedback shift register; the degree of the generator
- `poly: W` - The generator polynomial that sets the feedback tap positions of the shift register. The
- `init: W` - The settings of the bit cells at the start of each calculation, before reading the first
- `refin: bool` - If equal to `false`, specifies that the characters of the message are read bit-by-bit, most
- `refout: bool` - If equal to `false`, specifies that the contents of the register after reading the last
- `xorout: W` - The XOR value applied to the contents of the register after the last message bit has been
- `check: W` - The contents of the register after initialising, reading the UTF-8 string `"123456789"` (as
- `residue: W` - The contents of the register after initialising, reading an error-free codeword and



## crc_catalog::Width

*Trait*



## Module: algorithm

CRC algorithms as structs.



## Module: poly

CRC polynomials and their aliases.

These polynomials are collected from the following catalogues:
- [Wikipedia](https://wikipedia.org/wiki/Cyclic_redundancy_check#Polynomial_representations_of_cyclic_redundancy_checks)
- [Catalogue of parametrised CRC algorithms](https://reveng.sourceforge.io/crc-catalogue/all.htm)
- [CRC Polynomial Zoo](https://users.ece.cmu.edu/~koopman/crc/crc32.html)



