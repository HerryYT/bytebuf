import test from 'ava'

import { ByteBuf } from '../index.js'

test('test byte buffer creation', (t) => {
  const buf = new ByteBuf();
  t.is(buf.getArray().length, 0);
})

test('test byte buffer capacity', (t) => {
  const buf = new ByteBuf();

  // Test writing with 0 capacity
  t.is(buf.getCapacity(), 0);
  t.is(buf.isWriteable(), false);
  t.is(buf.isWriteable(5), false);

  // Test writing with 5 bytes capacity
  buf.setCapacity(5);
  t.is(buf.getCapacity(), 5);
  t.is(buf.isWriteable(), true);
  t.is(buf.isWriteable(5), true);
  // Test exceeding buffer capacity
  t.is(buf.isWriteable(6), false);

  buf.setCapacity(3);
  t.is(buf.getCapacity(), 3);

  const buf2 = new ByteBuf(Buffer.allocUnsafe(4));
  t.is(buf2.getCapacity(), 4);
})

test('test byte buffer readability', (t) => {
  const buf = new ByteBuf(Buffer.alloc(5));
  t.is(buf.isReadable(), true);
  t.is(buf.isReadable(6), false);
  t.is(buf.isReadable(5), true);
})

test('test byte buffer set indexes', (t) => {
  const buf = new ByteBuf();
  buf.setCapacity(8);

  t.throws(() => buf.setReaderIndex(2), { code: 'InvalidArg' });

  const buf2 = new ByteBuf(Buffer.allocUnsafe(8));

  // read long
  buf2.setReaderIndex(8);
  t.throws(() => buf2.setWriterIndex(4), { code: 'InvalidArg' });

  // Will obviously overflow, use the proper method, thanks.
  // const buf3 = new ByteBuf();
  // buf3.setReaderIndex(-2);
  // console.log(buf3.getReaderIndex())

  buf2.setIndex(2, 4);
  t.is(buf2.getWriterIndex(), 4);
  t.is(buf2.getReaderIndex(), 2);

  t.throws(() => buf2.setIndex(4, 2), { code: 'InvalidArg' })
  t.throws(() => buf2.setIndex(7, 9), { code: 'InvalidArg' })
})

test('test read byte and boolean', (t) => {
  const buf = new ByteBuf(Buffer.from([0x7f]));
  t.is(buf.readByte(), 0x7f);

  // Test signed overflow with -128 -> -0x80
  const buf2 = new ByteBuf(Buffer.from([0x80]));
  t.is(buf2.readByte(), -0x80);

  // Test boolean, anything not 0 is true
  const buf3 = new ByteBuf(Buffer.from([0x7f]));
  t.is(buf3.readBoolean(), true);
})

// test('test read short', (t) => {
  // const buf = new ByteBuf(Buffer.from([0x7f]));
  // t.is(buf.readByte(), 0x7f);
// })

// test('test read triad', (t) => {
  // const buf = new ByteBuf(Buffer.from([0x80, 0x00, 0x00]));
  // const buf = new ByteBuf();
  // buf.setCapacity(3);
  // buf.writeMedium(-8388607);
  // console.log(buf.getArray())
  // const buf2 = new ByteBuf(Buffer.from(buf.getArray()))
  // console.log(buf2.readMedium());
  // t.is(buf.readMedium(), -8388608);
// })

test('test read with no bytes left', (t) => {
  const buf = new ByteBuf(Buffer.from([0x7f]));
  t.is(buf.readByte(), 0x7f);
  t.throws(() => buf.readByte(), { message: 'cannot readByte, readableBytes is less than 1' });
  t.throws(() => buf.readBoolean(), { message: 'cannot readBoolean, readableBytes is less than 1' });
})


