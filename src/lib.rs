{%- if driver_type == "WDM" or driver_type == "KMDF" -%}
#![no_std]
{%- endif %}

use wdk_sys::{NTSTATUS, PCUNICODE_STRING, {% if driver_type == "WDM" or driver_type == "KMDF" %}DRIVER_OBJECT{% else %}PDRIVER_OBJECT{% endif %}};

{% if driver_type == "WDM" or driver_type == "KMDF" -%}
#[cfg(not(test))]
extern crate wdk_panic;
{%- endif %}

{% if allocator -%}
#[cfg(not(test))]
use wdk_alloc::WdkAllocator;

#[cfg(not(test))]
#[global_allocator]
static GLOBAL_ALLOCATOR: WdkAllocator = WdkAllocator;
{%- endif %}

#[export_name = "DriverEntry"] // WDF expects a symbol with the name DriverEntry
pub unsafe extern "system" fn driver_entry(
    driver: {% if driver_type == "WDM" or driver_type == "KMDF" %}&mut DRIVER_OBJECT{% else %}PDRIVER_OBJECT{% endif %},
    registry_path: PCUNICODE_STRING,
) -> NTSTATUS {
    0
}
