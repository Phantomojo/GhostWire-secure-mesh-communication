Crate utf8 Copy item path

<span class="sub-heading"><a href="../src/utf8/lib.rs.html#1-186" class="src">Source</a>
</span>

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.BufReadDecoder.html" class="struct"
title="struct utf8::BufReadDecoder">BufReadDecoder</a>  
Wraps a `std::io::BufRead` buffered byte stream and decode it as UTF-8.

<a href="struct.Incomplete.html" class="struct"
title="struct utf8::Incomplete">Incomplete</a>  
<a href="struct.LossyDecoder.html" class="struct"
title="struct utf8::LossyDecoder">LossyDecoder</a>  
A push-based, lossy decoder for UTF-8. Errors are replaced with the
U+FFFD replacement character.

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.BufReadDecoderError.html" class="enum"
title="enum utf8::BufReadDecoderError">BufReadDecoderError</a>

<a href="enum.DecodeError.html" class="enum"
title="enum utf8::DecodeError">DecodeError</a>

# Constants<a href="#constants" class="anchor">§</a>

<a href="constant.REPLACEMENT_CHARACTER.html" class="constant"
title="constant utf8::REPLACEMENT_CHARACTER">REPLACEMENT_CHARACTER</a>  
The replacement character, U+FFFD. In lossy decoding, insert it for
every decoding error.

# Functions<a href="#functions" class="anchor">§</a>

<a href="fn.decode.html" class="fn" title="fn utf8::decode">decode</a>
