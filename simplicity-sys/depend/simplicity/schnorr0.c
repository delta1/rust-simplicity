#include "schnorr0.h"

/* A length-prefixed encoding of the following Simplicity program:
 *     (scribe (toWord256 0xF9308A019258C31049344F85F89D5229B531C845836F99B08601F113BCE036F9) &&&
 *      zero word256) &&&
 *      witness (toWord512 0xE907831F80848D1069A5371B402410364BDF1C5F8307B0084C55F1CE2DCA821525F66A4A85EA8B71E482A74F382D2CE5EBEEE8FDB2172F477DF4900D310536C0) >>>
 *     Simplicity.Programs.LibSecp256k1.Lib.bip_0340_verify
 * with jets.
 */
const unsigned char schnorr0[] = {
  0xe4, 0xf2, 0x4a, 0x10, 0x20, 0x98, 0x0b, 0x00, 0xa6, 0x22, 0x81, 0x62, 0x16, 0x38, 0x89, 0xa0, 0x52, 0x0a, 0x05, 0x88,
  0x5a, 0x71, 0x16, 0xad, 0x42, 0x81, 0x68, 0xd6, 0x2c, 0xc2, 0xa1, 0x66, 0x16, 0xce, 0x00, 0x2d, 0xe1, 0x70, 0x1e, 0x02,
  0x26, 0xa1, 0x50, 0xb8, 0x47, 0x01, 0x13, 0x80, 0x8b, 0x5f, 0x01, 0x14, 0x0b, 0x10, 0xb5, 0x85, 0xc2, 0xb8, 0x38, 0x9c,
  0x28, 0x5c, 0x22, 0x0a, 0x05, 0x87, 0x10, 0x8b, 0x83, 0xee, 0x14, 0x0b, 0x00, 0xb8, 0x97, 0x85, 0x0b, 0x89, 0xf8, 0x8c,
  0x5c, 0x46, 0x15, 0x0b, 0x82, 0xf0, 0x71, 0x70, 0x8e, 0x26, 0x14, 0x0b, 0x00, 0xb4, 0x85, 0xc0, 0x82, 0xe2, 0xde, 0x30,
  0x13, 0x86, 0x0b, 0x8a, 0xb8, 0x8c, 0x50, 0x2e, 0x19, 0xc5, 0x02, 0xe0, 0xfc, 0x40, 0x28, 0x16, 0x01, 0x71, 0x67, 0x18,
  0x0b, 0x8d, 0x78, 0xd8, 0x4e, 0x39, 0x15, 0x0b, 0x8d, 0xf8, 0xdc, 0x5b, 0x78, 0xd0, 0x50, 0x2c, 0x02, 0xd2, 0x17, 0x1a,
  0xe8, 0x13, 0x8c, 0x85, 0xc8, 0x2e, 0x34, 0x17, 0x1a, 0xf1, 0xe0, 0xa0, 0x58, 0x05, 0xc1, 0xf8, 0xbc, 0x5c, 0x88, 0xe4,
  0x28, 0x9c, 0x80, 0x15, 0x0b, 0x90, 0x7c, 0x08, 0x4e, 0x43, 0x0a, 0x85, 0xa0, 0x2d, 0xe1, 0x71, 0x08, 0x5c, 0x88, 0xe4,
  0x40, 0x81, 0x02, 0x04, 0x08, 0x16, 0x67, 0x14, 0x6c, 0x40, 0x8f, 0x48, 0x03, 0xa4, 0x1e, 0x0c, 0x7e, 0x02, 0x12, 0x34,
  0x41, 0xa6, 0x94, 0xdc, 0x6d, 0x00, 0x90, 0x40, 0xd9, 0x2f, 0x7c, 0x71, 0x7e, 0x0c, 0x1e, 0xc0, 0x21, 0x31, 0x57, 0xc7,
  0x38, 0xb7, 0x2a, 0x08, 0x54, 0x97, 0xd9, 0xa9, 0x2a, 0x17, 0xaa, 0x2d, 0xc7, 0x92, 0x0a, 0x9d, 0x3c, 0xe0, 0xb4, 0xb3,
  0x97, 0xaf, 0xbb, 0xa3, 0xf6, 0xc8, 0x5c, 0xbd, 0x1d, 0xf7, 0xd2, 0x40, 0x34, 0xc4, 0x14, 0xdb, 0x00
};

const size_t sizeof_schnorr0 = sizeof(schnorr0);

/* The commitment Merkle root of the above schnorr0 Simplicity expression. */
const uint32_t schnorr0_cmr[] = {
  0x0088471du, 0x32e0e37au, 0xf84ad6aau, 0xa5a0406au, 0x2a9a1e83u, 0xfb838b6fu, 0xf1d47879u, 0xa6a8b15du
};

/* The identity Merkle root of the above schnorr0 Simplicity expression. */
const uint32_t schnorr0_imr[] = {
  0x95ade785u, 0x7551b78fu, 0xc2b500f4u, 0x0b1aa8ccu, 0x5601c6c2u, 0xe07a8598u, 0x6a67ef0eu, 0xeb515e41u
};

/* The annotated Merkle root of the above schnorr0 Simplicity expression. */
const uint32_t schnorr0_amr[] = {
  0x9a669478u, 0x67fa8659u, 0x68d2b3acu, 0x015fe474u, 0xf2d53ba9u, 0x0a7c7bc4u, 0x26f4f31au, 0x81404e70u
};