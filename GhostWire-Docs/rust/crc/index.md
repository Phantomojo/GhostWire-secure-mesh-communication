Crate crc Copy item path

<span class="sub-heading"><a href="../src/crc/lib.rs.html#1-93" class="src">Source</a>
</span>

Expand description

# <a href="#crc" class="doc-anchor">§</a>crc

Rust implementation of CRC.

### <a href="#examples" class="doc-anchor">§</a>Examples

Using a well-known algorithm:

``` {.rust .rust-example-rendered}
const X25: crc::Crc<u16> = crc::Crc::<u16>::new(&crc::CRC_16_IBM_SDLC);
assert_eq!(X25.checksum(b"123456789"), 0x906e);
```

Using a custom algorithm:

``` {.rust .rust-example-rendered}
const CUSTOM_ALG: crc::Algorithm<u16> = crc::Algorithm {
    width: 16,
    poly: 0x8005,
    init: 0xffff,
    refin: false,
    refout: false,
    xorout: 0x0000,
    check: 0xaee7,
    residue: 0x0000
};
let crc = crc::Crc::<u16>::new(&CUSTOM_ALG);
let mut digest = crc.digest();
digest.update(b"123456789");
assert_eq!(digest.finalize(), 0xaee7);
```

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Algorithm.html" class="struct"
title="struct crc::Algorithm">Algorithm</a>  
This struct describes a CRC algorithm using the fields specified by the
[Catalogue of parametrised CRC
algorithms](https://reveng.sourceforge.io/crc-catalogue/all.htm).

<a href="struct.Crc.html" class="struct" title="struct crc::Crc">Crc</a>  
Crc instance with a specific width, algorithm, and implementation.

<a href="struct.Digest.html" class="struct"
title="struct crc::Digest">Digest</a>  
<a href="struct.Table.html" class="struct"
title="struct crc::Table">Table</a>  
A table-based implementation of the CRC algorithm, with `L` lanes. The
number of entries in the lookup table is `L * 256`.

# Constants<a href="#constants" class="anchor">§</a>

<a href="constant.CRC_3_GSM.html" class="constant"
title="constant crc::CRC_3_GSM">CRC_3_GSM</a>

<a href="constant.CRC_3_ROHC.html" class="constant"
title="constant crc::CRC_3_ROHC">CRC_3_ROHC</a>

<a href="constant.CRC_4_G_704.html" class="constant"
title="constant crc::CRC_4_G_704">CRC_4_G_704</a>

<a href="constant.CRC_4_INTERLAKEN.html" class="constant"
title="constant crc::CRC_4_INTERLAKEN">CRC_4_INTERLAKEN</a>

<a href="constant.CRC_5_EPC_C1G2.html" class="constant"
title="constant crc::CRC_5_EPC_C1G2">CRC_5_EPC_C1G2</a>

<a href="constant.CRC_5_G_704.html" class="constant"
title="constant crc::CRC_5_G_704">CRC_5_G_704</a>

<a href="constant.CRC_5_USB.html" class="constant"
title="constant crc::CRC_5_USB">CRC_5_USB</a>

<a href="constant.CRC_6_CDMA2000_A.html" class="constant"
title="constant crc::CRC_6_CDMA2000_A">CRC_6_CDMA2000_A</a>

<a href="constant.CRC_6_CDMA2000_B.html" class="constant"
title="constant crc::CRC_6_CDMA2000_B">CRC_6_CDMA2000_B</a>

<a href="constant.CRC_6_DARC.html" class="constant"
title="constant crc::CRC_6_DARC">CRC_6_DARC</a>

<a href="constant.CRC_6_GSM.html" class="constant"
title="constant crc::CRC_6_GSM">CRC_6_GSM</a>

<a href="constant.CRC_6_G_704.html" class="constant"
title="constant crc::CRC_6_G_704">CRC_6_G_704</a>

<a href="constant.CRC_7_MMC.html" class="constant"
title="constant crc::CRC_7_MMC">CRC_7_MMC</a>

<a href="constant.CRC_7_ROHC.html" class="constant"
title="constant crc::CRC_7_ROHC">CRC_7_ROHC</a>

<a href="constant.CRC_7_UMTS.html" class="constant"
title="constant crc::CRC_7_UMTS">CRC_7_UMTS</a>

<a href="constant.CRC_8_AUTOSAR.html" class="constant"
title="constant crc::CRC_8_AUTOSAR">CRC_8_AUTOSAR</a>

<a href="constant.CRC_8_BLUETOOTH.html" class="constant"
title="constant crc::CRC_8_BLUETOOTH">CRC_8_BLUETOOTH</a>

<a href="constant.CRC_8_CDMA2000.html" class="constant"
title="constant crc::CRC_8_CDMA2000">CRC_8_CDMA2000</a>

<a href="constant.CRC_8_DARC.html" class="constant"
title="constant crc::CRC_8_DARC">CRC_8_DARC</a>

<a href="constant.CRC_8_DVB_S2.html" class="constant"
title="constant crc::CRC_8_DVB_S2">CRC_8_DVB_S2</a>

<a href="constant.CRC_8_GSM_A.html" class="constant"
title="constant crc::CRC_8_GSM_A">CRC_8_GSM_A</a>

<a href="constant.CRC_8_GSM_B.html" class="constant"
title="constant crc::CRC_8_GSM_B">CRC_8_GSM_B</a>

<a href="constant.CRC_8_HITAG.html" class="constant"
title="constant crc::CRC_8_HITAG">CRC_8_HITAG</a>

<a href="constant.CRC_8_I_432_1.html" class="constant"
title="constant crc::CRC_8_I_432_1">CRC_8_I_432_1</a>

<a href="constant.CRC_8_I_CODE.html" class="constant"
title="constant crc::CRC_8_I_CODE">CRC_8_I_CODE</a>

<a href="constant.CRC_8_LTE.html" class="constant"
title="constant crc::CRC_8_LTE">CRC_8_LTE</a>

<a href="constant.CRC_8_MAXIM_DOW.html" class="constant"
title="constant crc::CRC_8_MAXIM_DOW">CRC_8_MAXIM_DOW</a>

<a href="constant.CRC_8_MIFARE_MAD.html" class="constant"
title="constant crc::CRC_8_MIFARE_MAD">CRC_8_MIFARE_MAD</a>

<a href="constant.CRC_8_NRSC_5.html" class="constant"
title="constant crc::CRC_8_NRSC_5">CRC_8_NRSC_5</a>

<a href="constant.CRC_8_OPENSAFETY.html" class="constant"
title="constant crc::CRC_8_OPENSAFETY">CRC_8_OPENSAFETY</a>

<a href="constant.CRC_8_ROHC.html" class="constant"
title="constant crc::CRC_8_ROHC">CRC_8_ROHC</a>

<a href="constant.CRC_8_SAE_J1850.html" class="constant"
title="constant crc::CRC_8_SAE_J1850">CRC_8_SAE_J1850</a>

<a href="constant.CRC_8_SMBUS.html" class="constant"
title="constant crc::CRC_8_SMBUS">CRC_8_SMBUS</a>

<a href="constant.CRC_8_TECH_3250.html" class="constant"
title="constant crc::CRC_8_TECH_3250">CRC_8_TECH_3250</a>

<a href="constant.CRC_8_WCDMA.html" class="constant"
title="constant crc::CRC_8_WCDMA">CRC_8_WCDMA</a>

<a href="constant.CRC_10_ATM.html" class="constant"
title="constant crc::CRC_10_ATM">CRC_10_ATM</a>

<a href="constant.CRC_10_CDMA2000.html" class="constant"
title="constant crc::CRC_10_CDMA2000">CRC_10_CDMA2000</a>

<a href="constant.CRC_10_GSM.html" class="constant"
title="constant crc::CRC_10_GSM">CRC_10_GSM</a>

<a href="constant.CRC_11_FLEXRAY.html" class="constant"
title="constant crc::CRC_11_FLEXRAY">CRC_11_FLEXRAY</a>

<a href="constant.CRC_11_UMTS.html" class="constant"
title="constant crc::CRC_11_UMTS">CRC_11_UMTS</a>

<a href="constant.CRC_12_CDMA2000.html" class="constant"
title="constant crc::CRC_12_CDMA2000">CRC_12_CDMA2000</a>

<a href="constant.CRC_12_DECT.html" class="constant"
title="constant crc::CRC_12_DECT">CRC_12_DECT</a>

<a href="constant.CRC_12_GSM.html" class="constant"
title="constant crc::CRC_12_GSM">CRC_12_GSM</a>

<a href="constant.CRC_12_UMTS.html" class="constant"
title="constant crc::CRC_12_UMTS">CRC_12_UMTS</a>

<a href="constant.CRC_13_BBC.html" class="constant"
title="constant crc::CRC_13_BBC">CRC_13_BBC</a>

<a href="constant.CRC_14_DARC.html" class="constant"
title="constant crc::CRC_14_DARC">CRC_14_DARC</a>

<a href="constant.CRC_14_GSM.html" class="constant"
title="constant crc::CRC_14_GSM">CRC_14_GSM</a>

<a href="constant.CRC_15_CAN.html" class="constant"
title="constant crc::CRC_15_CAN">CRC_15_CAN</a>

<a href="constant.CRC_15_MPT1327.html" class="constant"
title="constant crc::CRC_15_MPT1327">CRC_15_MPT1327</a>

<a href="constant.CRC_16_ARC.html" class="constant"
title="constant crc::CRC_16_ARC">CRC_16_ARC</a>

<a href="constant.CRC_16_CDMA2000.html" class="constant"
title="constant crc::CRC_16_CDMA2000">CRC_16_CDMA2000</a>

<a href="constant.CRC_16_CMS.html" class="constant"
title="constant crc::CRC_16_CMS">CRC_16_CMS</a>

<a href="constant.CRC_16_DDS_110.html" class="constant"
title="constant crc::CRC_16_DDS_110">CRC_16_DDS_110</a>

<a href="constant.CRC_16_DECT_R.html" class="constant"
title="constant crc::CRC_16_DECT_R">CRC_16_DECT_R</a>

<a href="constant.CRC_16_DECT_X.html" class="constant"
title="constant crc::CRC_16_DECT_X">CRC_16_DECT_X</a>

<a href="constant.CRC_16_DNP.html" class="constant"
title="constant crc::CRC_16_DNP">CRC_16_DNP</a>

<a href="constant.CRC_16_EN_13757.html" class="constant"
title="constant crc::CRC_16_EN_13757">CRC_16_EN_13757</a>

<a href="constant.CRC_16_GENIBUS.html" class="constant"
title="constant crc::CRC_16_GENIBUS">CRC_16_GENIBUS</a>

<a href="constant.CRC_16_GSM.html" class="constant"
title="constant crc::CRC_16_GSM">CRC_16_GSM</a>

<a href="constant.CRC_16_IBM_3740.html" class="constant"
title="constant crc::CRC_16_IBM_3740">CRC_16_IBM_3740</a>

<a href="constant.CRC_16_IBM_SDLC.html" class="constant"
title="constant crc::CRC_16_IBM_SDLC">CRC_16_IBM_SDLC</a>

<a href="constant.CRC_16_ISO_IEC_14443_3_A.html" class="constant"
title="constant crc::CRC_16_ISO_IEC_14443_3_A">CRC_16_ISO_IEC_14443_3_A</a>

<a href="constant.CRC_16_KERMIT.html" class="constant"
title="constant crc::CRC_16_KERMIT">CRC_16_KERMIT</a>

<a href="constant.CRC_16_LJ1200.html" class="constant"
title="constant crc::CRC_16_LJ1200">CRC_16_LJ1200</a>

<a href="constant.CRC_16_M17.html" class="constant"
title="constant crc::CRC_16_M17">CRC_16_M17</a>

<a href="constant.CRC_16_MAXIM_DOW.html" class="constant"
title="constant crc::CRC_16_MAXIM_DOW">CRC_16_MAXIM_DOW</a>

<a href="constant.CRC_16_MCRF4XX.html" class="constant"
title="constant crc::CRC_16_MCRF4XX">CRC_16_MCRF4XX</a>

<a href="constant.CRC_16_MODBUS.html" class="constant"
title="constant crc::CRC_16_MODBUS">CRC_16_MODBUS</a>

<a href="constant.CRC_16_NRSC_5.html" class="constant"
title="constant crc::CRC_16_NRSC_5">CRC_16_NRSC_5</a>

<a href="constant.CRC_16_OPENSAFETY_A.html" class="constant"
title="constant crc::CRC_16_OPENSAFETY_A">CRC_16_OPENSAFETY_A</a>

<a href="constant.CRC_16_OPENSAFETY_B.html" class="constant"
title="constant crc::CRC_16_OPENSAFETY_B">CRC_16_OPENSAFETY_B</a>

<a href="constant.CRC_16_PROFIBUS.html" class="constant"
title="constant crc::CRC_16_PROFIBUS">CRC_16_PROFIBUS</a>

<a href="constant.CRC_16_RIELLO.html" class="constant"
title="constant crc::CRC_16_RIELLO">CRC_16_RIELLO</a>

<a href="constant.CRC_16_SPI_FUJITSU.html" class="constant"
title="constant crc::CRC_16_SPI_FUJITSU">CRC_16_SPI_FUJITSU</a>

<a href="constant.CRC_16_T10_DIF.html" class="constant"
title="constant crc::CRC_16_T10_DIF">CRC_16_T10_DIF</a>

<a href="constant.CRC_16_TELEDISK.html" class="constant"
title="constant crc::CRC_16_TELEDISK">CRC_16_TELEDISK</a>

<a href="constant.CRC_16_TMS37157.html" class="constant"
title="constant crc::CRC_16_TMS37157">CRC_16_TMS37157</a>

<a href="constant.CRC_16_UMTS.html" class="constant"
title="constant crc::CRC_16_UMTS">CRC_16_UMTS</a>

<a href="constant.CRC_16_USB.html" class="constant"
title="constant crc::CRC_16_USB">CRC_16_USB</a>

<a href="constant.CRC_16_XMODEM.html" class="constant"
title="constant crc::CRC_16_XMODEM">CRC_16_XMODEM</a>

<a href="constant.CRC_17_CAN_FD.html" class="constant"
title="constant crc::CRC_17_CAN_FD">CRC_17_CAN_FD</a>

<a href="constant.CRC_21_CAN_FD.html" class="constant"
title="constant crc::CRC_21_CAN_FD">CRC_21_CAN_FD</a>

<a href="constant.CRC_24_BLE.html" class="constant"
title="constant crc::CRC_24_BLE">CRC_24_BLE</a>

<a href="constant.CRC_24_FLEXRAY_A.html" class="constant"
title="constant crc::CRC_24_FLEXRAY_A">CRC_24_FLEXRAY_A</a>

<a href="constant.CRC_24_FLEXRAY_B.html" class="constant"
title="constant crc::CRC_24_FLEXRAY_B">CRC_24_FLEXRAY_B</a>

<a href="constant.CRC_24_INTERLAKEN.html" class="constant"
title="constant crc::CRC_24_INTERLAKEN">CRC_24_INTERLAKEN</a>

<a href="constant.CRC_24_LTE_A.html" class="constant"
title="constant crc::CRC_24_LTE_A">CRC_24_LTE_A</a>

<a href="constant.CRC_24_LTE_B.html" class="constant"
title="constant crc::CRC_24_LTE_B">CRC_24_LTE_B</a>

<a href="constant.CRC_24_OPENPGP.html" class="constant"
title="constant crc::CRC_24_OPENPGP">CRC_24_OPENPGP</a>

<a href="constant.CRC_24_OS_9.html" class="constant"
title="constant crc::CRC_24_OS_9">CRC_24_OS_9</a>

<a href="constant.CRC_30_CDMA.html" class="constant"
title="constant crc::CRC_30_CDMA">CRC_30_CDMA</a>

<a href="constant.CRC_31_PHILIPS.html" class="constant"
title="constant crc::CRC_31_PHILIPS">CRC_31_PHILIPS</a>

<a href="constant.CRC_32_AIXM.html" class="constant"
title="constant crc::CRC_32_AIXM">CRC_32_AIXM</a>

<a href="constant.CRC_32_AUTOSAR.html" class="constant"
title="constant crc::CRC_32_AUTOSAR">CRC_32_AUTOSAR</a>

<a href="constant.CRC_32_BASE91_D.html" class="constant"
title="constant crc::CRC_32_BASE91_D">CRC_32_BASE91_D</a>

<a href="constant.CRC_32_BZIP2.html" class="constant"
title="constant crc::CRC_32_BZIP2">CRC_32_BZIP2</a>

<a href="constant.CRC_32_CD_ROM_EDC.html" class="constant"
title="constant crc::CRC_32_CD_ROM_EDC">CRC_32_CD_ROM_EDC</a>

<a href="constant.CRC_32_CKSUM.html" class="constant"
title="constant crc::CRC_32_CKSUM">CRC_32_CKSUM</a>

<a href="constant.CRC_32_ISCSI.html" class="constant"
title="constant crc::CRC_32_ISCSI">CRC_32_ISCSI</a>

<a href="constant.CRC_32_ISO_HDLC.html" class="constant"
title="constant crc::CRC_32_ISO_HDLC">CRC_32_ISO_HDLC</a>

<a href="constant.CRC_32_JAMCRC.html" class="constant"
title="constant crc::CRC_32_JAMCRC">CRC_32_JAMCRC</a>

<a href="constant.CRC_32_MEF.html" class="constant"
title="constant crc::CRC_32_MEF">CRC_32_MEF</a>

<a href="constant.CRC_32_MPEG_2.html" class="constant"
title="constant crc::CRC_32_MPEG_2">CRC_32_MPEG_2</a>

<a href="constant.CRC_32_XFER.html" class="constant"
title="constant crc::CRC_32_XFER">CRC_32_XFER</a>

<a href="constant.CRC_40_GSM.html" class="constant"
title="constant crc::CRC_40_GSM">CRC_40_GSM</a>

<a href="constant.CRC_64_ECMA_182.html" class="constant"
title="constant crc::CRC_64_ECMA_182">CRC_64_ECMA_182</a>

<a href="constant.CRC_64_GO_ISO.html" class="constant"
title="constant crc::CRC_64_GO_ISO">CRC_64_GO_ISO</a>

<a href="constant.CRC_64_MS.html" class="constant"
title="constant crc::CRC_64_MS">CRC_64_MS</a>

<a href="constant.CRC_64_REDIS.html" class="constant"
title="constant crc::CRC_64_REDIS">CRC_64_REDIS</a>

<a href="constant.CRC_64_WE.html" class="constant"
title="constant crc::CRC_64_WE">CRC_64_WE</a>

<a href="constant.CRC_64_XZ.html" class="constant"
title="constant crc::CRC_64_XZ">CRC_64_XZ</a>

<a href="constant.CRC_82_DARC.html" class="constant"
title="constant crc::CRC_82_DARC">CRC_82_DARC</a>

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.Implementation.html" class="trait"
title="trait crc::Implementation">Implementation</a>

A trait for CRC implementations.

<a href="trait.Width.html" class="trait"
title="trait crc::Width">Width</a>

# Type Aliases<a href="#types" class="anchor">§</a>

<a href="type.NoTable.html" class="type"
title="type crc::NoTable">NoTable</a>  
An implementation of the CRC algorithm with no lookup table.
