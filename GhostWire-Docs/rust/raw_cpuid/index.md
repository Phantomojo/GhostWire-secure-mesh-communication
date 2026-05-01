Crate raw\_cpuid Copy item path

<span class="sub-heading"><a href="../src/raw_cpuid/lib.rs.html#1-6043" class="src">Source</a>
</span>

Expand description

A library to parse the x86 CPUID instruction, written in rust with no
external dependencies. The implementation closely resembles the Intel
CPUID manual description. The library works with no\_std.

## <a href="#example" class="doc-anchor">§</a>Example

``` {.rust .rust-example-rendered}
use raw_cpuid::CpuId;
let cpuid = CpuId::new();

if let Some(vf) = cpuid.get_vendor_info() {
    assert!(vf.as_str() == "GenuineIntel" || vf.as_str() == "AuthenticAMD");
}

let has_sse = cpuid.get_feature_info().map_or(false, |finfo| finfo.has_sse());
if has_sse {
    println!("CPU supports SSE!");
}

if let Some(cparams) = cpuid.get_cache_parameters() {
    for cache in cparams {
        let size = cache.associativity() * cache.physical_line_partitions() * cache.coherency_line_size() * cache.sets();
        println!("L{}-Cache size is {}", cache.level(), size);
    }
} else {
    println!("No cache parameter information available")
}
```

# <a href="#platform-support" class="doc-anchor">§</a>Platform support

CPU vendors may choose to not support certain functions/leafs in cpuid
or only support them partially. We highlight this with the following
emojis throughout the documentation:

-   ✅: This struct/function is fully supported by the vendor.
-   🟡: This struct is partially supported by the vendor, refer to
    individual functions for more information.
-   ❌: This struct/function is not supported by the vendor. When
    queried on this platform, we will return None/false/0 (or some other
    sane default).
-   ❓: This struct/function is not supported by the vendor according to
    the manual, but the in practice it still may return valid
    information.

Note that the presence of a ✅ does not guarantee that a specific
feature will exist for your CPU – just that it is potentially supported
by the vendor on some of its chips. You will still have to query it at
runtime.

# Re-exports<a href="#reexports" class="anchor">§</a>

`pub use native_cpuid::`<a href="native_cpuid/struct.CpuIdReaderNative.html" class="struct"
title="struct raw_cpuid::native_cpuid::CpuIdReaderNative"><code>CpuIdReaderNative</code></a>`;`

# Modules<a href="#modules" class="anchor">§</a>

<a href="native_cpuid/index.html" class="mod"
title="mod raw_cpuid::native_cpuid">native_cpuid</a>  
Uses Rust’s `cpuid` function from the `arch` module.

# Macros<a href="#macros" class="anchor">§</a>

<a href="macro.cpuid.html" class="macro"
title="macro raw_cpuid::cpuid">cpuid</a>  
Macro which queries cpuid directly.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.ApmInfo.html" class="struct"
title="struct raw_cpuid::ApmInfo">ApmInfo</a>  
Processor Power Management and RAS Capabilities (LEAF=0x8000\_0007).

<a href="struct.AssignableBandwidthMonitoringCounterInfo.html"
class="struct"
title="struct raw_cpuid::AssignableBandwidthMonitoringCounterInfo">AssignableBandwidthMonitoringCounterInfo</a>  
L3 Memory Bandwidth Enforcement Information (LEAF=0x8000\_0020\_x5).

<a href="struct.BandwidthMonitoringEventCounters.html" class="struct"
title="struct raw_cpuid::BandwidthMonitoringEventCounters">BandwidthMonitoringEventCounters</a>  
Bandwidth Monitoring Event Counters Information (LEAF=0x8000\_0020\_x3).

<a href="struct.CacheInfo.html" class="struct"
title="struct raw_cpuid::CacheInfo">CacheInfo</a>  
Describes any kind of cache (TLB, Data and Instruction caches plus
prefetchers).

<a href="struct.CacheInfoIter.html" class="struct"
title="struct raw_cpuid::CacheInfoIter">CacheInfoIter</a>  
Iterates over cache information (LEAF=0x02).

<a href="struct.CacheParameter.html" class="struct"
title="struct raw_cpuid::CacheParameter">CacheParameter</a>  
Information about an individual cache in the hierarchy.

<a href="struct.CacheParametersIter.html" class="struct"
title="struct raw_cpuid::CacheParametersIter">CacheParametersIter</a>  
Iterator over caches (LEAF=0x04).

<a href="struct.CpuId.html" class="struct"
title="struct raw_cpuid::CpuId">CpuId</a>  
The main type used to query information about the CPU we’re running on.

<a href="struct.CpuIdResult.html" class="struct"
title="struct raw_cpuid::CpuIdResult">CpuIdResult</a>  
Low-level data-structure to store result of cpuid instruction.

<a href="struct.DatInfo.html" class="struct"
title="struct raw_cpuid::DatInfo">DatInfo</a>  
Deterministic Address Translation Structure

<a href="struct.DatIter.html" class="struct"
title="struct raw_cpuid::DatIter">DatIter</a>  
Deterministic Address Translation Structure Iterator (LEAF=0x18).

<a href="struct.DirectCacheAccessInfo.html" class="struct"
title="struct raw_cpuid::DirectCacheAccessInfo">DirectCacheAccessInfo</a>  
Direct cache access info (LEAF=0x09).

<a href="struct.EpcSection.html" class="struct"
title="struct raw_cpuid::EpcSection">EpcSection</a>  
EBX:EAX and EDX:ECX provide information on the Enclave Page Cache (EPC)
section

<a href="struct.ExtendedCpuTopologyIter.html" class="struct"
title="struct raw_cpuid::ExtendedCpuTopologyIter">ExtendedCpuTopologyIter</a>  
Extended CPU Topology (LEAF=0x8000\_0026).

<a href="struct.ExtendedCpuTopologyLevel.html" class="struct"
title="struct raw_cpuid::ExtendedCpuTopologyLevel">ExtendedCpuTopologyLevel</a>  
Gives information about the current level in the cpu topology.

<a href="struct.ExtendedFeatureIdentification2.html" class="struct"
title="struct raw_cpuid::ExtendedFeatureIdentification2">ExtendedFeatureIdentification2</a>  
Extended Feature Identification 2 (LEAF=0x8000\_0021).

<a href="struct.ExtendedFeatures.html" class="struct"
title="struct raw_cpuid::ExtendedFeatures">ExtendedFeatures</a>  
Structured Extended Feature Identifiers (LEAF=0x07).

<a href="struct.ExtendedPerformanceMonitoringDebug.html" class="struct"
title="struct raw_cpuid::ExtendedPerformanceMonitoringDebug">ExtendedPerformanceMonitoringDebug</a>  
Extended Performance Monitoring and Debug (LEAF=0x8000\_0022).

<a href="struct.ExtendedProcessorFeatureIdentifiers.html" class="struct"
title="struct raw_cpuid::ExtendedProcessorFeatureIdentifiers">ExtendedProcessorFeatureIdentifiers</a>  
Extended Processor and Processor Feature Identifiers (LEAF=0x8000\_0001)

<a href="struct.ExtendedState.html" class="struct"
title="struct raw_cpuid::ExtendedState">ExtendedState</a>  
ExtendedState subleaf structure for things that need to be restored.

<a href="struct.ExtendedStateInfo.html" class="struct"
title="struct raw_cpuid::ExtendedStateInfo">ExtendedStateInfo</a>  
Information for saving/restoring extended register state (LEAF=0x0D).

<a href="struct.ExtendedStateIter.html" class="struct"
title="struct raw_cpuid::ExtendedStateIter">ExtendedStateIter</a>  
Yields
[ExtendedState](struct.ExtendedState.html "struct raw_cpuid::ExtendedState")
structs.

<a href="struct.ExtendedTopologyIter.html" class="struct"
title="struct raw_cpuid::ExtendedTopologyIter">ExtendedTopologyIter</a>  
Information about topology (LEAF=0x0B).

<a href="struct.ExtendedTopologyLevel.html" class="struct"
title="struct raw_cpuid::ExtendedTopologyLevel">ExtendedTopologyLevel</a>  
Gives information about the current level in the topology.

<a href="struct.FeatureInfo.html" class="struct"
title="struct raw_cpuid::FeatureInfo">FeatureInfo</a>  
Processor and Processor Feature Identifiers (LEAF=0x01).

<a href="struct.HypervisorInfo.html" class="struct"
title="struct raw_cpuid::HypervisorInfo">HypervisorInfo</a>  
Information about Hypervisor (LEAF=0x4000\_0001)

<a href="struct.InstructionBasedSamplingCapabilities.html"
class="struct"
title="struct raw_cpuid::InstructionBasedSamplingCapabilities">InstructionBasedSamplingCapabilities</a>  
Performance Optimization Identifier (LEAF=0x8000\_001A).

<a href="struct.L1CacheTlbInfo.html" class="struct"
title="struct raw_cpuid::L1CacheTlbInfo">L1CacheTlbInfo</a>  
L1 Cache and TLB Information (LEAF=0x8000\_0005).

<a href="struct.L2And3CacheTlbInfo.html" class="struct"
title="struct raw_cpuid::L2And3CacheTlbInfo">L2And3CacheTlbInfo</a>  
L2/L3 Cache and TLB Information (LEAF=0x8000\_0006).

<a href="struct.L2CatInfo.html" class="struct"
title="struct raw_cpuid::L2CatInfo">L2CatInfo</a>  
L2 Cache Allocation Technology Enumeration Sub-leaf (LEAF=0x10,
SUBLEAF=2).

<a href="struct.L3CatInfo.html" class="struct"
title="struct raw_cpuid::L3CatInfo">L3CatInfo</a>  
L3 Cache Allocation Technology Enumeration Sub-leaf (LEAF=0x10,
SUBLEAF=1).

<a href="struct.L3MemoryBandwidthEnforcementInformation.html"
class="struct"
title="struct raw_cpuid::L3MemoryBandwidthEnforcementInformation">L3MemoryBandwidthEnforcementInformation</a>  
L3 Memory Bandwidth Enforcement Information (LEAF=0x8000\_0020\_x1 and
x2).

<a href="struct.L3MonitoringInfo.html" class="struct"
title="struct raw_cpuid::L3MonitoringInfo">L3MonitoringInfo</a>  
Information about L3 cache monitoring.

<a href="struct.MemBwAllocationInfo.html" class="struct"
title="struct raw_cpuid::MemBwAllocationInfo">MemBwAllocationInfo</a>  
Memory Bandwidth Allocation Enumeration Sub-leaf (LEAF=0x10, SUBLEAF=3).

<a href="struct.MemoryEncryptionInfo.html" class="struct"
title="struct raw_cpuid::MemoryEncryptionInfo">MemoryEncryptionInfo</a>  
Encrypted Memory Capabilities (LEAF=0x8000\_001F).

<a href="struct.MonitorMwaitInfo.html" class="struct"
title="struct raw_cpuid::MonitorMwaitInfo">MonitorMwaitInfo</a>  
Information about how monitor/mwait works on this CPU (LEAF=0x05).

<a href="struct.MultiKeyEncryptedMemoryCapabilities.html" class="struct"
title="struct raw_cpuid::MultiKeyEncryptedMemoryCapabilities">MultiKeyEncryptedMemoryCapabilities</a>  
Multi-Key Encrypted Memory Capabilities (LEAF=0x8000\_0023).

<a href="struct.PerformanceMonitoringInfo.html" class="struct"
title="struct raw_cpuid::PerformanceMonitoringInfo">PerformanceMonitoringInfo</a>  
Info about performance monitoring – how many counters etc. (LEAF=0x0A)

<a href="struct.PerformanceOptimizationInfo.html" class="struct"
title="struct raw_cpuid::PerformanceOptimizationInfo">PerformanceOptimizationInfo</a>  
Performance Optimization Identifier (LEAF=0x8000\_001A).

<a href="struct.PqosExtendedFeatureInfo.html" class="struct"
title="struct raw_cpuid::PqosExtendedFeatureInfo">PqosExtendedFeatureInfo</a>  
Platform Quality of Service Information (LEAF=0x8000\_0020).

<a href="struct.ProcessorBrandString.html" class="struct"
title="struct raw_cpuid::ProcessorBrandString">ProcessorBrandString</a>  
Processor name (LEAF=0x8000\_0002..=0x8000\_0004).

<a href="struct.ProcessorCapacityAndFeatureInfo.html" class="struct"
title="struct raw_cpuid::ProcessorCapacityAndFeatureInfo">ProcessorCapacityAndFeatureInfo</a>  
Processor Capacity Parameters and Extended Feature Identification
(LEAF=0x8000\_0008).

<a href="struct.ProcessorFrequencyInfo.html" class="struct"
title="struct raw_cpuid::ProcessorFrequencyInfo">ProcessorFrequencyInfo</a>  
Processor Frequency Information (LEAF=0x16).

<a href="struct.ProcessorSerial.html" class="struct"
title="struct raw_cpuid::ProcessorSerial">ProcessorSerial</a>  
Processor Serial Number (LEAF=0x3).

<a href="struct.ProcessorTopologyInfo.html" class="struct"
title="struct raw_cpuid::ProcessorTopologyInfo">ProcessorTopologyInfo</a>  
Processor Topology Information (LEAF=0x8000\_001E).

<a href="struct.ProcessorTraceInfo.html" class="struct"
title="struct raw_cpuid::ProcessorTraceInfo">ProcessorTraceInfo</a>  
Intel Processor Trace Information (LEAF=0x14).

<a href="struct.RdtAllocationInfo.html" class="struct"
title="struct raw_cpuid::RdtAllocationInfo">RdtAllocationInfo</a>  
Quality of service enforcement information (LEAF=0x10).

<a href="struct.RdtMonitoringInfo.html" class="struct"
title="struct raw_cpuid::RdtMonitoringInfo">RdtMonitoringInfo</a>  
Intel Resource Director Technology RDT (LEAF=0x0F).

<a href="struct.SgxInfo.html" class="struct"
title="struct raw_cpuid::SgxInfo">SgxInfo</a>  
Intel SGX Capability Enumeration Leaf (LEAF=0x12).

<a href="struct.SgxSectionIter.html" class="struct"
title="struct raw_cpuid::SgxSectionIter">SgxSectionIter</a>  
Iterator over the SGX sub-leafs (ECX &gt;= 2).

<a href="struct.SoCVendorAttributesIter.html" class="struct"
title="struct raw_cpuid::SoCVendorAttributesIter">SoCVendorAttributesIter</a>  
Iterator for SoC vendor attributes.

<a href="struct.SoCVendorBrand.html" class="struct"
title="struct raw_cpuid::SoCVendorBrand">SoCVendorBrand</a>  
A vendor brand string as queried from the cpuid leaf.

<a href="struct.SoCVendorInfo.html" class="struct"
title="struct raw_cpuid::SoCVendorInfo">SoCVendorInfo</a>  
SoC vendor specific information (LEAF=0x17).

<a href="struct.SvmFeatures.html" class="struct"
title="struct raw_cpuid::SvmFeatures">SvmFeatures</a>  
Information about the SVM features that the processory supports
(LEAF=0x8000\_000A).

<a href="struct.ThermalPowerInfo.html" class="struct"
title="struct raw_cpuid::ThermalPowerInfo">ThermalPowerInfo</a>  
Query information about thermal and power management features of the CPU
(LEAF=0x06).

<a href="struct.Tlb1gbPageInfo.html" class="struct"
title="struct raw_cpuid::Tlb1gbPageInfo">Tlb1gbPageInfo</a>  
TLB 1-GiB Pages Information (LEAF=0x8000\_0019).

<a href="struct.TscInfo.html" class="struct"
title="struct raw_cpuid::TscInfo">TscInfo</a>  
Time Stamp Counter/Core Crystal Clock Information (LEAF=0x15).

<a href="struct.VendorInfo.html" class="struct"
title="struct raw_cpuid::VendorInfo">VendorInfo</a>  
Vendor Info String (LEAF=0x0)

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.Associativity.html" class="enum"
title="enum raw_cpuid::Associativity">Associativity</a>  
Info about cache Associativity.

<a href="enum.CacheInfoType.html" class="enum"
title="enum raw_cpuid::CacheInfoType">CacheInfoType</a>  
What type of cache are we dealing with?

<a href="enum.CacheType.html" class="enum"
title="enum raw_cpuid::CacheType">CacheType</a>  
Info about a what a given cache caches (instructions, data, etc.)

<a href="enum.DatType.html" class="enum"
title="enum raw_cpuid::DatType">DatType</a>  
Deterministic Address Translation cache type (EDX bits 04 – 00)

<a href="enum.ExtendedRegisterStateLocation.html" class="enum"
title="enum raw_cpuid::ExtendedRegisterStateLocation">ExtendedRegisterStateLocation</a>  
Where the extended register state is stored.

<a href="enum.ExtendedRegisterType.html" class="enum"
title="enum raw_cpuid::ExtendedRegisterType">ExtendedRegisterType</a>  
What kidn of extended register state this is.

<a href="enum.HierarchyLevelType.html" class="enum"
title="enum raw_cpuid::HierarchyLevelType">HierarchyLevelType</a>  
<a href="enum.Hypervisor.html" class="enum"
title="enum raw_cpuid::Hypervisor">Hypervisor</a>  
Identifies the different Hypervisor products.

<a href="enum.SgxSectionInfo.html" class="enum"
title="enum raw_cpuid::SgxSectionInfo">SgxSectionInfo</a>  
Intel SGX EPC Enumeration Leaf

<a href="enum.TopologyType.html" class="enum"
title="enum raw_cpuid::TopologyType">TopologyType</a>  
What type of core we have at this level in the topology (real CPU or
hyper-threaded).

# Constants<a href="#constants" class="anchor">§</a>

<a href="constant.CACHE_INFO_TABLE.html" class="constant"
title="constant raw_cpuid::CACHE_INFO_TABLE">CACHE_INFO_TABLE</a>  
This table is taken from Intel manual (Section CPUID instruction).

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.CpuIdReader.html" class="trait"
title="trait raw_cpuid::CpuIdReader">CpuIdReader</a>  
Implements function to read/write cpuid. This allows to conveniently
swap out the underlying cpuid implementation with one that returns data
that is deterministic (for unit-testing).
