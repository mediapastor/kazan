// SPDX-License-Identifier: LGPL-2.1-or-later
// Copyright 2018 Jacob Lifshay
#[macro_use]
extern crate enum_map;
extern crate sys_info;
extern crate uuid;
#[cfg(unix)]
extern crate xcb;
mod api;
mod api_impl;
mod handle;
use std::ffi::CStr;
use std::os::raw::c_char;

mod constants {
    pub const KAZAN_DEVICE_NAME: &'static str = "Kazan Software Renderer";
    pub const MIN_MEMORY_MAP_ALIGNMENT: usize = 128; // must be at least 64 and a power of 2 according to Vulkan spec
    pub const QUEUE_FAMILY_COUNT: u32 = 1;
    pub const QUEUE_COUNTS: [u32; QUEUE_FAMILY_COUNT as usize] = [1];
}

#[no_mangle]
pub unsafe extern "system" fn vk_icdGetInstanceProcAddr(
    instance: api::VkInstance,
    name: *const c_char,
) -> api::PFN_vkVoidFunction {
    api_impl::vkGetInstanceProcAddr(instance, name)
}

// note that if the following fails, then you may be encountering bindgen issue #1402
// https://github.com/rust-lang-nursery/rust-bindgen/issues/1402
#[allow(dead_code)]
const ASSERT_TYPE_VK_ICD_GET_INSTANCE_PROC_ADDR: api::PFN_vkGetInstanceProcAddr =
    Some(vk_icdGetInstanceProcAddr);

const ICD_VERSION: u32 = 5;

#[no_mangle]
pub unsafe extern "system" fn vk_icdNegotiateLoaderICDInterfaceVersion(
    supported_version: *mut u32,
) -> api::VkResult {
    if *supported_version > ICD_VERSION {
        *supported_version = ICD_VERSION;
    }
    api::VK_SUCCESS
}

#[allow(dead_code)]
const ASSERT_TYPE_VK_ICD_NEGOTIATE_LOADER_ICD_INTERFACE_VERSION:
    api::PFN_vkNegotiateLoaderICDInterfaceVersion = Some(vk_icdNegotiateLoaderICDInterfaceVersion);

#[no_mangle]
pub unsafe extern "system" fn vk_icdGetPhysicalDeviceProcAddr(
    instance: api::VkInstance,
    name: *const c_char,
) -> api::PFN_vkVoidFunction {
    match CStr::from_ptr(name).to_str().ok()? {
        "vkCreateDevice"
        | "vkCreateDisplayModeKHR"
        | "vkEnumerateDeviceExtensionProperties"
        | "vkEnumerateDeviceLayerProperties"
        | "vkGetDisplayModeProperties2KHR"
        | "vkGetDisplayModePropertiesKHR"
        | "vkGetDisplayPlaneCapabilities2KHR"
        | "vkGetDisplayPlaneCapabilitiesKHR"
        | "vkGetDisplayPlaneSupportedDisplaysKHR"
        | "vkGetPhysicalDeviceDisplayPlaneProperties2KHR"
        | "vkGetPhysicalDeviceDisplayPlanePropertiesKHR"
        | "vkGetPhysicalDeviceDisplayProperties2KHR"
        | "vkGetPhysicalDeviceDisplayPropertiesKHR"
        | "vkGetPhysicalDeviceExternalBufferProperties"
        | "vkGetPhysicalDeviceExternalBufferPropertiesKHR"
        | "vkGetPhysicalDeviceExternalFenceProperties"
        | "vkGetPhysicalDeviceExternalFencePropertiesKHR"
        | "vkGetPhysicalDeviceExternalImageFormatPropertiesNV"
        | "vkGetPhysicalDeviceExternalSemaphoreProperties"
        | "vkGetPhysicalDeviceExternalSemaphorePropertiesKHR"
        | "vkGetPhysicalDeviceFeatures"
        | "vkGetPhysicalDeviceFeatures2"
        | "vkGetPhysicalDeviceFeatures2KHR"
        | "vkGetPhysicalDeviceFormatProperties"
        | "vkGetPhysicalDeviceFormatProperties2"
        | "vkGetPhysicalDeviceFormatProperties2KHR"
        | "vkGetPhysicalDeviceGeneratedCommandsPropertiesNVX"
        | "vkGetPhysicalDeviceImageFormatProperties"
        | "vkGetPhysicalDeviceImageFormatProperties2"
        | "vkGetPhysicalDeviceImageFormatProperties2KHR"
        | "vkGetPhysicalDeviceMemoryProperties"
        | "vkGetPhysicalDeviceMemoryProperties2"
        | "vkGetPhysicalDeviceMemoryProperties2KHR"
        | "vkGetPhysicalDeviceMultisamplePropertiesEXT"
        | "vkGetPhysicalDevicePresentRectanglesKHR"
        | "vkGetPhysicalDeviceProperties"
        | "vkGetPhysicalDeviceProperties2"
        | "vkGetPhysicalDeviceProperties2KHR"
        | "vkGetPhysicalDeviceQueueFamilyProperties"
        | "vkGetPhysicalDeviceQueueFamilyProperties2"
        | "vkGetPhysicalDeviceQueueFamilyProperties2KHR"
        | "vkGetPhysicalDeviceSparseImageFormatProperties"
        | "vkGetPhysicalDeviceSparseImageFormatProperties2"
        | "vkGetPhysicalDeviceSparseImageFormatProperties2KHR"
        | "vkGetPhysicalDeviceSurfaceCapabilities2EXT"
        | "vkGetPhysicalDeviceSurfaceCapabilities2KHR"
        | "vkGetPhysicalDeviceSurfaceCapabilitiesKHR"
        | "vkGetPhysicalDeviceSurfaceFormats2KHR"
        | "vkGetPhysicalDeviceSurfaceFormatsKHR"
        | "vkGetPhysicalDeviceSurfacePresentModesKHR"
        | "vkGetPhysicalDeviceSurfaceSupportKHR"
        | "vkGetPhysicalDeviceXcbPresentationSupportKHR"
        | "vkReleaseDisplayEXT" => vk_icdGetInstanceProcAddr(instance, name),
        _ => None,
    }
}

#[allow(dead_code)]
const ASSERT_TYPE_VK_ICD_GET_PHYSICAL_DEVICE_PROC_ADDR: api::PFN_GetPhysicalDeviceProcAddr =
    Some(vk_icdGetInstanceProcAddr);

#[cfg(test)]
mod tests {}
