Crate yasna Copy item path

<span class="sub-heading"><a href="../src/yasna/lib.rs.html#9-171" class="src">Source</a>
</span>

Expand description

A library for reading and writing ASN.1 data.

# <a href="#examples" class="doc-anchor">§</a>Examples

## <a href="#encodingdecoding-simple-data" class="doc-anchor">§</a>Encoding/decoding simple data

A type implementing
[`DEREncodable`](trait.DEREncodable.html "trait yasna::DEREncodable")
can be easily encoded:

``` {.rust .rust-example-rendered}
fn main() {
    let der = yasna::encode_der(&(10, true));
    println!("(10, true) = {:?}", der);
}
```

Similarly, a type implementing
[`BERDecodable`](trait.BERDecodable.html "trait yasna::BERDecodable")
can be easily decoded:

``` {.rust .rust-example-rendered}
fn main() {
    let asn: (i64, bool) = yasna::decode_der(
        &[48, 6, 2, 1, 10, 1, 1, 255]).unwrap();
    println!("{:?} = [48, 6, 2, 1, 10, 1, 1, 255]", asn);
}
```

## <a href="#encodingdecoding-by-hand" class="doc-anchor">§</a>Encoding/decoding by hand

The default
[`DEREncodable`](trait.DEREncodable.html "trait yasna::DEREncodable")/[`BERDecodable`](trait.BERDecodable.html "trait yasna::BERDecodable")
implementations can’t handle all ASN.1 types. In many cases you have to
write your reader/writer by hand.

To serialize ASN.1 data, you can use
[`construct_der`](fn.construct_der.html "fn yasna::construct_der").

``` {.rust .rust-example-rendered}
fn main() {
    let der = yasna::construct_der(|writer| {
        writer.write_sequence(|writer| {
            writer.next().write_i64(10);
            writer.next().write_bool(true);
        })
    });
    println!("(10, true) = {:?}", der);
}
```

To deserialize ASN.1 data, you can use
[`parse_ber`](fn.parse_ber.html "fn yasna::parse_ber") or
[`parse_der`](fn.parse_der.html "fn yasna::parse_der").

``` {.rust .rust-example-rendered}
fn main() {
    let asn = yasna::parse_der(&[48, 6, 2, 1, 10, 1, 1, 255], |reader| {
        reader.read_sequence(|reader| {
            let i = reader.next().read_i64()?;
            let b = reader.next().read_bool()?;
            return Ok((i, b));
        })
    }).unwrap();
    println!("{:?} = [48, 6, 2, 1, 10, 1, 1, 255]", asn);
}
```

# Modules<a href="#modules" class="anchor">§</a>

<a href="models/index.html" class="mod"
title="mod yasna::models">models</a>  
Provides datatypes which correspond to ASN.1 types.

<a href="tags/index.html" class="mod" title="mod yasna::tags">tags</a>  
Provides universal tag constants.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.ASN1Error.html" class="struct"
title="struct yasna::ASN1Error">ASN1Error</a>  
<a href="struct.BERReader.html" class="struct"
title="struct yasna::BERReader">BERReader</a>  
A reader object for BER/DER-encoded ASN.1 data.

<a href="struct.BERReaderSeq.html" class="struct"
title="struct yasna::BERReaderSeq">BERReaderSeq</a>  
A reader object for a sequence of BER/DER-encoded ASN.1 data.

<a href="struct.BERReaderSet.html" class="struct"
title="struct yasna::BERReaderSet">BERReaderSet</a>  
A reader object for a set of BER/DER-encoded ASN.1 data.

<a href="struct.DERWriter.html" class="struct"
title="struct yasna::DERWriter">DERWriter</a>  
A writer object that accepts an ASN.1 value.

<a href="struct.DERWriterSeq.html" class="struct"
title="struct yasna::DERWriterSeq">DERWriterSeq</a>  
A writer object that accepts ASN.1 values.

<a href="struct.DERWriterSet.html" class="struct"
title="struct yasna::DERWriterSet">DERWriterSet</a>  
A writer object that accepts ASN.1 values.

<a href="struct.Tag.html" class="struct"
title="struct yasna::Tag">Tag</a>  
An ASN.1 tag.

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.ASN1ErrorKind.html" class="enum"
title="enum yasna::ASN1ErrorKind">ASN1ErrorKind</a>  
<a href="enum.BERMode.html" class="enum"
title="enum yasna::BERMode">BERMode</a>  
Used by [`BERReader`](struct.BERReader.html "struct yasna::BERReader")
to determine whether or not to enforce DER restrictions when parsing.

<a href="enum.PCBit.html" class="enum"
title="enum yasna::PCBit">PCBit</a>  
A value of the ASN.1 primitive/constructed (“P/C”) bit.

<a href="enum.TagClass.html" class="enum"
title="enum yasna::TagClass">TagClass</a>  
An ASN.1 tag class, used in
[`Tag`](struct.Tag.html "struct yasna::Tag").

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.BERDecodable.html" class="trait"
title="trait yasna::BERDecodable">BERDecodable</a>  
Types decodable in BER.

<a href="trait.DEREncodable.html" class="trait"
title="trait yasna::DEREncodable">DEREncodable</a>  
Types encodable in DER.

# Functions<a href="#functions" class="anchor">§</a>

<a href="fn.construct_der.html" class="fn"
title="fn yasna::construct_der">construct_der</a>  
Constructs DER-encoded data as `Vec<u8>`.

<a href="fn.construct_der_seq.html" class="fn"
title="fn yasna::construct_der_seq">construct_der_seq</a>  
Constructs DER-encoded sequence of data as `Vec<u8>`.

<a href="fn.decode_ber.html" class="fn"
title="fn yasna::decode_ber">decode_ber</a>  
Reads an ASN.1 value from `&[u8]`.

<a href="fn.decode_ber_general.html" class="fn"
title="fn yasna::decode_ber_general">decode_ber_general</a>  
Decodes DER/BER-encoded data.

<a href="fn.decode_der.html" class="fn"
title="fn yasna::decode_der">decode_der</a>  
Reads an ASN.1 value from `&[u8]`.

<a href="fn.encode_der.html" class="fn"
title="fn yasna::encode_der">encode_der</a>  
Encodes a value to DER-encoded ASN.1 data.

<a href="fn.parse_ber.html" class="fn"
title="fn yasna::parse_ber">parse_ber</a>  
Parses BER-encoded data.

<a href="fn.parse_ber_general.html" class="fn"
title="fn yasna::parse_ber_general">parse_ber_general</a>  
Parses DER/BER-encoded data.

<a href="fn.parse_der.html" class="fn"
title="fn yasna::parse_der">parse_der</a>  
Parses DER-encoded data.

<a href="fn.try_construct_der.html" class="fn"
title="fn yasna::try_construct_der">try_construct_der</a>  
Tries to construct DER-encoded data as `Vec<u8>`.

<a href="fn.try_construct_der_seq.html" class="fn"
title="fn yasna::try_construct_der_seq">try_construct_der_seq</a>  
Tries to construct a DER-encoded sequence of data as `Vec<u8>`.

# Type Aliases<a href="#types" class="anchor">§</a>

<a href="type.ASN1Result.html" class="type"
title="type yasna::ASN1Result">ASN1Result</a>
